use anyhow::Result;
use std::process::Stdio;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompileRequest {
    pub code: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CompileResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

pub async fn compile_and_run(code: &str) -> Result<CompileResult> {
    let start = std::time::Instant::now();
    let id = uuid::Uuid::new_v4().to_string();
    let tmp_dir = std::env::temp_dir().join(format!("rusticize-{}", id));
    fs::create_dir_all(&tmp_dir).await?;

    let src_path = tmp_dir.join("main.rs");
    let bin_path = tmp_dir.join("main");

    // Write source
    let mut file = fs::File::create(&src_path).await?;
    file.write_all(code.as_bytes()).await?;
    file.flush().await?;
    drop(file);

    // Compile
    let compile_output = timeout(
        Duration::from_secs(15),
        Command::new("rustc")
            .args([
                "--edition", "2021",
                "-o", bin_path.to_str().unwrap(),
                src_path.to_str().unwrap(),
            ])
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .output(),
    )
    .await;

    let mut result = CompileResult {
        success: false,
        stdout: String::new(),
        stderr: String::new(),
        duration_ms: start.elapsed().as_millis() as u64,
    };

    match compile_output {
        Ok(Ok(output)) => {
            if !output.status.success() {
                result.stderr = String::from_utf8_lossy(&output.stderr).to_string();
                cleanup(&tmp_dir).await.ok();
                return Ok(result);
            }
        }
        Ok(Err(e)) => {
            result.stderr = format!("Failed to run compiler: {}", e);
            cleanup(&tmp_dir).await.ok();
            return Ok(result);
        }
        Err(_) => {
            result.stderr = "Compilation timed out (15s)".into();
            cleanup(&tmp_dir).await.ok();
            return Ok(result);
        }
    }

    // Run compiled binary
    let run_output = timeout(
        Duration::from_secs(5),
        Command::new(bin_path.to_str().unwrap())
            .current_dir(&tmp_dir)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .output(),
    )
    .await;

    match run_output {
        Ok(Ok(output)) => {
            result.success = output.status.success();
            result.stdout = String::from_utf8_lossy(&output.stdout).to_string();
            result.stderr = String::from_utf8_lossy(&output.stderr).to_string();
        }
        Ok(Err(e)) => {
            result.stderr = format!("Runtime error: {}", e);
        }
        Err(_) => {
            result.stderr = "Execution timed out (5s)".into();
        }
    }

    result.duration_ms = start.elapsed().as_millis() as u64;
    cleanup(&tmp_dir).await.ok();
    Ok(result)
}

async fn cleanup(dir: &std::path::Path) -> Result<()> {
    fs::remove_dir_all(dir).await?;
    Ok(())
}
