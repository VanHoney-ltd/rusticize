use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::compiler::{compile_and_run, CompileRequest, CompileResult};
use crate::db::Database;
use crate::lessons::{validate, Lesson};

pub struct AppState {
    pub db: Mutex<Database>,
    pub lessons: Vec<Lesson>,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/compile", post(handle_compile))
        .route("/api/lessons", get(list_lessons))
        .route("/api/lessons/{id}", get(get_lesson))
        .route("/api/lessons/{id}/validate", post(validate_lesson))
        .route("/api/progress", get(get_progress))
        .route("/api/progress/{id}", post(save_progress))
        .with_state(state)
}

async fn handle_compile(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<CompileRequest>,
) -> Result<Json<CompileResult>, StatusCode> {
    match compile_and_run(&req.code).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_lessons(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let lessons: Vec<_> = state.lessons.iter().map(|l| {
        serde_json::json!({
            "id": l.id,
            "title": l.title,
            "level": l.level,
        })
    }).collect();
    Json(lessons)
}

async fn get_lesson(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let lesson = state.lessons.iter().find(|l| l.id == id);
    match lesson {
        Some(l) => Ok(Json(l.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(serde::Deserialize)]
pub struct ValidateRequest {
    pub code: String,
}

#[derive(serde::Serialize)]
pub struct ValidateResponse {
    pub success: bool,
    pub result: CompileResult,
    pub passed: bool,
}

async fn validate_lesson(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<ValidateRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let lesson = state.lessons.iter().find(|l| l.id == id);
    let lesson = match lesson {
        Some(l) => l,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let result = match compile_and_run(&req.code).await {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let passed = validate(&result, &lesson.validator);

    let db = state.db.lock().await;
    let _ = db.save_progress(&id, passed, &req.code);

    Ok(Json(ValidateResponse {
        success: result.success,
        result,
        passed,
    }))
}

async fn get_progress(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    match db.get_all_progress() {
        Ok(progress) => Json(progress).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct SaveProgressRequest {
    pub code: String,
    pub completed: bool,
}

async fn save_progress(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<SaveProgressRequest>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    match db.save_progress(&id, req.completed, &req.code) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
