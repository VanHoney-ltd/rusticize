use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub description: String,
    pub level: String,
    pub starter_code: String,
    pub validator: Validator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Validator {
    OutputContains { expected: String },
    OutputEquals { expected: String },
    Compiles,
    RegexMatch { pattern: String },
}

pub fn built_in_lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "hello-world".into(),
            title: "Hello, World!".into(),
            description: "Every Rust journey begins with a single print. Write a program that outputs `Hello, world!`.".into(),
            level: "beginner".into(),
            starter_code: "fn main() {\n    // Your code here\n}".into(),
            validator: Validator::OutputContains { expected: "Hello, world!".into() },
        },
        Lesson {
            id: "variables".into(),
            title: "Variables & Mutability".into(),
            description: "Variables are immutable by default in Rust. Declare a mutable variable `x`, set it to 5, then change it to 10 and print it.".into(),
            level: "beginner".into(),
            starter_code: "fn main() {\n    let x = 5;\n    // Make x mutable and change it to 10\n    println!(\"{}\", x);\n}".into(),
            validator: Validator::OutputContains { expected: "10".into() },
        },
        Lesson {
            id: "functions".into(),
            title: "Functions".into(),
            description: "Write a function `add` that takes two i32 parameters and returns their sum. Call it with 3 and 5 and print the result.".into(),
            level: "beginner".into(),
            starter_code: "fn main() {\n    // Call your add function here\n}\n\n// Define add below".into(),
            validator: Validator::OutputContains { expected: "8".into() },
        },
        Lesson {
            id: "control-flow".into(),
            title: "Control Flow".into(),
            description: "Use an `if` expression to print `even` if a number is even, or `odd` if it's odd. Test with the number 7.".into(),
            level: "beginner".into(),
            starter_code: "fn main() {\n    let number = 7;\n    // Your code here\n}".into(),
            validator: Validator::OutputContains { expected: "odd".into() },
        },
        Lesson {
            id: "ownership".into(),
            title: "Ownership".into(),
            description: "Understanding ownership is key to Rust. Create a String, pass it to a function that prints it, then try to use it again. Fix the compiler error.".into(),
            level: "intermediate".into(),
            starter_code: "fn main() {\n    let s = String::from(\"hello\");\n    print_string(s);\n    println!(\"{}\", s); // This will error — fix it!\n}\n\nfn print_string(s: String) {\n    println!(\"{}\", s);\n}".into(),
            validator: Validator::OutputContains { expected: "hello".into() },
        },
        Lesson {
            id: "structs".into(),
            title: "Structs".into(),
            description: "Define a `Rectangle` struct with `width` and `height` fields. Implement an `area` method that returns the area. Print the area of a 10x20 rectangle.".into(),
            level: "intermediate".into(),
            starter_code: "fn main() {\n    // Create a Rectangle and print its area\n}\n\n// Define Rectangle and its area method here".into(),
            validator: Validator::OutputContains { expected: "200".into() },
        },
        Lesson {
            id: "enums".into(),
            title: "Enums & Pattern Matching".into(),
            description: "Define an enum `Message` with variants `Quit`, `Move { x: i32, y: i32 }`, and `Write(String)`. Use `match` to handle each variant.".into(),
            level: "intermediate".into(),
            starter_code: "fn main() {\n    let msg = Message::Move { x: 10, y: 20 };\n    // Match on msg and print the coordinates\n}\n\n// Define Message enum here".into(),
            validator: Validator::OutputContains { expected: "10".into() },
        },
        Lesson {
            id: "error-handling".into(),
            title: "Error Handling".into(),
            description: "Write a function `parse_number` that parses a string to i32 and returns a Result. Use `?` in main to propagate errors. Test with `\"42\"`.".into(),
            level: "intermediate".into(),
            starter_code: "fn main() -> Result<(), Box<dyn std::error::Error>> {\n    // Call parse_number and print the result\n    Ok(())\n}\n\nfn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {\n    // Your code here\n}".into(),
            validator: Validator::OutputContains { expected: "42".into() },
        },
        Lesson {
            id: "collections".into(),
            title: "Collections".into(),
            description: "Create a `HashMap` that maps names to scores. Insert `\"Alice\" -> 100` and `\"Bob\" -> 85`. Print Alice's score.".into(),
            level: "intermediate".into(),
            starter_code: "use std::collections::HashMap;\n\nfn main() {\n    // Create and populate the HashMap\n    // Print Alice's score\n}".into(),
            validator: Validator::OutputContains { expected: "100".into() },
        },
        Lesson {
            id: "traits".into(),
            title: "Traits".into(),
            description: "Define a trait `Summarize` with a method `summarize`. Implement it for a `NewsArticle` struct. Print the summary.".into(),
            level: "advanced".into(),
            starter_code: "fn main() {\n    let article = NewsArticle {\n        headline: String::from(\"Rust 1.80 Released\"),\n        location: String::from(\"Internet\"),\n    };\n    println!(\"{}\", article.summarize());\n}\n\n// Define Summarize trait and implement for NewsArticle".into(),
            validator: Validator::OutputContains { expected: "Rust 1.80".into() },
        },
    ]
}

pub fn validate(result: &crate::compiler::CompileResult, validator: &Validator) -> bool {
    if !result.success {
        return false;
    }
    match validator {
        Validator::OutputContains { expected } => result.stdout.contains(expected),
        Validator::OutputEquals { expected } => result.stdout.trim() == expected.trim(),
        Validator::Compiles => result.success,
        Validator::RegexMatch { pattern } => {
            regex::Regex::new(pattern).map(|re| re.is_match(&result.stdout)).unwrap_or(false)
        }
    }
}
