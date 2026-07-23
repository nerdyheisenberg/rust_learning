# Lesson 5: Structs, Enums & Pattern Matching

## 🎯 Learning Objectives
- Master all struct types (named, tuple, unit)
- Understand Algebraic Data Types (ADTs)
- Write powerful enums with data variants
- Use `Option<T>` and `Result<T, E>` idiomatically
- Apply the `?` operator for error propagation
- Implement the Builder pattern

---

## 📚 Theory Deep-Dive

### 5.1 Algebraic Data Types (ADTs)

Rust's type system is built on two fundamental building blocks:

```
┌────────────────────────────────────────────────────────────────┐
│              Algebraic Data Types (ADTs)                        │
├──────────────────┬─────────────────────────────────────────────┤
│ PRODUCT Types     │ SUMS Types                                │
│ (Structs/Tuples)  │ (Enums)                                   │
├──────────────────┼─────────────────────────────────────────────┤
│ A AND B AND C     │ A OR B OR C                                │
│                   │                                            │
│ struct Point {    │ enum Shape {                               │
│   x: f64, ← AND  │   Circle(f64),      ← OR                  │
│   y: f64, ← AND  │   Rectangle(f64,f64),← OR                 │
│ }                 │   Triangle(f64,f64,f64),                   │
│                   │ }                                          │
│ Has ALL fields    │ Has exactly ONE variant at a time          │
├──────────────────┼─────────────────────────────────────────────┤
│ C++: struct/class │ C++: std::variant (C++17)                  │
│ Go:  struct       │ Go:  NO equivalent (uses interfaces)      │
└──────────────────┴─────────────────────────────────────────────┘
```

### 5.2 Enum Memory Layout

Rust enums use a **tag + union** internally:
```
enum IpAddr {
    V4(u8, u8, u8, u8),  // 4 bytes of data
    V6(String),            // 24 bytes of data (ptr+len+cap)
}

Memory layout:
┌──────────┬──────────────────────────────┐
│ tag (8B) │ data (max variant size: 24B) │
└──────────┴──────────────────────────────┘
Total: 32 bytes (tag + largest variant, with padding)

Niche optimization: Option<&T> is same size as &T!
Because the null pointer value represents None.
```

### 5.3 Option<T> — Replacing Null

```
enum Option<T> {
    Some(T),   // Contains a value
    None,      // No value (Rust's replacement for null)
}
```

**Why this is better than null:**
- The compiler FORCES you to handle None
- You can never get a NullPointerException
- The intent is explicit in the type signature

### 5.4 Result<T, E> — Replacing Exceptions

```
enum Result<T, E> {
    Ok(T),    // Success with value
    Err(E),   // Error with error value
}
```

**The `?` operator:**
```rust
fn read_file(path: &str) -> Result<String, io::Error> {
    let content = std::fs::read_to_string(path)?;  // If Err, return early
    Ok(content.to_uppercase())
}

// The ? operator desugars to:
// match std::fs::read_to_string(path) {
//     Ok(content) => content,
//     Err(e) => return Err(e.into()),
// }
```

---

## 🎤 Interview Questions

1. **Q: What's the difference between struct and enum in Rust?**
   A: A struct is a product type (has ALL fields), an enum is a sum type (is ONE of its variants). Enums can carry different data in each variant.

2. **Q: How does Rust replace null?**
   A: With `Option<T>`, an enum that is either `Some(value)` or `None`. The compiler forces exhaustive handling, preventing null pointer errors.

3. **Q: What does the `?` operator do?**
   A: It propagates errors. If the expression is `Err(e)`, it returns early from the function with `Err(e.into())`. If `Ok(v)`, it unwraps to `v`.

4. **Q: Why can `Option<&T>` be the same size as `&T`?**
   A: Niche optimization. Since references can never be null, the compiler uses the null pointer value to represent `None`, eliminating the tag byte.
