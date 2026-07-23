// ============================================================================
// LESSON 5: Structs, Enums & Pattern Matching
// ============================================================================

use std::fmt;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 5: Structs, Enums & Pattern Matching           ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Named Structs
    // ========================================================================
    println!("═══ Section 1: Named Structs ═══\n");

    // Named struct — like C++ struct
    #[derive(Debug, Clone)]
    struct User {
        username: String,
        email: String,
        active: bool,
        sign_in_count: u64,
    }

    let user1 = User {
        username: String::from("rohit"),
        email: String::from("rohit@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("User: {:?}", user1);

    // Struct update syntax (like spread operator in JS):
    let user2 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        ..user1.clone()  // Copy remaining fields from user1
    };
    println!("User2: {:?}", user2);

    // Field init shorthand (when variable name matches field name):
    let username = String::from("bob");
    let email = String::from("bob@example.com");
    let user3 = User {
        username,  // Shorthand — same as username: username
        email,     // Shorthand
        active: true,
        sign_in_count: 0,
    };
    println!("User3: {:?}", user3);

    // ========================================================================
    // SECTION 2: impl Blocks — Methods and Associated Functions
    // ========================================================================
    println!("\n═══ Section 2: impl Blocks ═══\n");

    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        // Associated function (like static method in C++) — no self
        fn new(width: f64, height: f64) -> Self {
            Rectangle { width, height }
        }

        fn square(size: f64) -> Self {
            Rectangle { width: size, height: size }
        }

        // Method — takes &self (immutable borrow)
        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }

        // Method — takes &mut self (mutable borrow)
        fn scale(&mut self, factor: f64) {
            self.width *= factor;
            self.height *= factor;
        }

        // Method — takes self (ownership) — consumes the rectangle
        fn into_square(self) -> Rectangle {
            let side = (self.width * self.height).sqrt();
            Rectangle { width: side, height: side }
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    // Implement Display trait for pretty printing
    impl fmt::Display for Rectangle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Rectangle({}×{})", self.width, self.height)
        }
    }

    let mut rect = Rectangle::new(10.0, 20.0);
    println!("Rect: {}", rect);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());

    rect.scale(2.0);
    println!("After scale(2): {}", rect);

    let square = Rectangle::square(5.0);
    println!("Square: {}", square);

    let small = Rectangle::new(5.0, 10.0);
    println!("Can rect hold small? {}", rect.can_hold(&small));

    // into_square consumes the rectangle
    let sq = small.into_square();
    println!("Converted to square: {}", sq);
    // println!("{}", small);  // ❌ small was consumed

    // ========================================================================
    // SECTION 3: Tuple Structs and Unit Structs
    // ========================================================================
    println!("\n═══ Section 3: Tuple & Unit Structs ═══\n");

    // Tuple struct — named tuple (useful for newtype pattern)
    #[derive(Debug)]
    struct Color(u8, u8, u8);

    #[derive(Debug)]
    struct Meters(f64);

    #[derive(Debug)]
    struct Kilograms(f64);

    let red = Color(255, 0, 0);
    let distance = Meters(100.0);
    let weight = Kilograms(75.5);

    println!("Color: {:?} (R={}, G={}, B={})", red, red.0, red.1, red.2);
    println!("Distance: {:?}", distance);
    println!("Weight: {:?}", weight);

    // Newtype pattern prevents mixing up types:
    // let wrong: Meters = weight;  // ❌ Type error! Can't assign Kilograms to Meters

    // Unit struct — zero-sized type (like an empty struct in C++)
    #[derive(Debug)]
    struct Marker;  // No fields, no memory

    let _m = Marker;
    println!("Marker size: {} bytes", std::mem::size_of::<Marker>());

    // ========================================================================
    // SECTION 4: Enums — Powerful Tagged Unions
    // ========================================================================
    println!("\n═══ Section 4: Enums ═══\n");

    // Basic enum (like C++ enum class)
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;
    println!("Direction: {:?}", dir);

    // Enum with data in each variant (MUCH more powerful than C++)
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    // Enum with varied data types
    #[derive(Debug)]
    enum Message {
        Quit,                          // No data (unit variant)
        Move { x: i32, y: i32 },       // Named fields (struct variant)
        Write(String),                  // Single value (tuple variant)
        ChangeColor(u8, u8, u8),        // Multiple values (tuple variant)
    }

    impl Message {
        fn process(&self) {
            match self {
                Message::Quit => println!("  Quit message"),
                Message::Move { x, y } => println!("  Move to ({}, {})", x, y),
                Message::Write(text) => println!("  Write: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!("  Change color to RGB({}, {}, {})", r, g, b)
                }
            }
        }
    }

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in &messages {
        msg.process();
    }

    // Memory layout: enum size = tag + largest variant
    println!("\nEnum sizes:");
    println!("  Direction: {} bytes", std::mem::size_of::<Direction>());
    println!("  IpAddr: {} bytes", std::mem::size_of::<IpAddr>());
    println!("  Message: {} bytes", std::mem::size_of::<Message>());

    // ========================================================================
    // SECTION 5: Option<T> — Replacing Null
    // ========================================================================
    println!("\n═══ Section 5: Option<T> ═══\n");

    // Option<T> is Rust's replacement for null
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("no_number: {:?}", no_number);

    // Niche optimization: Option<&T> uses null pointer for None
    println!("Size of Option<&i32>: {} bytes (same as &i32!)",
             std::mem::size_of::<Option<&i32>>());
    println!("Size of &i32: {} bytes", std::mem::size_of::<&i32>());

    // Working with Option using match:
    match some_number {
        Some(n) => println!("Got: {}", n),
        None => println!("Got nothing"),
    }

    // Option methods (functional style):
    let x: Option<i32> = Some(5);
    println!("unwrap_or: {}", x.unwrap_or(0));
    println!("map: {:?}", x.map(|n| n * 2));
    println!("and_then: {:?}", x.and_then(|n| if n > 3 { Some(n) } else { None }));
    println!("filter: {:?}", x.filter(|&n| n > 3));
    println!("is_some: {}, is_none: {}", x.is_some(), x.is_none());

    let none: Option<i32> = None;
    println!("unwrap_or(99): {}", none.unwrap_or(99));
    println!("unwrap_or_else: {}", none.unwrap_or_else(|| 42 * 2));

    // Option in real use: safe array access
    let v = vec![1, 2, 3];
    let third: Option<&i32> = v.get(2);   // Safe — returns Option
    let tenth: Option<&i32> = v.get(10);  // Safe — returns None
    println!("v.get(2) = {:?}", third);
    println!("v.get(10) = {:?}", tenth);

    // ========================================================================
    // SECTION 6: Result<T, E> — Error Handling
    // ========================================================================
    println!("\n═══ Section 6: Result<T, E> ═══\n");

    // Result<T, E> for operations that can fail
    let good: Result<i32, String> = Ok(42);
    let bad: Result<i32, String> = Err(String::from("something went wrong"));

    println!("good: {:?}", good);
    println!("bad: {:?}", bad);

    // Pattern matching on Result
    match divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.4}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // The ? operator for error propagation
    match process_data("42") {
        Ok(result) => println!("Processed: {}", result),
        Err(e) => println!("Processing error: {}", e),
    }

    match process_data("not_a_number") {
        Ok(result) => println!("Processed: {}", result),
        Err(e) => println!("Processing error: {}", e),
    }

    // Result methods
    let r: Result<i32, &str> = Ok(5);
    println!("map: {:?}", r.map(|n| n * 2));
    println!("unwrap_or: {}", r.unwrap_or(0));
    println!("is_ok: {}, is_err: {}", r.is_ok(), r.is_err());

    // ========================================================================
    // SECTION 7: Advanced Pattern Matching with Enums
    // ========================================================================
    println!("\n═══ Section 7: Advanced Pattern Matching ═══\n");

    #[derive(Debug)]
    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Triangle { base: f64, height: f64 },
        Point,
    }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
                Shape::Rectangle { width, height } => width * height,
                Shape::Triangle { base, height } => 0.5 * base * height,
                Shape::Point => 0.0,
            }
        }

        fn describe(&self) -> String {
            match self {
                Shape::Circle { radius } if *radius > 10.0 => {
                    format!("Large circle (r={})", radius)
                }
                Shape::Circle { radius } => format!("Small circle (r={})", radius),
                shape => format!("{:?} with area {:.2}", shape, shape.area()),
            }
        }
    }

    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Circle { radius: 15.0 },
        Shape::Rectangle { width: 10.0, height: 5.0 },
        Shape::Triangle { base: 8.0, height: 6.0 },
        Shape::Point,
    ];

    for shape in &shapes {
        println!("  {} → area = {:.2}", shape.describe(), shape.area());
    }

    // ========================================================================
    // SECTION 8: Builder Pattern
    // ========================================================================
    println!("\n═══ Section 8: Builder Pattern ═══\n");

    #[derive(Debug)]
    struct Server {
        host: String,
        port: u16,
        max_connections: u32,
        timeout_ms: u64,
        tls_enabled: bool,
    }

    struct ServerBuilder {
        host: String,
        port: u16,
        max_connections: u32,
        timeout_ms: u64,
        tls_enabled: bool,
    }

    impl ServerBuilder {
        fn new(host: &str, port: u16) -> Self {
            ServerBuilder {
                host: host.to_string(),
                port,
                max_connections: 100,   // Default
                timeout_ms: 30000,      // Default: 30 seconds
                tls_enabled: false,     // Default: no TLS
            }
        }

        fn max_connections(mut self, max: u32) -> Self {
            self.max_connections = max;
            self
        }

        fn timeout(mut self, ms: u64) -> Self {
            self.timeout_ms = ms;
            self
        }

        fn enable_tls(mut self) -> Self {
            self.tls_enabled = true;
            self
        }

        fn build(self) -> Server {
            Server {
                host: self.host,
                port: self.port,
                max_connections: self.max_connections,
                timeout_ms: self.timeout_ms,
                tls_enabled: self.tls_enabled,
            }
        }
    }

    let server = ServerBuilder::new("localhost", 8080)
        .max_connections(1000)
        .timeout(60000)
        .enable_tls()
        .build();

    println!("Server: {:#?}", server);

    // ========================================================================
    // SECTION 9: Enum-based State Machine
    // ========================================================================
    println!("\n═══ Section 9: State Machine with Enums ═══\n");

    #[derive(Debug)]
    enum ConnectionState {
        Disconnected,
        Connecting { attempt: u32 },
        Connected { session_id: String },
        Error { message: String },
    }

    impl ConnectionState {
        fn transition(&self) -> ConnectionState {
            match self {
                ConnectionState::Disconnected => {
                    println!("  Trying to connect...");
                    ConnectionState::Connecting { attempt: 1 }
                }
                ConnectionState::Connecting { attempt } if *attempt < 3 => {
                    println!("  Connection attempt {} failed, retrying...", attempt);
                    ConnectionState::Connecting { attempt: attempt + 1 }
                }
                ConnectionState::Connecting { attempt } if *attempt >= 3 => {
                    println!("  Max retries reached!");
                    ConnectionState::Error {
                        message: format!("Failed after {} attempts", attempt),
                    }
                }
                ConnectionState::Connected { session_id } => {
                    println!("  Session {} active", session_id);
                    ConnectionState::Connected {
                        session_id: session_id.clone(),
                    }
                }
                ConnectionState::Error { message } => {
                    println!("  Error state: {}", message);
                    ConnectionState::Disconnected
                }
                _ => ConnectionState::Disconnected,
            }
        }
    }

    let mut state = ConnectionState::Disconnected;
    for _ in 0..6 {
        println!("  State: {:?}", state);
        state = state.transition();
    }

    println!("\n✅ Lesson 5 Complete! Run: cargo run");
    println!("   Next: Lesson 6 — Traits (Rust's Polymorphism)");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn process_data(input: &str) -> Result<i32, String> {
    let number: i32 = input.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
    let doubled = number * 2;
    if doubled > 100 {
        Err(String::from("Result too large"))
    } else {
        Ok(doubled)
    }
}
