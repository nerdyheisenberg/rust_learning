// ============================================================================
// LESSON 1: Rust Fundamentals — First Programs
// ============================================================================
// This lesson covers:
//   - Hello World & println! macro
//   - Variables, mutability, shadowing
//   - All scalar types (integers, floats, bool, char)
//   - Compound types (tuples, arrays, slices)
//   - Constants and statics
//   - Type conversions (casting)
//   - Basic I/O
//   - String types introduction
// ============================================================================

// In Rust, the entry point is always `fn main()` — like int main() in C++.
// No return value needed (implicitly returns () — the "unit" type).
fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║   LESSON 1: Rust Fundamentals — First Programs  ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Hello World & The println! Macro
    // ========================================================================
    println!("═══ Section 1: Hello World & println! ═══\n");

    // println! is a MACRO (note the `!`), not a function.
    // In C++, you'd use std::cout or printf. Rust uses macros for formatted output.
    // Why a macro? Because it can accept variable number of arguments with
    // compile-time format string checking — like a type-safe printf.

    println!("Hello, World!");

    // Format arguments use {} for display formatting (like %d/%s in C printf)
    let name = "Rohit";
    let age = 30;
    println!("Name: {}, Age: {}", name, age);

    // Named arguments
    println!("{name} is {age} years old", name = "Rohit", age = 30);

    // Debug formatting with {:?} — works for any type that implements Debug trait
    // (Similar to operator<< overloading in C++, but automatic with #[derive(Debug)])
    println!("Debug output: {:?}", (1, 2, 3));

    // Pretty-print debug with {:#?}
    println!("Pretty debug: {:#?}", vec![1, 2, 3, 4, 5]);

    // Formatting options (like printf format specifiers)
    println!("Padding:    '{:>10}'", "right");     // Right-align in 10 chars
    println!("Padding:    '{:<10}'", "left");       // Left-align in 10 chars
    println!("Padding:    '{:^10}'", "center");     // Center in 10 chars
    println!("Padding:    '{:0>5}'", 42);           // Zero-pad to 5 digits
    println!("Binary:     {:b}", 42);               // Binary representation
    println!("Octal:      {:o}", 42);               // Octal
    println!("Hex:        {:x}", 255);              // Hex lowercase
    println!("Hex upper:  {:X}", 255);              // Hex uppercase
    println!("Scientific: {:e}", 1000000.0_f64);    // Scientific notation
    println!("Precision:  {:.3}", 3.14159265);      // 3 decimal places

    // eprintln! prints to stderr (like std::cerr in C++)
    eprintln!("This goes to stderr!");

    // ========================================================================
    // SECTION 2: Variables and Mutability
    // ========================================================================
    println!("\n═══ Section 2: Variables & Mutability ═══\n");

    // In Rust, variables are IMMUTABLE by default.
    // This is the opposite of C++ where everything is mutable unless marked `const`.
    let x = 5;  // Immutable — equivalent to `const int x = 5;` in C++
    println!("x = {}", x);

    // x = 6;  // ❌ ERROR: cannot assign twice to immutable variable `x`

    // To make a variable mutable, use `mut`:
    let mut y = 5;  // Mutable — equivalent to `int y = 5;` in C++
    println!("y before: {}", y);
    y = 10;  // ✅ OK because y is declared as `mut`
    println!("y after:  {}", y);

    // SHADOWING — a unique Rust feature
    // You can declare a NEW variable with the same name:
    let z = 5;
    println!("z = {} (i32)", z);

    let z = z + 1;  // New variable z, shadows the old one
    println!("z = {} (i32, shadowed)", z);

    let z = "now I'm a string!";  // DIFFERENT TYPE! This is valid in Rust!
    println!("z = {} (str, shadowed with different type)", z);

    // Shadowing vs mut: The key difference
    // - `mut`: Same type, same variable, different values over time
    // - Shadowing: New variable, can be different type, old one is gone

    // A practical use case for shadowing:
    let spaces = "   ";          // &str type
    let spaces = spaces.len();   // usize type — the name makes sense in both contexts
    println!("Number of spaces: {}", spaces);

    // ========================================================================
    // SECTION 3: Scalar Types
    // ========================================================================
    println!("\n═══ Section 3: Scalar Types ═══\n");

    // --- Integers ---
    // Signed integers: i8, i16, i32, i64, i128, isize
    // Unsigned integers: u8, u16, u32, u64, u128, usize
    let a: i8 = -128;              // Min value for i8 (same as C++)
    let b: i8 = 127;               // Max value for i8
    let c: u8 = 255;               // Max value for u8
    let d: i32 = 42;               // Default integer type (like int in C++)
    let e: i64 = 9_223_372_036_854_775_807;  // Underscores for readability!
    let f: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // 128-bit!
    let g: usize = 42;             // Pointer-sized unsigned (like size_t in C++)
    let h: isize = -42;            // Pointer-sized signed (like ptrdiff_t in C++)

    println!("i8  range: {} to {}", a, b);
    println!("u8  max:   {}", c);
    println!("i32:       {}", d);
    println!("i64:       {}", e);
    println!("i128:      {}", f);
    println!("usize:     {} (pointer-sized: {} bytes)", g, std::mem::size_of::<usize>());
    println!("isize:     {} (pointer-sized: {} bytes)", h, std::mem::size_of::<isize>());

    // Integer literals can have type suffixes:
    let decimal = 98_222i64;       // Type suffix
    let hex = 0xFFu8;              // Hexadecimal
    let octal = 0o77u8;            // Octal
    let binary = 0b1111_0000u8;    // Binary
    let byte = b'A';               // Byte literal (u8 only) — ASCII value of 'A'
    println!("Literals: decimal={}, hex={}, octal={}, binary={}, byte={}",
             decimal, hex, octal, binary, byte);

    // --- Floats ---
    // f32 (32-bit, like float in C++) and f64 (64-bit, like double in C++)
    let float32: f32 = 3.14;       // Single precision
    let float64 = 2.71828;         // f64 is the DEFAULT (unlike C++ where float is common)
    let float_inf = f64::INFINITY;
    let float_nan = f64::NAN;

    println!("f32: {}", float32);
    println!("f64: {}", float64);
    println!("Infinity: {}", float_inf);
    println!("NaN: {} (is NaN? {})", float_nan, float_nan.is_nan());

    // NaN comparison — same as C++, NaN != NaN
    println!("NaN == NaN? {}", float_nan == float_nan);  // false!

    // --- Boolean ---
    let is_active: bool = true;    // 1 byte (same as C++)
    let is_greater = 10 > 5;       // Type inference to bool
    println!("is_active: {}, is_greater: {}", is_active, is_greater);
    println!("bool size: {} byte", std::mem::size_of::<bool>());

    // --- Character ---
    // IMPORTANT: In Rust, `char` is 4 BYTES (Unicode scalar value)
    // In C++, `char` is 1 byte (ASCII only). Rust's char is more like C++ wchar_t.
    let letter = 'A';
    let emoji = '🦀';           // Yes, emojis are valid chars!
    let chinese = '中';
    let heart = '❤';
    println!("letter: {} (size: {} bytes)", letter, std::mem::size_of::<char>());
    println!("emoji:  {} (size: {} bytes)", emoji, std::mem::size_of::<char>());
    println!("chinese:{} (size: {} bytes)", chinese, std::mem::size_of::<char>());
    println!("heart:  {} (size: {} bytes)", heart, std::mem::size_of::<char>());

    // ========================================================================
    // SECTION 4: Compound Types
    // ========================================================================
    println!("\n═══ Section 4: Compound Types ═══\n");

    // --- Tuples ---
    // Fixed-size, heterogeneous collection (like std::tuple in C++)
    let tuple: (i32, f64, char) = (42, 6.28, '🦀');

    // Access by index (0-based):
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    // Destructuring (like C++17 structured bindings):
    let (x, y, z) = tuple;
    println!("Destructured: x={}, y={}, z={}", x, y, z);

    // Unit tuple — () — like void in C++
    let unit: () = ();
    println!("Unit value: {:?} (size: {} bytes)", unit, std::mem::size_of::<()>());

    // --- Arrays ---
    // Fixed-size, homogeneous, STACK-allocated (like std::array in C++)
    let arr: [i32; 5] = [1, 2, 3, 4, 5];  // [type; size]
    println!("Array: {:?}", arr);
    println!("First: {}, Last: {}", arr[0], arr[arr.len() - 1]);
    println!("Length: {}", arr.len());

    // Initialize with same value:
    let zeros = [0i32; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    println!("Zeros: {:?}", zeros);

    // IMPORTANT: Array bounds are CHECKED at runtime!
    // In C++, accessing arr[10] is undefined behavior (buffer overflow).
    // In Rust, it PANICS (crashes safely) — no silent memory corruption.
    // let crash = arr[10];  // Would panic: "index out of bounds"

    // --- Slices ---
    // A VIEW into a contiguous sequence (like std::span in C++20)
    // They don't own the data — they're just a (pointer, length) pair.
    let slice: &[i32] = &arr[1..4];  // Elements at index 1, 2, 3
    println!("Slice of arr[1..4]: {:?}", slice);

    let slice_from: &[i32] = &arr[2..];   // From index 2 to end
    let slice_to: &[i32] = &arr[..3];     // From start to index 2
    let slice_full: &[i32] = &arr[..];    // Full slice
    println!("From 2:    {:?}", slice_from);
    println!("To 3:      {:?}", slice_to);
    println!("Full:      {:?}", slice_full);

    // Slice is a "fat pointer" — it stores both address AND length
    println!("Slice size: {} bytes (pointer + length)",
             std::mem::size_of::<&[i32]>());

    // ========================================================================
    // SECTION 5: Constants and Statics
    // ========================================================================
    println!("\n═══ Section 5: Constants & Statics ═══\n");

    // const: Compile-time constant, inlined at every use (like constexpr in C++)
    const MAX_LEVEL: u32 = 100;
    const PI: f64 = 3.14159265358979323846;

    // static: Global variable with fixed memory address (like static const in C++)
    static APP_NAME: &str = "Rust Learning";

    println!("MAX_LEVEL: {}", MAX_LEVEL);
    println!("PI: {}", PI);
    println!("APP_NAME: {}", APP_NAME);

    // Difference: const is inlined (no address), static has a fixed address
    // const can be used in const contexts (array sizes, etc.)
    let arr_from_const = [0u8; MAX_LEVEL as usize]; // const in array size
    println!("Array of size MAX_LEVEL: {} elements", arr_from_const.len());

    // ========================================================================
    // SECTION 6: Type Casting (as keyword)
    // ========================================================================
    println!("\n═══ Section 6: Type Casting ═══\n");

    // Rust does NOT allow implicit type conversions (no implicit narrowing!)
    // You must use explicit `as` keyword:
    let big: i64 = 1000;
    let small: i32 = big as i32;     // Explicit cast
    let float_val: f64 = big as f64;  // int to float
    let int_val: i32 = 3.99_f64 as i32;  // Truncates! (not rounds) = 3

    println!("i64 {} → i32 {}", big, small);
    println!("i64 {} → f64 {}", big, float_val);
    println!("f64 3.99 → i32 {} (truncated!)", int_val);

    // Safe conversions with From/Into traits (preferred over `as`):
    let small_val: i32 = 42;
    let big_val: i64 = i64::from(small_val);  // Guaranteed safe conversion
    let big_val2: i64 = small_val.into();      // Same thing, different syntax
    println!("From/Into: i32 {} → i64 {} / {}", small_val, big_val, big_val2);

    // Dangerous cast: what happens with overflow?
    let overflow: u8 = 256u16 as u8;  // Wraps around! 256 % 256 = 0
    println!("256u16 as u8 = {} (wraps!)", overflow);

    // ========================================================================
    // SECTION 7: String Types Introduction
    // ========================================================================
    println!("\n═══ Section 7: String Types ═══\n");

    // Rust has TWO main string types:
    // 1. String — Owned, heap-allocated, growable (like std::string in C++)
    // 2. &str   — Borrowed string slice (like std::string_view in C++20)

    // String — owned, mutable, heap-allocated
    let mut owned = String::from("Hello");
    owned.push_str(", World!");
    owned.push('!');
    println!("Owned String: {}", owned);
    println!("  Length: {} bytes", owned.len());
    println!("  Capacity: {} bytes", owned.capacity());

    // &str — string slice, immutable reference to string data
    let slice: &str = "Hello, World!";  // String literal — stored in binary
    println!("String slice: {}", slice);
    println!("  Length: {} bytes", slice.len());

    // Converting between them:
    let s: String = slice.to_string();   // &str → String (allocates)
    let s2: String = String::from(slice); // Same thing
    let back: &str = &s;                   // String → &str (just borrows)
    println!("Conversions: '{}' → '{}' → '{}'", slice, s2, back);

    // String formatting
    let formatted = format!("{} is {} years old", "Rohit", 30);
    println!("Formatted: {}", formatted);

    // IMPORTANT: Strings in Rust are UTF-8!
    let emoji_str = String::from("Hello 🦀 World 🌍");
    println!("Emoji string: {}", emoji_str);
    println!("  Byte length: {}", emoji_str.len());        // Bytes, not chars!
    println!("  Char count: {}", emoji_str.chars().count()); // Actual char count

    // ========================================================================
    // SECTION 8: Basic I/O
    // ========================================================================
    println!("\n═══ Section 8: Basic I/O ═══\n");

    // Reading from stdin (commented out so program runs non-interactively):
    // use std::io;
    // let mut input = String::new();
    // println!("Enter your name:");
    // io::stdin().read_line(&mut input).expect("Failed to read line");
    // let input = input.trim(); // Remove trailing newline
    // println!("Hello, {}!", input);

    // Parsing strings to numbers:
    let num_str = "42";
    let num: i32 = num_str.parse().expect("Not a number!");
    println!("Parsed '{}' → {}", num_str, num);

    // Turbofish syntax for parse (explicit type):
    let num2 = "3.14".parse::<f64>().expect("Not a float!");
    println!("Parsed '3.14' → {}", num2);

    // Safe parsing with match:
    let result = "not_a_number".parse::<i32>();
    match result {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }

    // ========================================================================
    // SECTION 9: Operators
    // ========================================================================
    println!("\n═══ Section 9: Operators ═══\n");

    // Arithmetic (same as C++)
    let a = 10;
    let b = 3;
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {} (integer division)", a, b, a / b);
    println!("{} % {} = {} (remainder)", a, b, a % b);

    // No ++ or -- operators in Rust! Use += 1 instead.
    let mut counter = 0;
    counter += 1;  // Instead of counter++
    println!("Counter: {}", counter);

    // Logical operators (same as C++)
    println!("true && false = {}", true && false);
    println!("true || false = {}", true || false);
    println!("!true = {}", !true);

    // Bitwise operators (same as C++)
    println!("0b1010 & 0b1100 = {:04b}", 0b1010u8 & 0b1100u8);
    println!("0b1010 | 0b1100 = {:04b}", 0b1010u8 | 0b1100u8);
    println!("0b1010 ^ 0b1100 = {:04b}", 0b1010u8 ^ 0b1100u8);
    println!("0b1010 << 2 = {:08b}", 0b1010u8 << 2);
    println!("0b1010 >> 1 = {:04b}", 0b1010u8 >> 1);

    // Comparison operators (same as C++)
    println!("5 == 5: {}", 5 == 5);
    println!("5 != 3: {}", 5 != 3);
    println!("5 > 3:  {}", 5 > 3);
    println!("5 < 3:  {}", 5 < 3);
    println!("5 >= 5: {}", 5 >= 5);
    println!("5 <= 3: {}", 5 <= 3);

    // ========================================================================
    // SECTION 10: Memory Layout Visualization
    // ========================================================================
    println!("\n═══ Section 10: Memory Layout (sizes) ═══\n");

    // Let's see how Rust lays out types in memory:
    println!("Type sizes:");
    println!("  bool:   {} byte", std::mem::size_of::<bool>());
    println!("  u8:     {} byte", std::mem::size_of::<u8>());
    println!("  i16:    {} bytes", std::mem::size_of::<i16>());
    println!("  i32:    {} bytes", std::mem::size_of::<i32>());
    println!("  i64:    {} bytes", std::mem::size_of::<i64>());
    println!("  i128:   {} bytes", std::mem::size_of::<i128>());
    println!("  f32:    {} bytes", std::mem::size_of::<f32>());
    println!("  f64:    {} bytes", std::mem::size_of::<f64>());
    println!("  char:   {} bytes", std::mem::size_of::<char>());
    println!("  usize:  {} bytes", std::mem::size_of::<usize>());
    println!("  &str:   {} bytes (ptr + len)", std::mem::size_of::<&str>());
    println!("  String: {} bytes (ptr + len + cap)", std::mem::size_of::<String>());
    println!("  ():     {} bytes (zero-sized!)", std::mem::size_of::<()>());
    println!("  [i32;5]:{} bytes", std::mem::size_of::<[i32; 5]>());

    println!("\n✅ Lesson 1 Complete! Run: cargo run");
    println!("   Next: Lesson 2 — Control Flow, Functions & Expressions");
}
