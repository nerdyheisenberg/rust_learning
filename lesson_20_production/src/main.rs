// ============================================================================
// LESSON 20: Production Rust — Real-World Mastery (Capstone)
// ============================================================================
// This final lesson brings everything together into a production-quality
// mini application: a concurrent key-value store with logging, configuration,
// error handling, and clean architecture.
// ============================================================================

use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

// ============================================================================
// MODULE 1: Error Handling (Production-Grade)
// ============================================================================
#[derive(Debug)]
enum KvError {
    KeyNotFound(String),
    InvalidCommand(String),
    IoError(std::io::Error),
    LockPoisoned,
}

impl fmt::Display for KvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KvError::KeyNotFound(key) => write!(f, "Key not found: '{}'", key),
            KvError::InvalidCommand(cmd) => write!(f, "Invalid command: '{}'", cmd),
            KvError::IoError(e) => write!(f, "IO error: {}", e),
            KvError::LockPoisoned => write!(f, "Lock poisoned"),
        }
    }
}

impl std::error::Error for KvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            KvError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for KvError {
    fn from(e: std::io::Error) -> Self { KvError::IoError(e) }
}

type Result<T> = std::result::Result<T, KvError>;

// ============================================================================
// MODULE 2: Logging (Structured)
// ============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

struct Logger {
    min_level: LogLevel,
    start_time: Instant,
}

impl Logger {
    fn new(min_level: LogLevel) -> Self {
        Logger { min_level, start_time: Instant::now() }
    }

    fn log(&self, level: LogLevel, message: &str) {
        if level >= self.min_level {
            let elapsed = self.start_time.elapsed();
            println!("[{:>7.3}s] [{}] {}", elapsed.as_secs_f64(), level, message);
        }
    }

    fn info(&self, msg: &str) { self.log(LogLevel::Info, msg); }
    fn warn(&self, msg: &str) { self.log(LogLevel::Warn, msg); }
    fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
    fn debug(&self, msg: &str) { self.log(LogLevel::Debug, msg); }
}

// ============================================================================
// MODULE 3: Configuration
// ============================================================================
#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    max_connections: usize,
    log_level: LogLevel,
    default_ttl_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: 18090,
            max_connections: 100,
            log_level: LogLevel::Info,
            default_ttl_secs: 3600,
        }
    }
}

impl Config {
    fn from_env() -> Self {
        let mut config = Config::default();
        if let Ok(port) = std::env::var("KV_PORT") {
            if let Ok(p) = port.parse() { config.port = p; }
        }
        if let Ok(level) = std::env::var("KV_LOG_LEVEL") {
            config.log_level = match level.to_uppercase().as_str() {
                "DEBUG" => LogLevel::Debug,
                "WARN" => LogLevel::Warn,
                "ERROR" => LogLevel::Error,
                _ => LogLevel::Info,
            };
        }
        config
    }

    fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

// ============================================================================
// MODULE 4: Key-Value Store (Thread-Safe)
// ============================================================================
#[derive(Debug, Clone)]
struct Entry {
    value: String,
    created_at: SystemTime,
    access_count: u64,
}

struct KvStore {
    data: RwLock<HashMap<String, Entry>>,
    stats: RwLock<StoreStats>,
}

#[derive(Debug, Default)]
struct StoreStats {
    total_gets: u64,
    total_sets: u64,
    total_deletes: u64,
    cache_hits: u64,
    cache_misses: u64,
}

impl KvStore {
    fn new() -> Self {
        KvStore {
            data: RwLock::new(HashMap::new()),
            stats: RwLock::new(StoreStats::default()),
        }
    }

    fn get(&self, key: &str) -> Result<String> {
        let mut data = self.data.write().map_err(|_| KvError::LockPoisoned)?;
        let mut stats = self.stats.write().map_err(|_| KvError::LockPoisoned)?;
        stats.total_gets += 1;

        match data.get_mut(key) {
            Some(entry) => {
                stats.cache_hits += 1;
                entry.access_count += 1;
                Ok(entry.value.clone())
            }
            None => {
                stats.cache_misses += 1;
                Err(KvError::KeyNotFound(key.to_string()))
            }
        }
    }

    fn set(&self, key: String, value: String) -> Result<()> {
        let mut data = self.data.write().map_err(|_| KvError::LockPoisoned)?;
        let mut stats = self.stats.write().map_err(|_| KvError::LockPoisoned)?;
        stats.total_sets += 1;

        data.insert(key, Entry {
            value,
            created_at: SystemTime::now(),
            access_count: 0,
        });
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<String> {
        let mut data = self.data.write().map_err(|_| KvError::LockPoisoned)?;
        let mut stats = self.stats.write().map_err(|_| KvError::LockPoisoned)?;
        stats.total_deletes += 1;

        data.remove(key)
            .map(|entry| entry.value)
            .ok_or_else(|| KvError::KeyNotFound(key.to_string()))
    }

    fn keys(&self) -> Result<Vec<String>> {
        let data = self.data.read().map_err(|_| KvError::LockPoisoned)?;
        Ok(data.keys().cloned().collect())
    }

    fn len(&self) -> Result<usize> {
        let data = self.data.read().map_err(|_| KvError::LockPoisoned)?;
        Ok(data.len())
    }

    fn stats_summary(&self) -> Result<String> {
        let stats = self.stats.read().map_err(|_| KvError::LockPoisoned)?;
        let data = self.data.read().map_err(|_| KvError::LockPoisoned)?;
        Ok(format!(
            "Keys: {}, Gets: {}, Sets: {}, Deletes: {}, Hits: {}, Misses: {}",
            data.len(), stats.total_gets, stats.total_sets,
            stats.total_deletes, stats.cache_hits, stats.cache_misses
        ))
    }
}

// ============================================================================
// MODULE 5: Command Protocol
// ============================================================================
#[derive(Debug)]
enum Command {
    Get(String),
    Set(String, String),
    Delete(String),
    Keys,
    Stats,
    Ping,
    Quit,
}

fn parse_command(input: &str) -> Result<Command> {
    let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();
    match parts.first().map(|s| s.to_uppercase()).as_deref() {
        Some("GET") => {
            let key = parts.get(1).ok_or_else(|| KvError::InvalidCommand("GET requires a key".into()))?;
            Ok(Command::Get(key.to_string()))
        }
        Some("SET") => {
            let key = parts.get(1).ok_or_else(|| KvError::InvalidCommand("SET requires key and value".into()))?;
            let value = parts.get(2).ok_or_else(|| KvError::InvalidCommand("SET requires a value".into()))?;
            Ok(Command::Set(key.to_string(), value.to_string()))
        }
        Some("DEL") | Some("DELETE") => {
            let key = parts.get(1).ok_or_else(|| KvError::InvalidCommand("DEL requires a key".into()))?;
            Ok(Command::Delete(key.to_string()))
        }
        Some("KEYS") => Ok(Command::Keys),
        Some("STATS") => Ok(Command::Stats),
        Some("PING") => Ok(Command::Ping),
        Some("QUIT") | Some("EXIT") => Ok(Command::Quit),
        _ => Err(KvError::InvalidCommand(input.trim().to_string())),
    }
}

// ============================================================================
// MODULE 6: Server
// ============================================================================
fn handle_client(stream: TcpStream, store: Arc<KvStore>, logger: Arc<Logger>) {
    let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_default();
    logger.info(&format!("Client connected: {}", peer));

    let reader = BufReader::new(&stream);
    let mut writer = stream.try_clone().unwrap();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line.trim().is_empty() { continue; }

        logger.debug(&format!("← {} | {}", peer, line.trim()));

        let response = match parse_command(&line) {
            Ok(Command::Get(key)) => {
                match store.get(&key) {
                    Ok(val) => format!("+{}\n", val),
                    Err(e) => format!("-{}\n", e),
                }
            }
            Ok(Command::Set(key, value)) => {
                match store.set(key, value) {
                    Ok(()) => "+OK\n".to_string(),
                    Err(e) => format!("-{}\n", e),
                }
            }
            Ok(Command::Delete(key)) => {
                match store.delete(&key) {
                    Ok(val) => format!("+Deleted: {}\n", val),
                    Err(e) => format!("-{}\n", e),
                }
            }
            Ok(Command::Keys) => {
                match store.keys() {
                    Ok(keys) => format!("+{}\n", keys.join(", ")),
                    Err(e) => format!("-{}\n", e),
                }
            }
            Ok(Command::Stats) => {
                match store.stats_summary() {
                    Ok(s) => format!("+{}\n", s),
                    Err(e) => format!("-{}\n", e),
                }
            }
            Ok(Command::Ping) => "+PONG\n".to_string(),
            Ok(Command::Quit) => {
                let _ = writer.write_all(b"+BYE\n");
                logger.info(&format!("Client disconnected: {}", peer));
                return;
            }
            Err(e) => format!("-{}\n", e),
        };

        if writer.write_all(response.as_bytes()).is_err() { break; }
    }

    logger.info(&format!("Client connection closed: {}", peer));
}

// ============================================================================
// MAIN — Bringing It All Together
// ============================================================================
fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 20: Production Rust — Capstone Project         ║");
    println!("║  A Concurrent Key-Value Store Server                   ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // --- Configuration ---
    let config = Config::from_env();
    let logger = Arc::new(Logger::new(config.log_level));

    logger.info("Starting KV Store Server");
    logger.info(&format!("Config: {:?}", config));

    // --- Initialize Store ---
    let store = Arc::new(KvStore::new());

    // Pre-populate with demo data
    store.set("name".into(), "Rohit".into()).unwrap();
    store.set("language".into(), "Rust".into()).unwrap();
    store.set("lesson".into(), "20".into()).unwrap();
    logger.info(&format!("Pre-populated {} keys", store.len().unwrap()));

    // --- Start Server in Background ---
    let server_store = Arc::clone(&store);
    let server_logger = Arc::clone(&logger);
    let addr = config.address();

    let server_handle = thread::spawn(move || {
        let listener = match TcpListener::bind(&addr) {
            Ok(l) => l,
            Err(e) => {
                server_logger.error(&format!("Failed to bind: {}", e));
                return;
            }
        };

        server_logger.info(&format!("Listening on {}", addr));

        // Handle 5 connections then stop (demo purposes)
        for stream in listener.incoming().take(5) {
            match stream {
                Ok(stream) => {
                    let store = Arc::clone(&server_store);
                    let logger = Arc::clone(&server_logger);
                    thread::spawn(move || handle_client(stream, store, logger));
                }
                Err(e) => server_logger.error(&format!("Accept error: {}", e)),
            }
        }
        server_logger.info("Server stopped");
    });

    thread::sleep(Duration::from_millis(200));

    // --- Demo Client ---
    logger.info("Running demo client...");

    let commands = vec![
        "PING",
        "SET greeting Hello World",
        "GET greeting",
        "GET name",
        "GET language",
        "KEYS",
        "DEL greeting",
        "GET greeting",
        "STATS",
        "QUIT",
    ];

    if let Ok(stream) = TcpStream::connect(config.address()) {
        let reader = BufReader::new(&stream);
        let mut writer = stream.try_clone().unwrap();
        let mut lines = reader.lines();

        for cmd in &commands {
            println!("\n  → {}", cmd);
            writeln!(writer, "{}", cmd).unwrap();
            if let Some(Ok(response)) = lines.next() {
                println!("  ← {}", response);
            }
            if *cmd == "QUIT" { break; }
        }
    }

    // --- Summary ---
    println!("\n{}", "=".repeat(60));
    println!("  FINAL STORE STATE");
    println!("{}", "=".repeat(60));
    println!("  {}", store.stats_summary().unwrap());
    println!("  Keys: {:?}", store.keys().unwrap());

    let _ = server_handle.join();

    // ========================================================================
    // PRODUCTION CHECKLIST
    // ========================================================================
    println!("\n{}", "=".repeat(60));
    println!("  PRODUCTION RUST CHECKLIST");
    println!("{}", "=".repeat(60));
    println!("  ✅ Structured error handling (custom Error type)");
    println!("  ✅ Logging with levels (in production: use tracing crate)");
    println!("  ✅ Configuration from environment (in production: config crate)");
    println!("  ✅ Thread-safe shared state (Arc<RwLock>)");
    println!("  ✅ Clean module architecture");
    println!("  ✅ Protocol parsing with proper errors");
    println!("  ✅ Graceful client handling");
    println!("  ✅ Unit tests (run: cargo test)");
    println!("  ");
    println!("  For production deployment:");
    println!("  • cargo build --release");
    println!("  • cargo clippy -- -D warnings");
    println!("  • cargo fmt --check");
    println!("  • cargo audit (security vulnerabilities)");
    println!("  • Docker: FROM rust:1.94 as builder");
    println!("  • CI: GitHub Actions with rust-toolchain.toml");

    println!("\n🎉 CONGRATULATIONS! You've completed all 20 Rust lessons!");
    println!("   From fundamentals to production-grade systems.");
    println!("   Keep building and exploring the Rust ecosystem! 🦀");
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_store_basic() {
        let store = KvStore::new();
        store.set("key1".into(), "value1".into()).unwrap();
        assert_eq!(store.get("key1").unwrap(), "value1");
    }

    #[test]
    fn test_kv_store_overwrite() {
        let store = KvStore::new();
        store.set("key".into(), "v1".into()).unwrap();
        store.set("key".into(), "v2".into()).unwrap();
        assert_eq!(store.get("key").unwrap(), "v2");
    }

    #[test]
    fn test_kv_store_delete() {
        let store = KvStore::new();
        store.set("key".into(), "value".into()).unwrap();
        assert_eq!(store.delete("key").unwrap(), "value");
        assert!(store.get("key").is_err());
    }

    #[test]
    fn test_kv_store_not_found() {
        let store = KvStore::new();
        assert!(store.get("nonexistent").is_err());
    }

    #[test]
    fn test_parse_command() {
        assert!(matches!(parse_command("PING").unwrap(), Command::Ping));
        assert!(matches!(parse_command("GET key").unwrap(), Command::Get(_)));
        assert!(matches!(parse_command("SET k v").unwrap(), Command::Set(_, _)));
        assert!(matches!(parse_command("DEL k").unwrap(), Command::Delete(_)));
        assert!(matches!(parse_command("KEYS").unwrap(), Command::Keys));
        assert!(matches!(parse_command("QUIT").unwrap(), Command::Quit));
    }

    #[test]
    fn test_parse_invalid_command() {
        assert!(parse_command("INVALID").is_err());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.port, 18090);
        assert_eq!(config.host, "127.0.0.1");
    }

    #[test]
    fn test_concurrent_access() {
        let store = Arc::new(KvStore::new());
        let mut handles = vec![];

        for i in 0..10 {
            let store = Arc::clone(&store);
            handles.push(thread::spawn(move || {
                store.set(format!("key_{}", i), format!("val_{}", i)).unwrap();
            }));
        }

        for h in handles { h.join().unwrap(); }
        assert_eq!(store.len().unwrap(), 10);
    }
}
