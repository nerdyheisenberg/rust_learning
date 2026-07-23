// ============================================================================
// LESSON 9: Error Handling — The Complete Picture
// ============================================================================

use std::fmt;
use std::num::ParseIntError;
use std::io;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 9: Error Handling — The Complete Picture       ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: unwrap, expect, and friends
    // ========================================================================
    println!("═══ Section 1: unwrap Variants ═══\n");

    // unwrap() — panics on Err/None
    let x: Result<i32, &str> = Ok(42);
    println!("unwrap: {}", x.unwrap());

    // expect() — panics with custom message
    let y: Option<i32> = Some(42);
    println!("expect: {}", y.expect("Value should exist"));

    // unwrap_or() — default value on failure
    let none: Option<i32> = None;
    println!("unwrap_or: {}", none.unwrap_or(0));

    // unwrap_or_else() — lazy default (computed only if needed)
    let err: Result<i32, &str> = Err("failed");
    println!("unwrap_or_else: {}", err.unwrap_or_else(|e| {
        println!("  (Error was: {})", e);
        -1
    }));

    // unwrap_or_default() — uses Default trait
    let none: Option<i32> = None;
    println!("unwrap_or_default: {}", none.unwrap_or_default());  // 0

    // ========================================================================
    // SECTION 2: Combinators on Result/Option
    // ========================================================================
    println!("\n═══ Section 2: Combinators ═══\n");

    // map — transform the Ok/Some value
    let x: Result<i32, &str> = Ok(5);
    let mapped = x.map(|v| v * 2);
    println!("map: {:?}", mapped);  // Ok(10)

    // map_err — transform the error
    let x: Result<i32, i32> = Err(5);
    let mapped = x.map_err(|e| format!("Error code: {}", e));
    println!("map_err: {:?}", mapped);

    // and_then — chain operations (like flatMap/bind in other languages)
    let result = "42"
        .parse::<i32>()
        .and_then(|n| {
            if n > 0 { Ok(n * 2) } else { Err("must be positive".parse::<i32>().unwrap_err()) }
        });
    println!("and_then: {:?}", result);

    // or_else — try alternative on error
    fn parse_or_default(s: &str) -> Result<i32, ParseIntError> {
        s.parse::<i32>().or_else(|_| "0".parse::<i32>())
    }
    println!("or_else: {:?}", parse_or_default("abc"));

    // Option → Result conversion
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("none found");
    println!("ok_or: {:?}", res);

    let none: Option<i32> = None;
    let res: Result<i32, &str> = none.ok_or("none found");
    println!("ok_or (None): {:?}", res);

    // ========================================================================
    // SECTION 3: The ? Operator
    // ========================================================================
    println!("\n═══ Section 3: The ? Operator ═══\n");

    match read_username_uppercase("Rohit") {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error: {}", e),
    }

    // Chaining multiple ? operations
    match complex_operation("42") {
        Ok(result) => println!("Complex result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match complex_operation("not_a_number") {
        Ok(result) => println!("Complex result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // ========================================================================
    // SECTION 4: Custom Error Types
    // ========================================================================
    println!("\n═══ Section 4: Custom Error Types ═══\n");

    match process_config("port=8080") {
        Ok(port) => println!("Port: {}", port),
        Err(e) => println!("Config error: {}", e),
    }

    match process_config("port=abc") {
        Ok(port) => println!("Port: {}", port),
        Err(e) => println!("Config error: {}", e),
    }

    match process_config("invalid_format") {
        Ok(port) => println!("Port: {}", port),
        Err(e) => println!("Config error: {}", e),
    }

    // ========================================================================
    // SECTION 5: Error Conversion with From trait
    // ========================================================================
    println!("\n═══ Section 5: Error Conversion ═══\n");

    match read_and_parse("42") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // ========================================================================
    // SECTION 6: Box<dyn Error> — The Catch-All
    // ========================================================================
    println!("\n═══ Section 6: Box<dyn Error> ═══\n");

    // For applications, Box<dyn Error> is a quick catch-all:
    match app_logic() {
        Ok(result) => println!("App result: {}", result),
        Err(e) => println!("App error: {}", e),
    }

    // ========================================================================
    // SECTION 7: Handling Errors in Iterators
    // ========================================================================
    println!("\n═══ Section 7: Errors in Iterators ═══\n");

    let strings = vec!["1", "2", "three", "4", "five"];

    // Collect all results — stops at first error
    let result: Result<Vec<i32>, _> = strings.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Collect Result: {:?}", result);  // Err at "three"

    // Filter only successful parses
    let numbers: Vec<i32> = strings.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("filter_map (skip errors): {:?}", numbers);

    // Partition into successes and failures
    let (successes, failures): (Vec<_>, Vec<_>) = strings.iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let successes: Vec<i32> = successes.into_iter().map(Result::unwrap).collect();
    let failures: Vec<ParseIntError> = failures.into_iter().map(Result::unwrap_err).collect();
    println!("Successes: {:?}", successes);
    println!("Failures: {} errors", failures.len());

    // ========================================================================
    // SECTION 8: panic!, catch_unwind, and Process Abort
    // ========================================================================
    println!("\n═══ Section 8: panic! and Recovery ═══\n");

    // panic! is for unrecoverable errors
    // panic!("This would crash the program");

    // catch_unwind — catch panics (like try/catch, but NOT recommended for flow control)
    use std::panic;

    let result = panic::catch_unwind(|| {
        println!("  Before potential panic");
        let v = vec![1, 2, 3];
        // This would panic but catch_unwind catches it:
        // v[10]
        42  // Return normally
    });
    println!("  catch_unwind result: {:?}", result);

    // Catching an actual panic:
    let result = panic::catch_unwind(|| {
        let v: Vec<i32> = vec![1, 2, 3];
        v[10]  // Index out of bounds — panics!
    });
    println!("  Caught panic: {:?}", result.is_err());

    println!("\n✅ Lesson 9 Complete!");
    println!("   Next: Lesson 10 — Modules, Crates & Project Architecture");
}

// ============================================================================
// HELPER FUNCTIONS AND TYPES
// ============================================================================

fn read_username_uppercase(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err(String::from("Name cannot be empty"));
    }
    Ok(name.to_uppercase())
}

fn complex_operation(input: &str) -> Result<String, String> {
    let number = input.parse::<i32>()
        .map_err(|e| format!("Parse error: {}", e))?;  // ? propagates Err

    let doubled = number.checked_mul(2)
        .ok_or_else(|| String::from("Overflow!"))?;     // Convert Option to Result, then ?

    Ok(format!("{} × 2 = {}", number, doubled))
}

// --- Custom Error Type ---
#[derive(Debug)]
enum ConfigError {
    ParseError(ParseIntError),
    FormatError(String),
    IoError(io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::ParseError(e) => write!(f, "Parse error: {}", e),
            ConfigError::FormatError(msg) => write!(f, "Format error: {}", msg),
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::ParseError(e) => Some(e),
            ConfigError::IoError(e) => Some(e),
            ConfigError::FormatError(_) => None,
        }
    }
}

// Implement From for automatic conversion with ?
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::ParseError(e)
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::IoError(e)
    }
}

fn process_config(input: &str) -> Result<u16, ConfigError> {
    let parts: Vec<&str> = input.split('=').collect();
    if parts.len() != 2 {
        return Err(ConfigError::FormatError(
            format!("Expected 'key=value', got '{}'", input)
        ));
    }
    let port = parts[1].parse::<u16>()?;  // ? auto-converts ParseIntError → ConfigError
    Ok(port)
}

// --- Application-level Error with Box<dyn Error> ---
type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

fn read_and_parse(s: &str) -> AppResult<i32> {
    let n = s.parse::<i32>()?;  // ParseIntError auto-boxed
    Ok(n * 2)
}

fn app_logic() -> AppResult<String> {
    let number = read_and_parse("21")?;
    Ok(format!("Computed: {}", number))
}
