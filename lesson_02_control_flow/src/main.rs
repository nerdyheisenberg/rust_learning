// ============================================================================
// LESSON 2: Control Flow, Functions & Expressions
// ============================================================================
// This lesson covers:
//   - if/else as expressions
//   - loop, while, for
//   - match expressions & pattern matching
//   - Functions and return values
//   - Closures (anonymous functions)
//   - Labeled loops and break with values
//   - Range expressions
//   - The never type (!)
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  LESSON 2: Control Flow, Functions & Expressions    ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: if/else as Expressions
    // ========================================================================
    println!("═══ Section 1: if/else as Expressions ═══\n");

    let number = 7;

    // Basic if/else (no parentheses needed, unlike C++!)
    if number > 5 {
        println!("{} is greater than 5", number);
    } else if number > 3 {
        println!("{} is greater than 3", number);
    } else {
        println!("{} is 3 or less", number);
    }

    // if as an EXPRESSION — this replaces C++'s ternary operator (? :)
    // Rust doesn't have a ternary operator because if IS an expression!
    let condition = true;
    let value = if condition { 5 } else { 10 };
    println!("Conditional value: {}", value);

    // Both branches MUST return the same type:
    let x = if number % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", number, x);

    // The block expression — a sequence of statements ending with an expression
    let y = {
        let a = 10;
        let b = 20;
        a + b    // No semicolon! This is the return value = 30
    };
    println!("Block expression result: {}", y);

    // CRITICAL: Adding a semicolon changes the return type to ()
    let z: () = {
        let _a = 10;
        let _b = 20;
        // a + b;  // With semicolon, this returns (), not 30
    };
    println!("Block with semicolon: {:?}", z);  // ()

    // ========================================================================
    // SECTION 2: Loops
    // ========================================================================
    println!("\n═══ Section 2: Loops ═══\n");

    // --- loop --- (infinite loop, like while(true) in C++)
    // Rust prefers `loop` over `while true` because it's semantically clearer
    // AND it can return values!
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // `break` with a value! Loop returns 20.
        }
    };
    println!("Loop returned: {}", result);

    // --- while --- (conditional loop)
    let mut countdown = 5;
    print!("While countdown: ");
    while countdown > 0 {
        print!("{} ", countdown);
        countdown -= 1;  // No -- operator in Rust!
    }
    println!("Liftoff!");

    // --- for --- (iterator-based, like range-based for in C++11)
    // This is the PREFERRED loop in Rust
    print!("For range: ");
    for i in 0..5 {        // Exclusive range [0, 5)
        print!("{} ", i);
    }
    println!();

    print!("For inclusive range: ");
    for i in 1..=5 {       // Inclusive range [1, 5]
        print!("{} ", i);
    }
    println!();

    // For loop with array
    let fruits = ["apple", "banana", "cherry", "date"];
    print!("Fruits: ");
    for fruit in fruits.iter() {  // .iter() borrows each element
        print!("{} ", fruit);
    }
    println!();

    // For loop with index (like enumerate in Python)
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  fruits[{}] = {}", index, fruit);
    }

    // For loop with reverse
    print!("Reverse: ");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();

    // For loop with step (using step_by)
    print!("Step by 2: ");
    for i in (0..10).step_by(2) {
        print!("{} ", i);
    }
    println!();

    // --- Labeled loops --- (for nested loop control)
    // In C++, breaking from nested loops requires goto or flags.
    // Rust has labeled loops!
    println!("\nLabeled loop example:");
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i + j == 6 {
                println!("  Breaking outer at i={}, j={}", i, j);
                break 'outer;  // Breaks the OUTER loop!
            }
        }
    }

    // Labeled continue
    println!("Labeled continue:");
    'outer2: for i in 0..3 {
        for j in 0..3 {
            if j == 1 {
                continue 'outer2;  // Skips to next iteration of OUTER loop
            }
            println!("  i={}, j={}", i, j);
        }
    }

    // ========================================================================
    // SECTION 3: match — The Superpower Pattern Matching
    // ========================================================================
    println!("\n═══ Section 3: match Expressions ═══\n");

    // Basic match (like switch in C++, but much more powerful)
    let number = 13;
    let description = match number {
        1 => "one",
        2 => "two",
        3..=12 => "three to twelve",   // Range pattern!
        13 => "thirteen — lucky!",
        _ => "something else",         // _ is the catch-all (like default:)
    };
    println!("{} is: {}", number, description);

    // match with multiple patterns (like multiple case labels)
    let day = 3;
    let day_type = match day {
        1 | 7 => "weekend",          // OR patterns with |
        2..=6 => "weekday",          // Range pattern
        _ => "invalid",
    };
    println!("Day {} is: {}", day, day_type);

    // match with tuple destructuring
    let point = (3, -5);
    let quadrant = match point {
        (0, 0) => "origin",
        (x, 0) if x > 0 => "positive x-axis",
        (0, y) if y > 0 => "positive y-axis",
        (x, y) if x > 0 && y > 0 => "first quadrant",
        (x, y) if x < 0 && y > 0 => "second quadrant",
        (x, y) if x < 0 && y < 0 => "third quadrant",
        (_, _) => "fourth quadrant",
    };
    println!("Point {:?} is in: {}", point, quadrant);

    // match with guards (if conditions on patterns)
    let temperature = 25;
    let feeling = match temperature {
        t if t < 0 => String::from("freezing"),
        t if t < 15 => format!("cold ({}°C)", t),
        t if t < 25 => format!("comfortable ({}°C)", t),
        t if t < 35 => format!("warm ({}°C)", t),
        t => format!("hot! ({}°C)", t),
    };
    println!("Temperature: {}", feeling);

    // match with binding (@)
    let age = 25;
    let category = match age {
        0 => "newborn",
        age @ 1..=12 => {
            println!("  (Child aged {})", age);
            "child"
        }
        age @ 13..=17 => {
            println!("  (Teen aged {})", age);
            "teenager"
        }
        age @ 18..=64 => {
            println!("  (Adult aged {})", age);
            "adult"
        }
        _ => "senior",
    };
    println!("Category: {}", category);

    // match is exhaustive — compiler ensures ALL cases are covered
    // Try removing the `_` arm and the compiler will error!

    // ========================================================================
    // SECTION 4: if let and while let — Concise Pattern Matching
    // ========================================================================
    println!("\n═══ Section 4: if let & while let ═══\n");

    // if let — syntactic sugar for match when you care about ONE pattern
    let optional_value: Option<i32> = Some(42);

    // Instead of:
    match optional_value {
        Some(val) => println!("Match: Got value {}", val),
        None => println!("Match: No value"),
    }

    // You can write:
    if let Some(val) = optional_value {
        println!("if let: Got value {}", val);
    } else {
        println!("if let: No value");
    }

    // while let — loop while pattern matches
    let mut stack = vec![1, 2, 3, 4, 5];
    print!("Stack pop: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("(empty)");

    // let else — (Rust 1.65+) — inverse of if let
    let value: Option<i32> = Some(42);
    let Some(inner) = value else {
        println!("This won't execute because value is Some");
        return;  // Must diverge: return, break, continue, or panic!
    };
    println!("let else: inner = {}", inner);

    // ========================================================================
    // SECTION 5: Functions
    // ========================================================================
    println!("\n═══ Section 5: Functions ═══\n");

    // Basic function call
    greet("Rohit");

    // Function with return value
    let sum = add(5, 3);
    println!("add(5, 3) = {}", sum);

    // Function with explicit return type
    let greeting = build_greeting("Rohit", 30);
    println!("{}", greeting);

    // Functions as expressions
    let factorial_5 = factorial(5);
    println!("5! = {}", factorial_5);

    // Functions demonstrating expression return (no return keyword)
    let is_even_result = is_even(42);
    println!("42 is even? {}", is_even_result);

    // Early return with `return` keyword
    let div_result = safe_divide(10.0, 3.0);
    println!("10.0 / 3.0 = {:?}", div_result);
    let div_zero = safe_divide(10.0, 0.0);
    println!("10.0 / 0.0 = {:?}", div_zero);

    // Recursive function
    let fib_10 = fibonacci(10);
    println!("fibonacci(10) = {}", fib_10);

    // ========================================================================
    // SECTION 6: Closures (Anonymous Functions)
    // ========================================================================
    println!("\n═══ Section 6: Closures ═══\n");

    // Basic closure syntax: |parameters| body
    let add_one = |x: i32| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // Multi-line closure with block
    let calculate = |x: i32, y: i32| {
        let sum = x + y;
        let product = x * y;
        (sum, product)  // Returns a tuple
    };
    let (s, p) = calculate(3, 4);
    println!("calculate(3, 4) = sum:{}, product:{}", s, p);

    // Closures capture environment (C++ captures are explicit, Rust is automatic)
    let base = 10;
    let add_base = |x| x + base;  // Captures `base` by reference automatically
    println!("add_base(5) = {} (base={})", add_base(5), base);
    // `base` is still usable because closure borrowed it immutably

    // Mutable capture
    let mut total = 0;
    let mut accumulate = |x: i32| {
        total += x;  // Captures `total` by mutable reference
        total
    };
    println!("accumulate(10) = {}", accumulate(10));
    println!("accumulate(20) = {}", accumulate(20));
    println!("accumulate(30) = {}", accumulate(30));
    // total is now 60, but can't use it while closure exists (mut borrow)
    drop(accumulate);  // Explicitly drop the closure
    println!("Final total: {}", total);

    // move closure — takes ownership of captured variables
    let name = String::from("Rohit");
    let greet_closure = move || {
        println!("Hello, {}!", name);  // name is MOVED into closure
    };
    greet_closure();
    // println!("{}", name);  // ❌ ERROR: name was moved into closure

    // Closures as function parameters
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);

    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    let product: i32 = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Product: {}", product);

    // Closures with higher-order functions
    println!("Applied: {}", apply_twice(5, |x| x * 2));
    println!("Applied: {}", apply_twice(3, |x| x + 10));

    // Returning closures from functions
    let doubler = make_multiplier(2);
    let tripler = make_multiplier(3);
    println!("doubler(5) = {}", doubler(5));
    println!("tripler(5) = {}", tripler(5));

    // ========================================================================
    // SECTION 7: Advanced Pattern Matching
    // ========================================================================
    println!("\n═══ Section 7: Advanced Patterns ═══\n");

    // Nested destructuring
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    println!("Nested destructure: {}, {}, {}, {}", a, b, c, d);

    // Ignoring values with _
    let (first, _, third) = (1, 2, 3);
    println!("Ignoring second: {}, {}", first, third);

    // Ignoring rest with ..
    let numbers = (1, 2, 3, 4, 5);
    let (first, .., last) = numbers;
    println!("First and last: {}, {}", first, last);

    // Ref patterns in match
    let reference = &42;
    match reference {
        &val => println!("Dereferenced: {}", val),
    }

    // Matching multiple conditions
    let pair = (true, 42);
    match pair {
        (true, n) if n > 0 => println!("Positive truth: {}", n),
        (true, _) => println!("Negative truth"),
        (false, _) => println!("Falsehood"),
    }

    // ========================================================================
    // SECTION 8: Loop Patterns
    // ========================================================================
    println!("\n═══ Section 8: Loop Patterns ═══\n");

    // Iterate with ownership (moves elements)
    let names = vec!["Alice", "Bob", "Charlie"];
    for name in &names {  // Borrow instead of consuming
        print!("{} ", name);
    }
    println!();
    // names is still usable because we borrowed
    println!("Names still here: {:?}", names);

    // Mutable iteration
    let mut scores = vec![85, 92, 78, 95, 88];
    // Add curve to all scores
    for score in scores.iter_mut() {
        *score = (*score + 5).min(100);  // Cap at 100
    }
    println!("Curved scores: {:?}", scores);

    // Loop with index, using chunks
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for (i, chunk) in data.chunks(3).enumerate() {
        println!("  Chunk {}: {:?}", i, chunk);
    }

    // Windows (sliding window)
    print!("Sliding windows of 3: ");
    for window in data.windows(3) {
        print!("{:?} ", window);
    }
    println!();

    println!("\n✅ Lesson 2 Complete! Run: cargo run");
    println!("   Next: Lesson 3 — Ownership (The Heart of Rust)");
}

// ============================================================================
// FUNCTION DEFINITIONS
// ============================================================================

// Basic function — no return value (returns () implicitly)
fn greet(name: &str) {
    // Note: parameter types MUST always be annotated (no type inference for params)
    println!("Hello, {}!", name);
}

// Function with return value
// The last expression WITHOUT semicolon is the return value
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon, no `return` — this IS the return value
    // Equivalent to: return a + b;
}

// Function with String return
fn build_greeting(name: &str, age: u32) -> String {
    format!("Hello, {}! You are {} years old.", name, age)
}

// Recursive function (like C++ recursive functions, identical concept)
fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => n * factorial(n - 1),
    }
}

// Function returning bool — expression return
fn is_even(n: i32) -> bool {
    n % 2 == 0  // Expression return — no `return` keyword needed
}

// Function with early return using `return` keyword
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None;  // Early return — need `return` keyword here
    }
    Some(a / b)  // Implicit return
}

// Recursive fibonacci
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Higher-order function — takes a closure as parameter
// `impl Fn(i32) -> i32` means "any type implementing the Fn trait"
fn apply_twice(x: i32, f: impl Fn(i32) -> i32) -> i32 {
    f(f(x))
}

// Function returning a closure — must use Box<dyn Fn> or impl Fn
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor  // `move` captures `factor` by value
}
