use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub description: String,
    pub level: String,
    pub starter_code: String,
    pub validator: Validator,
    /// What you'll learn from this lesson
    pub objectives: Vec<String>,
    /// Detailed concept explanation (HTML content)
    pub explanation: String,
    /// Progressive hints — each reveals more
    pub hints: Vec<String>,
    /// Common beginner mistakes and why they happen
    pub common_mistakes: Vec<CommonMistake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonMistake {
    pub mistake: String,
    pub why_it_happens: String,
    pub how_to_fix: String,
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
            description: "Every Rust program needs an entry point. Write your first program that greets the world.".into(),
            level: "beginner".into(),
            starter_code: "fn main() {\n    // Your code here\n}".into(),
            validator: Validator::OutputContains { expected: "Hello, world!".into() },
            objectives: vec![
                "Understand that `fn main()` is the program entry point".into(),
                "Learn that `println!` is a macro (note the `!`)".into(),
                "See how strings work in Rust with double quotes".into(),
            ],
            explanation: r#"
<h3>The Main Function</h3>
<p>Every Rust executable needs a function named <code>main</code>. This is where execution begins. There is no top-level code outside of functions.</p>
<h3>Macros vs Functions</h3>
<p><code>println!</code> ends with an exclamation mark because it's a <strong>macro</strong>, not a regular function. Macros generate code at compile time. For now, just remember: if it has a <code>!</code>, it's a macro.</p>
<h3>Strings</h3>
<p>Rust strings use double quotes (<code>"Hello"</code>). Single quotes are for single characters (<code>'H'</code>) — a completely different type called <code>char</code>.</p>
"#.into(),
            hints: vec![
                "Inside `main()`, use `println!(\"Hello, world!\");`".into(),
                "Make sure the string matches exactly — capitalization and punctuation matter".into(),
                "Don't forget the semicolon at the end of the line".into(),
            ],
            common_mistakes: vec![
                CommonMistake {
                    mistake: "Using `print!` instead of `println!`".into(),
                    why_it_happens: "`print!` doesn't add a newline, so the validator might not see the output cleanly.".into(),
                    how_to_fix: "Use `println!` — it prints a line and adds a newline.".into(),
                },
                CommonMistake {
                    mistake: "Forgetting the `!` in `println`".into(),
                    why_it_happens: "`println` without `!` doesn't exist as a function — it's a macro.".into(),
                    how_to_fix: "Always write `println!` with the exclamation mark.".into(),
                },
            ],
        },
        Lesson {
            id: "variables".into(),
            title: "Variables & Mutability".into(),
            description: "Variables are immutable by default. Learn why, and how to make them mutable when you need to.".into(),
            level: "beginner".into(),
            starter_code: "fn main() {\n    let x = 5;\n    // Make x mutable and change it to 10\n    println!(\"{}\", x);\n}".into(),
            validator: Validator::OutputContains { expected: "10".into() },
            objectives: vec![
                "Understand immutability by default".into(),
                "Learn the `mut` keyword".into(),
                "See the compiler error when you try to mutate an immutable variable".into(),
            ],
            explanation: r#"
<h3>Immutability by Default</h3>
<p>In Rust, variables are <strong>immutable</strong> (unchangeable) by default. Once you bind a value to a name, you can't change that value. This might seem restrictive, but it prevents an entire class of bugs where something changes unexpectedly.</p>
<h3>Making Variables Mutable</h3>
<p>To allow a variable to change, add <code>mut</code> before the variable name:</p>
<pre><code>let mut x = 5;
x = 10; // This works!</code></pre>
<h3>Shadowing</h3>
<p>You can also declare a new variable with the same name, <strong>shadowing</strong> the old one:</p>
<pre><code>let x = 5;
let x = x + 1; // New x, old x is gone</code></pre>
<p>Shadowing is different from `mut` — you're creating a completely new variable that happens to have the same name.</p>
"#.into(),
            hints: vec![
                "Change `let x = 5;` to `let mut x = 5;`".into(),
                "After declaring `x` as mutable, assign `x = 10;` before the println".into(),
                "The order matters: declare, mutate, then print".into(),
            ],
            common_mistakes: vec![
                CommonMistake {
                    mistake: "Writing `x = 10;` without `mut`".into(),
                    why_it_happens: "Rust enforces immutability at compile time. Without `mut`, reassignment is forbidden.".into(),
                    how_to_fix: "Change the declaration to `let mut x = 5;`.".into(),
                },
                CommonMistake {
                    mistake: "Shadowing instead of mutating: `let x = 10;`".into(),
                    why_it_happens: "This creates a new variable. It works here but doesn't teach mutability.".into(),
                    how_to_fix: "Use `mut` and reassign: `x = 10;`".into(),
                },
            ],
        },
        Lesson {
            id: "functions".into(),
            title: "Functions".into(),
            description: "Learn how to define functions, specify return types, and understand expressions vs statements.".into(),
            level: "beginner".into(),
            starter_code: "fn main() {\n    // Call your add function here\n}\n\n// Define add below".into(),
            validator: Validator::OutputContains { expected: "8".into() },
            objectives: vec![
                "Define a function with parameters".into(),
                "Specify a return type with `->`".into(),
                "Understand the difference between statements and expressions".into(),
            ],
            explanation: r#"
<h3>Function Syntax</h3>
<pre><code>fn name(parameter: Type) -> ReturnType {
    // body
}</code></pre>
<h3>Parameters</h3>
<p>Every parameter must have a type annotation. Rust requires explicit types for function signatures — this is how the compiler verifies your code.</p>
<h3>Return Values</h3>
<p>Use <code>-></code> to specify the return type. The last expression in a function (without a semicolon) is automatically returned:</p>
<pre><code>fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = expression = return value
}</code></pre>
<p>If you add a semicolon, it becomes a <strong>statement</strong> and returns <code>()</code> (unit type), causing a compile error.</p>
<h3>Statements vs Expressions</h3>
<ul>
<li><strong>Statement</strong>: Does something, doesn't return a value. Ends with <code>;</code></li>
<li><strong>Expression</strong>: Returns a value. No <code>;</code> at the end.</li>
</ul>
"#.into(),
            hints: vec![
                "Define `fn add(a: i32, b: i32) -> i32`".into(),
                "The body should be `a + b` with NO semicolon — it's an expression".into(),
                "In `main()`, call `let sum = add(3, 5);` and print it".into(),
            ],
            common_mistakes: vec![
                CommonMistake {
                    mistake: "Adding a semicolon to the return expression: `a + b;`".into(),
                    why_it_happens: "The semicolon turns the expression into a statement. Statements return `()`, not `i32`.".into(),
                    how_to_fix: "Remove the semicolon: `a + b`".into(),
                },
                CommonMistake {
                    mistake: "Using `return a + b;` unnecessarily".into(),
                    why_it_happens: "`return` works but is idiomatically used only for early returns. Rust prefers implicit returns.".into(),
                    how_to_fix: "Omit `return` and the semicolon for the final expression.".into(),
                },
            ],
        },
        Lesson {
            id: "control-flow".into(),
            title: "Control Flow".into(),
            description: "Use `if` expressions to make decisions. In Rust, `if` is an expression, not just a statement.".into(),
            level: "beginner".into(),
            starter_code: "fn main() {\n    let number = 7;\n    // Your code here\n}".into(),
            validator: Validator::OutputContains { expected: "odd".into() },
            objectives: vec![
                "Use `if` / `else if` / `else` chains".into(),
                "Understand that `if` is an expression in Rust".into(),
                "Learn that conditions don't need parentheses".into(),
            ],
            explanation: r#"
<h3>`if` as an Expression</h3>
<p>In Rust, <code>if</code> is an <strong>expression</strong> — it returns a value. You can assign the result of an <code>if</code> to a variable:</p>
<pre><code>let result = if condition { "yes" } else { "no" };</code></pre>
<h3>No Parentheses Needed</h3>
<p>Unlike C or JavaScript, Rust <strong>does not</strong> require parentheses around conditions:</p>
<pre><code>if number % 2 == 0 {  // Correct
if (number % 2 == 0) { // Also works, but not idiomatic</code></pre>
<h3>Exhaustiveness</h3>
<p>The compiler won't let you use an <code>if</code> expression without an <code>else</code> if you're assigning the result. Both branches must return the same type.</p>
<h3>The Modulo Operator</h3>
<p><code>%</code> gives the remainder of division. <code>7 % 2</code> equals <code>1</code>, so 7 is odd.</p>
"#.into(),
            hints: vec![
                "Check if `number % 2 == 0` for even".into(),
                "Use `if` and `else` — no parentheses around the condition".into(),
                "Print `even` or `odd` based on the result".into(),
            ],
            common_mistakes: vec![
                CommonMistake {
                    mistake: "Using `=` instead of `==` in the condition".into(),
                    why_it_happens: "`=` is assignment, `==` is comparison. Rust catches this at compile time if types mismatch.".into(),
                    how_to_fix: "Use `==` for comparisons.".into(),
                },
                CommonMistake {
                    mistake: "Forgetting `else` when assigning an `if` expression".into(),
                    why_it_happens: "The compiler needs both branches to determine the type.".into(),
                    how_to_fix: "Always provide `else` when using `if` as an expression.".into(),
                },
            ],
        },
        Lesson {
            id: "ownership".into(),
            title: "Ownership".into(),
            description: "Ownership is Rust's most unique feature. Understand move semantics, borrowing, and references.".into(),
            level: "intermediate".into(),
            starter_code: "fn main() {\n    let s = String::from(\"hello\");\n    print_string(s);\n    println!(\"{}\", s); // This will error — fix it!\n}\n\nfn print_string(s: String) {\n    println!(\"{}\", s);\n}".into(),
            validator: Validator::OutputContains { expected: "hello".into() },
            objectives: vec![
                "Understand that `String` is moved, not copied".into(),
                "Learn about references (`&`) and borrowing".into(),
                "Fix ownership errors using borrowing".into(),
            ],
            explanation: r#"
<h3>The Ownership Rules</h3>
<ol>
<li>Each value has an <strong>owner</strong>.</li>
<li>There can only be one owner at a time.</li>
<li>When the owner goes out of scope, the value is dropped.</li>
</ol>
<h3>Move Semantics</h3>
<p>When you pass a <code>String</code> to a function, the <strong>ownership moves</strong>. The original variable can no longer use it:</p>
<pre><code>let s = String::from("hello");
take_ownership(s);  // s is moved
// s is invalid here!</code></pre>
<h3>Borrowing with References</h3>
<p>Instead of giving away ownership, you can <strong>borrow</strong> a value using a reference:</p>
<pre><code>fn print_string(s: &String) {  // Borrow, don't take
    println!("{}", s);
}

let s = String::from("hello");
print_string(&s);  // s is borrowed
println!("{}", s); // s is still valid!</code></pre>
<p>The function signature changes to <code>&String</code> (a reference), and you call it with <code>&s</code>.</p>
<h3>Why Not Just Copy Everything?</h3>
<p>Copying a <code>String</code> means duplicating heap memory. That's expensive. References let multiple parts of your code read the same data without copying or fighting over ownership.</p>
"#.into(),
            hints: vec![
                "Change `print_string`'s parameter from `String` to `&String`".into(),
                "Change the call from `print_string(s)` to `print_string(&s)`".into(),
                "The `println!` after the call should now work because `s` was borrowed, not moved".into(),
            ],
            common_mistakes: vec![
                CommonMistake {
                    mistake: "Cloning the string: `print_string(s.clone())`".into(),
                    why_it_happens: "This works but is inefficient — it duplicates heap memory unnecessarily.".into(),
                    how_to_fix: "Use borrowing (`&s`) instead of cloning.".into(),
                },
                CommonMistake {
                    mistake: "Changing only the function parameter but not the call site".into(),
                    why_it_happens: "Both the parameter type and the call must use references.".into(),
                    how_to_fix: "Use `&String` in the function AND `&s` when calling it.".into(),
                },
            ],
        },
        Lesson {
            id: "structs".into(),
            title: "Structs".into(),
            description: "Define custom data types with structs. Add methods with `impl` blocks.".into(),
            level: "intermediate".into(),
            starter_code: "fn main() {\n    // Create a Rectangle and print its area\n}\n\n// Define Rectangle and its area method here".into(),
            validator: Validator::OutputContains { expected: "200".into() },
            objectives: vec![
                "Define a struct with named fields".into(),
                "Create an instance with field init shorthand".into(),
                "Implement methods using `impl` blocks".into(),
                "Understand `&self` in methods".into(),
            ],
            explanation: r#"
<h3>Defining Structs</h3>
<p>A struct lets you group related data under a single name:</p>
<pre><code>struct Rectangle {
    width: u32,
    height: u32,
}</code></pre>
<h3>Creating Instances</h3>
<pre><code>let rect = Rectangle {
    width: 10,
    height: 20,
};</code></pre>
<h3>Methods with `impl`</h3>
<p>Use an <code>impl</code> block to add methods to a struct:</p>
<pre><code>impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}</code></pre>
<h3>`&self`</h3>
<p><code>&self</code> is short for <code>self: &Self</code> — a reference to the instance. It borrows the struct so you can use it after calling the method. Without <code>&</code>, the method would take ownership.</p>
<h3>Method Call Syntax</h3>
<p>Use dot notation: <code>rect.area()</code>. Rust automatically handles references, so you can call <code>area()</code> on <code>rect</code> even though it expects <code>&self</code>.</p>
"#.into(),
            hints: vec![
                "Define `struct Rectangle { width: u32, height: u32 }`".into(),
                "In `impl Rectangle`, define `fn area(&self) -> u32 { self.width * self.height }`".into(),
                "Create the rectangle with `Rectangle { width: 10, height: 20 }` and print `rect.area()`".into(),
            ],
            common_mistakes: vec![
                CommonMistake {
                    mistake: "Using `self` instead of `&self`".into(),
                    why_it_happens: "`self` takes ownership. After `rect.area()`, `rect` would be invalid.".into(),
                    how_to_fix: "Use `&self` so the method borrows instead of taking ownership.".into(),
                },
                CommonMistake {
                    mistake: "Forgetting the `impl` keyword: `Rectangle { fn area...`".into(),
                    why_it_happens: "Methods must be inside an `impl BlockName { }` block.".into(),
                    how_to_fix: "Wrap methods in `impl Rectangle { ... }`.".into(),
                },
            ],
        },
        Lesson {
            id: "enums".into(),
            title: "Enums & Pattern Matching".into(),
            description: "Enums let a value be one of several variants. Use `match` to handle every possibility.".into(),
            level: "intermediate".into(),
            starter_code: "fn main() {\n    let msg = Message::Move { x: 10, y: 20 };\n    // Match on msg and print the coordinates\n}\n\n// Define Message enum here".into(),
            validator: Validator::OutputContains { expected: "10".into() },
            objectives: vec![
                "Define an enum with multiple variants".into(),
                "Use `match` for exhaustive pattern matching".into(),
                "Understand that `match` must handle every variant".into(),
            ],
            explanation: r#"
<h3>Enums in Rust</h3>
<p>Rust enums are <strong>algebraic data types</strong> — each variant can hold different data:</p>
<pre><code>enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}</code></pre>
<h3>Pattern Matching with `match`</h3>
<p><code>match</code> is like a super-powered <code>switch</code> that <strong>must</strong> handle every possible variant:</p>
<pre><code>match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    Message::Write(text) => println!("Text: {}", text),
}</code></pre>
<p>The compiler checks exhaustiveness — if you add a variant later, every <code>match</code> that uses the enum will need updating. This prevents bugs.</p>
<h3>Destructuring</h3>
<p>In a <code>match</code> arm, you can extract values from variants:</p>
<pre><code>Message::Move { x, y } => {
    // x and y are now local variables
}</code></pre>
"#.into(),
            hints: vec![
                "Define the `Message` enum with `Quit`, `Move { x: i32, y: i32 }`, and `Write(String)`".into(),
                "Use `match msg { ... }` in main".into(),
                "In the `Move` arm, destructure with `Message::Move { x, y }` and print both coordinates".into(),
            ],
            common_mistakes: vec![
                CommonMistake {
                    mistake: "Missing a variant in the `match`".into(),
                    why_it_happens: "Rust requires exhaustive matching. Every variant must have an arm.".into(),
                    how_to_fix: "Add an arm for every enum variant, or use `_ =>` as a catch-all.".into(),
                },
                CommonMistake {
                    mistake: "Forgetting curly braces in `Move { x, y }`".into(),
                    why_it_happens: "Named fields in variants use struct-like syntax with braces.".into(),
                    how_to_fix: "Write `Message::Move { x, y } =>` not `Message::Move(x, y) =>`.".into(),
                },
            ],
        },
        Lesson {
            id: "error-handling".into(),
            title: "Error Handling".into(),
            description: "Rust doesn't use exceptions. It uses `Result<T, E>` and the `?` operator for elegant error propagation.".into(),
            level: "intermediate".into(),
            starter_code: "fn main() -> Result<(), Box<dyn std::error::Error>> {\n    // Call parse_number and print the result\n    Ok(())\n}\n\nfn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {\n    // Your code here\n}".into(),
            validator: Validator::OutputContains { expected: "42".into() },
            objectives: vec![
                "Understand `Result<T, E>` as the primary error type".into(),
                "Use `?` to propagate errors".into(),
                "Learn why Rust avoids exceptions".into(),
            ],
            explanation: r#"
<h3>`Result<T, E>`</h3>
<p>Rust represents fallible operations with <code>Result</code>:</p>
<pre><code>enum Result<T, E> {
    Ok(T),   // Success, contains the value
    Err(E),  // Failure, contains the error
}</code></pre>
<h3>The `?` Operator</h3>
<p>Instead of manually matching on every <code>Result</code>, use <code>?</code> to propagate errors:</p>
<pre><code>let num = parse_number("42")?; // If Err, return it immediately</code></pre>
<p><code>?</code> works in functions that return <code>Result</code>. It unwraps <code>Ok</code> values and early-returns <code>Err</code> values.</p>
<h3>No Exceptions</h3>
<p>Rust doesn't have exceptions. Every possible failure is visible in the type signature. This makes error handling explicit and composable — you can't forget to handle an error because the compiler won't let you.</p>
<h3>Boxed Errors</h3>
<p><code>Box<dyn std::error::Error></code> is a trait object that can hold any error type. It's useful in <code>main()</code> when you don't know or care about the exact error type.</p>
"#.into(),
            hints: vec![
                "In `parse_number`, call `s.parse::<i32>()`".into(),
                "`parse()` returns a `Result` — you can just return it directly since the types match".into(),
                "In `main()`, use `let num = parse_number(\"42\")?;` and print it".into(),
            ],
            common_mistakes: vec![
                CommonMistake {
                    mistake: "Using `.unwrap()` instead of `?`".into(),
                    why_it_happens: "`.unwrap()` panics on error. It defeats Rust's safety guarantees.".into(),
                    how_to_fix: "Use `?` to propagate errors properly.".into(),
                },
                CommonMistake {
                    mistake: "Forgetting `Ok(())` at the end of main".into(),
                    why_it_happens: "`main()` returns `Result<(), ...>`. The last expression must be `Ok(())` to signal success.".into(),
                    how_to_fix: "End `main()` with `Ok(())`.".into(),
                },
            ],
        },
        Lesson {
            id: "collections".into(),
            title: "Collections".into(),
            description: "Rust's standard library provides powerful collections. Learn `Vec`, `HashMap`, and when to use each.".into(),
            level: "intermediate".into(),
            starter_code: "use std::collections::HashMap;\n\nfn main() {\n    // Create and populate the HashMap\n    // Print Alice's score\n}".into(),
            validator: Validator::OutputContains { expected: "100".into() },
            objectives: vec![
                "Create and populate a `HashMap`".into(),
                "Understand ownership with collections".into(),
                "Retrieve values with `.get()`".into(),
            ],
            explanation: r#"
<h3>`Vec<T>` — Growable Array</h3>
<p>Stores values of the same type in a contiguous heap allocation:</p>
<pre><code>let mut v = Vec::new();
v.push(1);
v.push(2);</code></pre>
<h3>`HashMap<K, V>` — Key-Value Store</h3>
<p>Maps keys to values using a hash function:</p>
<pre><code>let mut scores = HashMap::new();
scores.insert("Alice", 100);
scores.insert("Bob", 85);</code></pre>
<h3>Retrieving Values</h3>
<p><code>.get()</code> returns <code>Option<&V></code> because the key might not exist:</p>
<pre><code>match scores.get("Alice") {
    Some(score) => println!("{}", score),
    None => println!("Not found"),
}</code></pre>
<h3>Ownership Gotcha</h3>
<p>If you insert owned values (like <code>String</code>), the <code>HashMap</code> takes ownership. You can't use the original variable afterward. Use <code>&str</code> or clone if you need the original.</p>
"#.into(),
            hints: vec![
                "Create with `let mut scores = HashMap::new();`".into(),
                "Insert with `scores.insert(\"Alice\", 100);` and `scores.insert(\"Bob\", 85);`".into(),
                "Get Alice's score with `scores.get(\"Alice\")` and handle the `Option`".into(),
            ],
            common_mistakes: vec![
                CommonMistake {
                    mistake: "Using `scores[\"Alice\"]` like in Python".into(),
                    why_it_happens: "Rust doesn't have indexing for HashMap. Use `.get()` which returns `Option`.".into(),
                    how_to_fix: "Use `scores.get(\"Alice\")` and pattern match or unwrap carefully.".into(),
                },
                CommonMistake {
                    mistake: "Forgetting to `use std::collections::HashMap;`".into(),
                    why_it_happens: "`HashMap` is not in the prelude — you must import it.".into(),
                    how_to_fix: "Add `use std::collections::HashMap;` at the top.".into(),
                },
            ],
        },
        Lesson {
            id: "traits".into(),
            title: "Traits".into(),
            description: "Traits define shared behavior. They're like interfaces, but more powerful. Learn to define and implement traits.".into(),
            level: "advanced".into(),
            starter_code: "fn main() {\n    let article = NewsArticle {\n        headline: String::from(\"Rust 1.80 Released\"),\n        location: String::from(\"Internet\"),\n    };\n    println!(\"{}\", article.summarize());\n}\n\n// Define Summarize trait and implement for NewsArticle".into(),
            validator: Validator::OutputContains { expected: "Rust 1.80".into() },
            objectives: vec![
                "Define a trait with method signatures".into(),
                "Implement a trait for a struct".into(),
                "Understand trait methods vs struct methods".into(),
            ],
            explanation: r#"
<h3>What Are Traits?</h3>
<p>Traits define a set of methods that a type must implement. They're Rust's way of defining <strong>shared behavior</strong> — similar to interfaces in other languages, but with more flexibility.</p>
<h3>Defining a Trait</h3>
<pre><code>pub trait Summarize {
    fn summarize(&self) -> String;
}</code></pre>
<h3>Implementing a Trait</h3>
<pre><code>impl Summarize for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} (from {})", self.headline, self.location)
    }
}</code></pre>
<h3>Trait Methods vs Inherent Methods</h3>
<ul>
<li><strong>Inherent methods</strong>: Defined in `impl StructName { }` — belong to the struct directly.</li>
<li><strong>Trait methods</strong>: Defined in `impl TraitName for StructName { }` — brought into scope by the trait.</li>
</ul>
<h3>Default Implementations</h3>
<p>Traits can provide default implementations that types can override:</p>
<pre><code>trait Summarize {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}</code></pre>
"#.into(),
            hints: vec![
                "Define `trait Summarize { fn summarize(&self) -> String; }`".into(),
                "Define `struct NewsArticle { headline: String, location: String }`".into(),
                "Write `impl Summarize for NewsArticle { fn summarize(&self) -> String { ... } }`".into(),
            ],
            common_mistakes: vec![
                CommonMistake {
                    mistake: "Defining the struct inside the trait impl".into(),
                    why_it_happens: "`impl Trait for Struct` requires the struct to be defined separately.".into(),
                    how_to_fix: "Define `struct NewsArticle` first, then `impl Summarize for NewsArticle`.".into(),
                },
                CommonMistake {
                    mistake: "Forgetting `for` in `impl Summarize for NewsArticle`".into(),
                    why_it_happens: "The syntax requires `for` to link the trait to the type.".into(),
                    how_to_fix: "Write `impl Summarize for NewsArticle`, not `impl Summarize NewsArticle`.".into(),
                },
            ],
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
