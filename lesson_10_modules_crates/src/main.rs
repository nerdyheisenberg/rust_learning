// ============================================================================
// LESSON 10: Modules, Crates & Project Architecture
// ============================================================================
// Since this lesson is about modules, the main.rs shows how to use them.
// The module system itself is demonstrated through code organization.
// ============================================================================

// Module declarations
mod shapes;      // Loads from shapes.rs or shapes/mod.rs
mod utils;       // Loads from utils.rs

// Re-export for convenience
pub use shapes::Shape;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 10: Modules, Crates & Project Architecture     ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Module Basics
    // ========================================================================
    println!("═══ Section 1: Module Basics ═══\n");

    // Inline module — defined right here
    mod greetings {
        pub fn hello(name: &str) {
            println!("  Hello, {}!", name);
        }

        pub fn goodbye(name: &str) {
            println!("  Goodbye, {}!", name);
        }

        // Private function — not accessible outside this module
        fn _secret() {
            println!("  This is private");
        }

        // Nested module
        pub mod formal {
            pub fn greet(name: &str) {
                println!("  Good day, {}. How do you do?", name);
            }
        }
    }

    greetings::hello("Rohit");
    greetings::goodbye("Rohit");
    greetings::formal::greet("Sir Rohit");

    // ========================================================================
    // SECTION 2: Visibility Modifiers
    // ========================================================================
    println!("\n═══ Section 2: Visibility ═══\n");

    // pub          — public to all
    // pub(crate)   — public within the crate only
    // pub(super)   — public to parent module
    // pub(in path) — public within specified path
    // (default)    — private to current module

    mod outer {
        pub struct Public {
            pub field: i32,           // Accessible everywhere
            pub(crate) crate_only: i32,   // Accessible within crate
            private: i32,              // Only in this module
        }

        impl Public {
            pub fn new(a: i32, b: i32, c: i32) -> Self {
                Public { field: a, crate_only: b, private: c }
            }

            pub fn get_private(&self) -> i32 {
                self.private
            }
        }
    }

    let p = outer::Public::new(1, 2, 3);
    println!("Public field: {}", p.field);
    println!("Crate-only field: {}", p.crate_only);
    // println!("Private: {}", p.private);  // ❌ ERROR: private
    println!("Private (via method): {}", p.get_private());

    // ========================================================================
    // SECTION 3: Using Separate Module Files
    // ========================================================================
    println!("\n═══ Section 3: File-based Modules ═══\n");

    // Using the shapes module (from shapes.rs)
    let circle = shapes::Circle::new(5.0);
    let rect = shapes::Rectangle::new(10.0, 5.0);
    println!("Circle area: {:.2}", circle.area());
    println!("Rectangle area: {:.2}", rect.area());

    // Using re-exported Shape trait
    let shapes_vec: Vec<Box<dyn Shape>> = vec![
        Box::new(circle),
        Box::new(rect),
    ];
    for shape in &shapes_vec {
        println!("  {} area = {:.2}", shape.name(), shape.area());
    }

    // Using utils module
    let result = utils::math::add(5, 3);
    println!("utils::math::add(5, 3) = {}", result);

    let formatted = utils::strings::capitalize("hello world");
    println!("capitalize: {}", formatted);

    // ========================================================================
    // SECTION 4: use and Path Resolution
    // ========================================================================
    println!("\n═══ Section 4: use Statements ═══\n");

    // Importing specific items
    use greetings::hello;
    hello("World");

    // Importing with alias
    use greetings::formal::greet as formal_greet;
    formal_greet("World");

    // Glob import (import everything public)
    // use greetings::*;  // Generally discouraged in application code

    // ========================================================================
    // SECTION 5: Conditional Compilation
    // ========================================================================
    println!("\n═══ Section 5: Conditional Compilation ═══\n");

    // #[cfg(...)] — compile code conditionally

    #[cfg(target_os = "linux")]
    println!("  Running on Linux!");

    #[cfg(target_os = "windows")]
    println!("  Running on Windows!");

    #[cfg(target_os = "macos")]
    println!("  Running on macOS!");

    // cfg! macro — evaluates at compile time, returns bool
    if cfg!(debug_assertions) {
        println!("  Debug mode enabled");
    } else {
        println!("  Release mode");
    }

    println!("  Target OS: {}", std::env::consts::OS);
    println!("  Target arch: {}", std::env::consts::ARCH);

    // ========================================================================
    // SECTION 6: Tests Organization
    // ========================================================================
    println!("\n═══ Section 6: Test Organization ═══\n");

    // Unit tests go in the same file with #[cfg(test)]
    // Integration tests go in tests/ directory
    // Doc tests go in documentation comments

    println!("  Run tests with: cargo test");
    println!("  Unit tests: same file, #[cfg(test)] mod tests {{ }}");
    println!("  Integration tests: tests/ directory");
    println!("  Doc tests: in /// documentation comments");

    println!("\n✅ Lesson 10 Complete!");
    println!("   Next: Lesson 11 — Smart Pointers & Memory Management");
}

// ============================================================================
// UNIT TESTS — Example of test organization
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shapes() {
        let circle = shapes::Circle::new(1.0);
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_utils() {
        assert_eq!(utils::math::add(2, 3), 5);
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(utils::strings::capitalize("hello"), "Hello");
    }

    #[test]
    #[should_panic(expected = "negative")]
    fn test_negative_radius() {
        shapes::Circle::new(-1.0);
    }
}
