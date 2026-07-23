// ============================================================================
// LESSON 19: Testing, Benchmarking & Performance
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 19: Testing, Benchmarking & Performance        ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Testing Overview
    // ========================================================================
    println!("═══ Section 1: Testing Framework ═══\n");

    println!("  Rust has built-in testing support:");
    println!("  - Unit tests:       #[test] in same file");
    println!("  - Integration tests: tests/ directory");
    println!("  - Doc tests:        in /// documentation");
    println!("  - Run: cargo test");
    println!("  - Run specific: cargo test test_name");
    println!("  - Show output: cargo test -- --nocapture\n");

    // Actually test our functions
    let lib = MathLib;
    println!("  factorial(5) = {}", lib.factorial(5));
    println!("  fibonacci(10) = {}", lib.fibonacci(10));
    println!("  is_prime(17) = {}", lib.is_prime(17));
    println!("  gcd(48, 18) = {}", lib.gcd(48, 18));

    // ========================================================================
    // SECTION 2: Performance Measurement
    // ========================================================================
    println!("\n═══ Section 2: Performance Measurement ═══\n");

    use std::time::Instant;

    // Manual benchmarking
    let iterations = 1_000_000;

    // Benchmark: Vec push
    let start = Instant::now();
    let mut v = Vec::new();
    for i in 0..iterations {
        v.push(i);
    }
    let elapsed = start.elapsed();
    println!("  Vec push {} items: {:?}", iterations, elapsed);

    // Benchmark: Vec with capacity
    let start = Instant::now();
    let mut v = Vec::with_capacity(iterations);
    for i in 0..iterations {
        v.push(i);
    }
    let elapsed = start.elapsed();
    println!("  Vec with_capacity push: {:?} (pre-allocated)", elapsed);

    // Benchmark: Iterator vs loop
    let data: Vec<i64> = (0..iterations as i64).collect();

    let start = Instant::now();
    let mut sum = 0i64;
    for &x in &data {
        if x % 2 == 0 { sum += x * x; }
    }
    let loop_time = start.elapsed();
    println!("  Loop sum of even squares: {} in {:?}", sum, loop_time);

    let start = Instant::now();
    let iter_sum: i64 = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    let iter_time = start.elapsed();
    println!("  Iterator sum: {} in {:?}", iter_sum, iter_time);
    assert_eq!(sum, iter_sum);

    // Benchmark: HashMap vs BTreeMap
    use std::collections::{HashMap, BTreeMap};

    let start = Instant::now();
    let mut hmap = HashMap::new();
    for i in 0..100_000 {
        hmap.insert(i, i * 2);
    }
    println!("  HashMap 100k inserts: {:?}", start.elapsed());

    let start = Instant::now();
    let mut bmap = BTreeMap::new();
    for i in 0..100_000 {
        bmap.insert(i, i * 2);
    }
    println!("  BTreeMap 100k inserts: {:?}", start.elapsed());

    // ========================================================================
    // SECTION 3: Optimization Techniques
    // ========================================================================
    println!("\n═══ Section 3: Optimization Techniques ═══\n");

    // 1. Use iterators (zero-cost abstraction)
    println!("  1. Iterators compile to same assembly as hand-written loops");

    // 2. Avoid unnecessary allocations
    println!("  2. Preallocate with Vec::with_capacity()");

    // 3. Use &str instead of String when possible
    println!("  3. Use &str for function params (avoids allocation)");

    // 4. Profile before optimizing
    println!("  4. Use cargo flamegraph / perf for profiling");

    // 5. Compiler hints
    println!("  5. #[inline] for small frequently-called functions");
    println!("     #[cold] for unlikely error paths");
    println!("     cargo build --release for optimized builds\n");

    // 6. Use appropriate data structures
    println!("  Data structure selection guide:");
    println!("  ┌──────────────┬──────────────┬───────────────┐");
    println!("  │ Operation    │ Vec          │ HashMap       │");
    println!("  ├──────────────┼──────────────┼───────────────┤");
    println!("  │ Push/Pop     │ O(1) amort.  │ N/A           │");
    println!("  │ Insert       │ O(n)         │ O(1) amort.   │");
    println!("  │ Lookup       │ O(n)         │ O(1) amort.   │");
    println!("  │ Sorted iter  │ O(n log n)   │ O(n log n)    │");
    println!("  │ Memory       │ Contiguous   │ Sparse        │");
    println!("  └──────────────┴──────────────┴───────────────┘");

    // ========================================================================
    // SECTION 4: Cargo Commands for Testing/Perf
    // ========================================================================
    println!("\n═══ Section 4: Cargo Commands Reference ═══\n");

    println!("  cargo test                    # Run all tests");
    println!("  cargo test test_name          # Run specific test");
    println!("  cargo test -- --nocapture     # Show println! output");
    println!("  cargo test -- --test-threads=1 # Run tests sequentially");
    println!("  cargo bench                   # Run benchmarks");
    println!("  cargo clippy                  # Lint for common mistakes");
    println!("  cargo build --release         # Optimized build");
    println!("  cargo run --release           # Run optimized");

    println!("\n  For production benchmarking, add to Cargo.toml:");
    println!("  [dev-dependencies]");
    println!("  criterion = {{ version = \"0.5\", features = [\"html_reports\"] }}");
    println!("  ");
    println!("  [[bench]]");
    println!("  name = \"my_benchmarks\"");
    println!("  harness = false");

    println!("\n✅ Lesson 19 Complete!");
    println!("   Next: Lesson 20 — Production Rust (Final!)");
}

// ============================================================================
// Library Code to Test
// ============================================================================
struct MathLib;

impl MathLib {
    fn factorial(&self, n: u64) -> u64 {
        (1..=n).product()
    }

    fn fibonacci(&self, n: u32) -> u64 {
        if n <= 1 { return n as u64; }
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }

    fn is_prime(&self, n: u64) -> bool {
        if n < 2 { return false; }
        if n == 2 || n == 3 { return true; }
        if n % 2 == 0 || n % 3 == 0 { return false; }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 { return false; }
            i += 6;
        }
        true
    }

    fn gcd(&self, mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

// ============================================================================
// UNIT TESTS — Comprehensive examples
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Basic test
    #[test]
    fn test_factorial() {
        let lib = MathLib;
        assert_eq!(lib.factorial(0), 1);
        assert_eq!(lib.factorial(1), 1);
        assert_eq!(lib.factorial(5), 120);
        assert_eq!(lib.factorial(10), 3628800);
    }

    // Test with assert_ne!
    #[test]
    fn test_fibonacci() {
        let lib = MathLib;
        assert_eq!(lib.fibonacci(0), 0);
        assert_eq!(lib.fibonacci(1), 1);
        assert_eq!(lib.fibonacci(10), 55);
        assert_ne!(lib.fibonacci(10), 56);
    }

    // Test primes
    #[test]
    fn test_primes() {
        let lib = MathLib;
        assert!(!lib.is_prime(0));
        assert!(!lib.is_prime(1));
        assert!(lib.is_prime(2));
        assert!(lib.is_prime(17));
        assert!(!lib.is_prime(15));
        assert!(lib.is_prime(997));
    }

    // Test GCD
    #[test]
    fn test_gcd() {
        let lib = MathLib;
        assert_eq!(lib.gcd(48, 18), 6);
        assert_eq!(lib.gcd(100, 75), 25);
        assert_eq!(lib.gcd(7, 13), 1);  // Coprime
    }

    // Test that should panic
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _ = v[10];
    }

    // Test returning Result
    #[test]
    fn test_parse() -> Result<(), Box<dyn std::error::Error>> {
        let n: i32 = "42".parse()?;
        assert_eq!(n, 42);
        Ok(())
    }

    // Ignored test (slow test)
    #[test]
    #[ignore]
    fn test_slow_computation() {
        let lib = MathLib;
        // Test large fibonacci
        assert_eq!(lib.fibonacci(50), 12586269025);
    }

    // Parameterized tests with helper
    #[test]
    fn test_fibonacci_sequence() {
        let lib = MathLib;
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (i, &expected_val) in expected.iter().enumerate() {
            assert_eq!(
                lib.fibonacci(i as u32), expected_val,
                "fibonacci({}) should be {}", i, expected_val
            );
        }
    }

    // Test with setup/teardown using Drop
    struct TestContext {
        data: Vec<i32>,
    }

    impl TestContext {
        fn new() -> Self {
            // Setup
            TestContext { data: vec![1, 2, 3, 4, 5] }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            // Teardown (cleanup)
        }
    }

    #[test]
    fn test_with_context() {
        let ctx = TestContext::new();
        assert_eq!(ctx.data.len(), 5);
        assert_eq!(ctx.data.iter().sum::<i32>(), 15);
    }
}
