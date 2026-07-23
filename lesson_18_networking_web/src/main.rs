// ============================================================================
// LESSON 18: Networking, Web & Systems Programming
// ============================================================================
// This lesson uses ONLY std library for networking basics.
// For real web services, you'd add: axum, serde, reqwest, sqlx, etc.
// ============================================================================

use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 18: Networking, Web & Systems Programming      ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: TCP Server & Client
    // ========================================================================
    println!("═══ Section 1: TCP Echo Server ═══\n");

    // Start echo server in background
    let server_handle = thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        println!("  Server listening on {}", addr);

        // Send the address back through a channel isn't possible here,
        // so we use a known port approach
        // For demo, we handle just 2 connections then stop
        for stream in listener.incoming().take(2) {
            match stream {
                Ok(mut stream) => {
                    let mut buf = [0u8; 1024];
                    let n = stream.read(&mut buf).unwrap();
                    let msg = String::from_utf8_lossy(&buf[..n]);
                    println!("  Server received: {}", msg.trim());
                    stream.write_all(format!("Echo: {}", msg).as_bytes()).unwrap();
                }
                Err(e) => eprintln!("  Server error: {}", e),
            }
        }
    });

    // Give server time to start
    thread::sleep(Duration::from_millis(100));

    // We'll demo the HTTP server pattern instead (more practical):

    // ========================================================================
    // SECTION 2: Simple HTTP Server (from scratch!)
    // ========================================================================
    println!("\n═══ Section 2: HTTP Server from Scratch ═══\n");

    let server_handle = thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:18080").unwrap();
        println!("  HTTP Server on http://127.0.0.1:18080");

        // Handle 3 requests then stop (for demo purposes)
        for stream in listener.incoming().take(3) {
            match stream {
                Ok(stream) => handle_http_connection(stream),
                Err(e) => eprintln!("  Error: {}", e),
            }
        }
        println!("  Server shutting down");
    });

    thread::sleep(Duration::from_millis(200));

    // Client sends HTTP requests
    for path in &["/", "/api/hello", "/not-found"] {
        match TcpStream::connect("127.0.0.1:18080") {
            Ok(mut stream) => {
                let request = format!("GET {} HTTP/1.1\r\nHost: localhost\r\n\r\n", path);
                stream.write_all(request.as_bytes()).unwrap();
                stream.shutdown(std::net::Shutdown::Write).unwrap();

                let mut response = String::new();
                stream.read_to_string(&mut response).unwrap();

                // Show just the first line of response
                let first_line = response.lines().next().unwrap_or("");
                println!("  GET {} → {}", path, first_line);
            }
            Err(e) => println!("  Client error: {}", e),
        }
    }

    let _ = server_handle.join();

    // ========================================================================
    // SECTION 3: JSON-like Serialization (Manual)
    // ========================================================================
    println!("\n═══ Section 3: Manual Serialization ═══\n");

    // In practice, use serde. Here's the concept manually:
    trait ToJson {
        fn to_json(&self) -> String;
    }

    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
        email: String,
        active: bool,
    }

    impl ToJson for User {
        fn to_json(&self) -> String {
            format!(
                r#"{{"name":"{}","age":{},"email":"{}","active":{}}}"#,
                self.name, self.age, self.email, self.active
            )
        }
    }

    let user = User {
        name: "Rohit".into(),
        age: 30,
        email: "rohit@example.com".into(),
        active: true,
    };
    println!("  JSON: {}", user.to_json());

    // Simple JSON parser (subset)
    fn parse_json_value(s: &str) -> Option<JsonValue> {
        let s = s.trim();
        if s == "true" { return Some(JsonValue::Bool(true)); }
        if s == "false" { return Some(JsonValue::Bool(false)); }
        if s == "null" { return Some(JsonValue::Null); }
        if let Ok(n) = s.parse::<f64>() { return Some(JsonValue::Number(n)); }
        if s.starts_with('"') && s.ends_with('"') {
            return Some(JsonValue::Str(s[1..s.len()-1].to_string()));
        }
        None
    }

    #[derive(Debug)]
    enum JsonValue {
        Null,
        Bool(bool),
        Number(f64),
        Str(String),
    }

    println!("  Parse '42': {:?}", parse_json_value("42"));
    println!("  Parse 'true': {:?}", parse_json_value("true"));
    println!("  Parse '\"hello\"': {:?}", parse_json_value("\"hello\""));

    // ========================================================================
    // SECTION 4: File System Operations
    // ========================================================================
    println!("\n═══ Section 4: File System Operations ═══\n");

    use std::fs;
    use std::path::Path;

    // Write file
    let path = "/tmp/rust_lesson_18_demo.txt";
    fs::write(path, "Hello from Rust!\nSecond line\nThird line").unwrap();
    println!("  Written to {}", path);

    // Read file
    let content = fs::read_to_string(path).unwrap();
    println!("  Read: {} chars", content.len());

    // Read line by line
    let file = fs::File::open(path).unwrap();
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        println!("  Line {}: {}", i, line.unwrap());
    }

    // File metadata
    let metadata = fs::metadata(path).unwrap();
    println!("  Size: {} bytes", metadata.len());
    println!("  Is file: {}", metadata.is_file());

    // Path operations
    let p = Path::new("/home/rohit/code/main.rs");
    println!("  Parent: {:?}", p.parent());
    println!("  File name: {:?}", p.file_name());
    println!("  Extension: {:?}", p.extension());
    println!("  Stem: {:?}", p.file_stem());

    // Cleanup
    fs::remove_file(path).unwrap();

    // ========================================================================
    // SECTION 5: Environment and Process
    // ========================================================================
    println!("\n═══ Section 5: Environment & Process ═══\n");

    // Environment variables
    println!("  HOME: {:?}", std::env::var("HOME"));
    println!("  PATH contains {} entries", std::env::var("PATH")
        .unwrap_or_default().split(':').count());

    // Current directory
    println!("  CWD: {:?}", std::env::current_dir().unwrap());

    // Command line arguments
    let args: Vec<String> = std::env::args().collect();
    println!("  Args: {:?}", args);

    // Process info
    println!("  PID: {}", std::process::id());

    // ========================================================================
    // SECTION 6: Serde & Axum Reference (Theory)
    // ========================================================================
    println!("\n═══ Section 6: Real-World Web Stack ═══\n");

    println!("  For production web services, add to Cargo.toml:");
    println!("  [dependencies]");
    println!("  axum = \"0.7\"            # Web framework");
    println!("  serde = {{ version = \"1\", features = [\"derive\"] }}");
    println!("  serde_json = \"1\"        # JSON serialization");
    println!("  tokio = {{ version = \"1\", features = [\"full\"] }}");
    println!("  reqwest = \"0.12\"        # HTTP client");
    println!("  sqlx = {{ version = \"0.8\", features = [\"postgres\",\"runtime-tokio\"] }}");
    println!();
    println!("  // Example axum handler:");
    println!("  async fn get_user(Path(id): Path<u64>) -> Json<User> {{");
    println!("      let user = db.get_user(id).await.unwrap();");
    println!("      Json(user)");
    println!("  }}");

    // ========================================================================
    // SECTION 7: CLI Tool Pattern
    // ========================================================================
    println!("\n═══ Section 7: CLI Pattern ═══\n");

    // Simple argument parser (in practice, use clap crate)
    #[derive(Debug)]
    struct CliArgs {
        verbose: bool,
        output: String,
        files: Vec<String>,
    }

    fn parse_args(args: &[String]) -> CliArgs {
        let mut verbose = false;
        let mut output = String::from("stdout");
        let mut files = Vec::new();

        let mut i = 1; // Skip program name
        while i < args.len() {
            match args[i].as_str() {
                "-v" | "--verbose" => verbose = true,
                "-o" | "--output" => {
                    i += 1;
                    if i < args.len() { output = args[i].clone(); }
                }
                arg if !arg.starts_with('-') => files.push(arg.to_string()),
                _ => eprintln!("Unknown arg: {}", args[i]),
            }
            i += 1;
        }

        CliArgs { verbose, output, files }
    }

    let test_args: Vec<String> = vec![
        "myapp".into(), "-v".into(), "--output".into(),
        "result.txt".into(), "input1.txt".into(), "input2.txt".into(),
    ];
    let parsed = parse_args(&test_args);
    println!("  Parsed CLI args: {:#?}", parsed);

    println!("\n✅ Lesson 18 Complete!");
    println!("   Next: Lesson 19 — Testing, Benchmarking & Performance");
}

// ============================================================================
// HTTP Server Handler
// ============================================================================
fn handle_http_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap_or(Ok(String::new())).unwrap_or_default();

    let (status_line, body) = match request_line.as_str() {
        "GET / HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            r#"{"message": "Welcome to Rust HTTP Server!", "status": "ok"}"#,
        ),
        line if line.starts_with("GET /api/") => (
            "HTTP/1.1 200 OK",
            r#"{"endpoint": "api", "data": "Hello from API!"}"#,
        ),
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            r#"{"error": "Not Found"}"#,
        ),
    };

    let response = format!(
        "{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status_line, body.len(), body
    );

    stream.write_all(response.as_bytes()).unwrap();
}
