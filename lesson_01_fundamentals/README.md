# Lesson 1: Rust Philosophy, Setup & First Programs

## 🎯 Learning Objectives
By the end of this lesson, you will:
- Understand WHY Rust exists and what problems it solves
- Know the Rust compilation pipeline
- Master variables, types, mutability, and shadowing
- Write and run your first Rust programs

---

## 📚 Theory Deep-Dive

### 1.1 Why Rust Exists: The Billion-Dollar Problem

In C++, you've likely encountered these bugs:
- **Use-after-free**: Accessing memory after `delete`
- **Double-free**: Calling `delete` twice on the same pointer
- **Buffer overflow**: Writing past array boundaries
- **Data races**: Two threads accessing shared data without synchronization
- **Dangling pointers**: Pointers to deallocated memory
- **Null pointer dereference**: The "billion dollar mistake" (Tony Hoare)

**Microsoft** reported that ~70% of their CVEs are memory safety bugs.
**Google Chrome** reported similar numbers for Chromium.

Rust was created at Mozilla in 2010 (by Graydon Hoare) to solve this:

> **Rust's promise: Memory safety WITHOUT garbage collection, with ZERO runtime cost.**

```
┌─────────────────────────────────────────────────────────────────┐
│                    The Safety-Performance Spectrum               │
│                                                                  │
│  C/C++          Rust              Go/Java/C#         Python      │
│  ──────         ────              ──────────         ──────      │
│  Manual         Ownership +       Garbage            Garbage     │
│  Memory         Borrow Checker    Collector          Collector   │
│  Mgmt           (compile-time)    (runtime)          (runtime)   │
│                                                                  │
│  ⚡ Fastest     ⚡ Fastest        🐢 Slower          🐌 Slowest │
│  ❌ Unsafe      ✅ Safe           ✅ Safe            ✅ Safe     │
│  ❌ UB prone    ✅ No UB*         ✅ No UB           ✅ No UB   │
└─────────────────────────────────────────────────────────────────┘
                  * In safe Rust code
```

### 1.2 Rust vs C++: A Philosophical Comparison

| Aspect | C++ | Rust |
|--------|-----|------|
| Memory Safety | Manual (RAII helps, but not enforced) | Enforced by compiler (ownership system) |
| Null | `nullptr` exists | No null — uses `Option<T>` |
| Exceptions | `try/catch/throw` | No exceptions — uses `Result<T, E>` |
| Headers | `.h`/`.hpp` files needed | No header files — modules |
| Build System | CMake, Make, Bazel, etc. | Cargo (unified) |
| Package Manager | vcpkg, conan (fragmented) | crates.io (unified) |
| Undefined Behavior | Rampant (100+ sources) | None in safe code |
| Move Semantics | Opt-in via `std::move()` | Default behavior |
| Copy Semantics | Default (implicit copy) | Explicit (`Copy` trait) |
| Inheritance | Class hierarchies | No inheritance — composition via traits |

### 1.3 Zero-Cost Abstractions

This is Rust's core motto (borrowed from C++, but Rust actually delivers):

> **What you don't use, you don't pay for. What you do use, you couldn't hand-code any better.**

Example: Rust's iterators compile down to the SAME assembly as a hand-written loop:

```rust
// This high-level code:
let sum: i32 = (1..=100).filter(|x| x % 2 == 0).sum();

// Compiles to IDENTICAL assembly as:
let mut sum = 0i32;
let mut i = 1;
while i <= 100 {
    if i % 2 == 0 { sum += i; }
    i += 1;
}
```

### 1.4 The Rust Compilation Pipeline

```
                    Rust Compilation Pipeline
┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
│  Source   │──▶│   AST    │──▶│   HIR    │──▶│   MIR    │──▶│ LLVM IR  │
│  (.rs)   │   │ (Parse)  │   │(Desugar) │   │(Borrow   │   │(Optimize)│
│          │   │          │   │          │   │ Check)   │   │          │
└──────────┘   └──────────┘   └──────────┘   └──────────┘   └──────────┘
                                                                  │
                                                                  ▼
                                                            ┌──────────┐
                                                            │ Machine  │
                                                            │  Code    │
                                                            │ (Binary) │
                                                            └──────────┘

1. Source → AST:     Lexing and parsing (syntax check)
2. AST → HIR:       High-level IR — desugars syntax (for loops → iterators)
3. HIR → MIR:       Mid-level IR — where BORROW CHECKING happens!
4. MIR → LLVM IR:   Handed to LLVM backend for optimization
5. LLVM IR → Binary: LLVM produces optimized machine code
```

**Key insight**: The borrow checker operates on MIR, which is why it can reason
about lifetimes and moves at a granular level.

### 1.5 Cargo — The Build System and Package Manager

Unlike C++ (CMake, Make, Bazel, Meson...), Rust has ONE official build tool: **Cargo**.

```
Cargo.toml — The Manifest File (like CMakeLists.txt + package.json combined)
┌─────────────────────────────────────────────────┐
│ [package]                                        │
│ name = "my_project"                              │
│ version = "0.1.0"                                │
│ edition = "2021"    ← Rust edition (like C++17)  │
│                                                  │
│ [dependencies]                                   │
│ serde = "1.0"       ← External crate dependency  │
│ tokio = { version = "1", features = ["full"] }   │
│                                                  │
│ [dev-dependencies]                               │
│ criterion = "0.5"   ← Test-only dependencies     │
└─────────────────────────────────────────────────┘
```

Key cargo commands:
```bash
cargo new project_name   # Create new project
cargo build              # Compile (debug mode)
cargo build --release    # Compile with optimizations
cargo run                # Build + Run
cargo test               # Run tests
cargo check              # Fast type-check without codegen
cargo clippy             # Lint (like clang-tidy)
cargo fmt                # Format code (like clang-format)
cargo doc --open         # Generate and open documentation
```

### 1.6 Rust Edition System

Rust editions (2015, 2018, 2021, 2024) are like C++ standards (C++11, C++14, C++17, C++20):
- They allow **backward-incompatible syntax changes**
- But ALL editions can interoperate (crate A on edition 2021 can use crate B on 2018)
- Set in `Cargo.toml` with `edition = "2021"`

### 1.7 The Type System — Scalar Types

Rust is **statically typed** with **type inference** (like C++ `auto`, but much more powerful):

```
┌─────────────────────────────────────────────────────┐
│                   Scalar Types                       │
├──────────────┬──────────────────────────────────────┤
│ Integers     │ i8, i16, i32, i64, i128, isize       │
│ (signed)     │ u8, u16, u32, u64, u128, usize       │
├──────────────┼──────────────────────────────────────┤
│ Floats       │ f32, f64 (default)                    │
├──────────────┼──────────────────────────────────────┤
│ Boolean      │ bool (true/false) — 1 byte            │
├──────────────┼──────────────────────────────────────┤
│ Character    │ char — 4 bytes (Unicode scalar value)│
└──────────────┴──────────────────────────────────────┘
```

**Important differences from C++:**
- `char` is 4 bytes (Unicode), NOT 1 byte like in C++
- `isize`/`usize` are pointer-sized (like `intptr_t`/`size_t` in C++)
- Integer overflow is CHECKED in debug mode (panics!), wraps in release
- No implicit type conversions — `let x: i64 = 42i32;` won't compile!

### 1.8 Variables, Mutability & Shadowing

**The biggest surprise for C++ developers**: Variables are **immutable by default**.

```rust
let x = 5;       // Immutable — like "const int x = 5;" in C++
let mut y = 5;   // Mutable — like "int y = 5;" in C++
```

**Shadowing** — a concept foreign to C++ (but powerful):
```rust
let x = 5;           // x is i32, value 5
let x = x + 1;       // New variable "x" shadows old one, value 6
let x = "hello";     // Same name, DIFFERENT TYPE! This is valid in Rust!
```

In C++, this would be illegal — you can't redeclare a variable with a different type.
In Rust, shadowing creates a BRAND NEW variable that happens to have the same name.

### 1.9 Constants and Statics

```rust
const MAX_POINTS: u32 = 100_000;  // Compile-time constant (like constexpr in C++)
static GREETING: &str = "Hello";   // Global static variable (like static const in C++)
static mut COUNTER: i32 = 0;       // Mutable static — unsafe to access!
```

**Key rules:**
- `const`: Inlined everywhere it's used (no memory address), MUST have type annotation
- `static`: Has a fixed memory address, lives for entire program (`'static` lifetime)
- `static mut`: Exists but accessing it requires `unsafe` (because thread safety)

---

## 🔑 Key Concepts Summary

1. **Rust = Safety + Performance** — No GC, no UB in safe code
2. **Variables are immutable by default** — Opt-in mutability with `mut`
3. **No implicit type conversions** — Everything must be explicit
4. **Shadowing** lets you reuse names with different types
5. **Cargo** is your one-stop build + test + dependency tool
6. **Editions** let the language evolve without breaking old code

---

## ⚠️ Common Mistakes

1. **Forgetting `mut`**: Coming from C++ where everything is mutable
2. **Expecting implicit conversions**: `let x: f64 = 42;` works, but `let x: f64 = 42i32;` doesn't
3. **Integer overflow**: Will panic in debug mode (unlike C++ which wraps silently)
4. **Thinking `char` is 1 byte**: It's 4 bytes in Rust (Unicode)
5. **Using semicolons everywhere**: Adding `;` to the last expression changes return value

---

## 🎤 Interview Questions

1. **Q: What problem does Rust solve that C++ doesn't?**
   A: Rust guarantees memory safety at compile time through its ownership and borrowing system, eliminating entire classes of bugs (use-after-free, data races, null dereference) without runtime overhead.

2. **Q: What are zero-cost abstractions?**
   A: High-level language features that compile to code as efficient as hand-written low-level equivalents. For example, Rust iterators compile to the same assembly as manual loops.

3. **Q: How does Rust handle null?**
   A: Rust has no null. Instead, it uses `Option<T>` — an enum that is either `Some(value)` or `None`. The compiler forces you to handle both cases.

4. **Q: What's the difference between `let` and `let mut`?**
   A: `let` creates an immutable binding (the default). `let mut` creates a mutable binding. This is the opposite of C++ where mutability is the default.

5. **Q: Explain shadowing in Rust.**
   A: Shadowing lets you declare a new variable with the same name, potentially with a different type. It creates a new binding rather than mutating the original.
