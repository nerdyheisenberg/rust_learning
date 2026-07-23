# Lesson 2: Control Flow, Functions & Expressions

## 🎯 Learning Objectives
- Understand Rust as an **expression-oriented** language
- Master all control flow constructs
- Write functions with complex return types
- Use closures (anonymous functions)

---

## 📚 Theory Deep-Dive

### 2.1 Statements vs Expressions — The Fundamental Difference

This is the MOST IMPORTANT concept to understand about Rust's design.

In C++, `if` is a **statement** — it does something but doesn't produce a value:
```cpp
// C++: if is a statement
int x;
if (condition) { x = 5; } else { x = 10; }  // Must assign inside
// Or use ternary: int x = condition ? 5 : 10;
```

In Rust, `if` is an **expression** — it produces a value:
```rust
// Rust: if is an expression
let x = if condition { 5 } else { 10 };  // The if itself IS the value
```

**Rule: In Rust, almost everything is an expression that returns a value.**

```
┌─────────────────────────────────────────────────────────────┐
│               Statements vs Expressions                      │
├──────────────────────┬──────────────────────────────────────┤
│ STATEMENT            │ EXPRESSION                           │
│ (does something,     │ (evaluates to a value)               │
│  returns nothing)    │                                      │
├──────────────────────┼──────────────────────────────────────┤
│ let x = 5;           │ 5                                    │
│ fn foo() { }         │ if cond { a } else { b }             │
│ use std::io;         │ match x { ... }                      │
│                      │ { let y = 5; y + 1 }  ← block expr  │
│                      │ loop { break 42; }    ← loop expr   │
└──────────────────────┴──────────────────────────────────────┘
```

**Critical rule about semicolons:**
- Adding `;` to an expression turns it into a statement (discards the value)
- The LAST expression in a block (without `;`) IS the return value of that block

```rust
let x = {
    let y = 5;
    y + 1       // No semicolon! This is the return value of the block → 6
};
// x = 6

let x = {
    let y = 5;
    y + 1;      // Semicolon! This discards the value, block returns ()
};
// x = () — the unit type
```

### 2.2 Pattern Matching: Rust's Superpower

`match` is Rust's version of C++ `switch`, but infinitely more powerful:
- It's an **expression** (returns a value)
- It's **exhaustive** (compiler forces you to handle ALL cases)
- It can **destructure** complex types
- It works with ranges, guards, bindings, and nested patterns

```
┌─────────────────────────────────────────────────────────┐
│              match vs switch comparison                   │
├─────────────────────┬───────────────────────────────────┤
│ C++ switch           │ Rust match                       │
├─────────────────────┼───────────────────────────────────┤
│ Only integers/enums  │ ANY type with patterns           │
│ Fall-through default │ No fall-through                  │
│ Not exhaustive       │ MUST be exhaustive               │
│ Statement only       │ Expression (returns value)       │
│ No destructuring     │ Full destructuring support       │
│ No range patterns    │ Range patterns (1..=5)           │
│ No guard conditions  │ Guard conditions (if cond)       │
└─────────────────────┴───────────────────────────────────┘
```

### 2.3 Closures — Anonymous Functions

Rust closures capture variables from their environment (like C++ lambdas):

```
┌────────────────────────────────────────────────────────────┐
│              C++ Lambda vs Rust Closure                      │
├────────────────────────┬───────────────────────────────────┤
│ C++ Lambda              │ Rust Closure                     │
├────────────────────────┼───────────────────────────────────┤
│ [&x](int y) { x + y } │ |y| x + y                        │
│ [=](int y) { x + y }  │ move |y| x + y                   │
│ Must specify captures  │ Compiler infers capture mode      │
│ [&] or [=] for all     │ Captures by ref or move auto     │
└────────────────────────┴───────────────────────────────────┘
```

Closures implement one of three traits:
- `Fn`: Borrows immutably from environment (`&self`)
- `FnMut`: Borrows mutably from environment (`&mut self`)
- `FnOnce`: Takes ownership from environment (`self`)

### 2.4 The Never Type (`!`)

Rust has a special type `!` called the "never type" — it represents computations 
that never complete:

```rust
fn infinite_loop() -> ! {    // This function NEVER returns
    loop { }
}

fn crash() -> ! {
    panic!("This crashes!");  // panic! has type !
}
```

The `!` type can coerce into any other type, making it useful in match arms
and if branches where one path never returns.

### 2.5 Ranges

Rust has built-in range syntax:
```
0..5     → [0, 1, 2, 3, 4]        (exclusive end — like Python)
0..=5    → [0, 1, 2, 3, 4, 5]     (inclusive end)
..5      → up to 5 (exclusive)    (for slicing)
5..      → from 5 onwards         (for slicing)
..       → full range             (for slicing)
```

---

## 🔑 Key Concepts Summary

1. **Everything is an expression** — `if`, `match`, blocks all return values
2. **Semicolons matter** — last expression without `;` is the return value
3. **`match` is exhaustive** — compiler ensures all cases are handled
4. **Closures** capture environment automatically (Rust figures out how)
5. **No `++`/`--` operators** — use `+= 1` instead
6. **`loop` is preferred** over `while true` — it's semantically clearer

---

## ⚠️ Common Mistakes

1. **Adding `;` to the last expression**: Changes the return type to `()`
2. **Non-exhaustive match**: Forgetting `_ => ...` for catch-all
3. **Mismatched types in if/else branches**: Both branches must return same type
4. **Forgetting `mut` for closures that mutate captured variables**
5. **Using `return` everywhere**: Idiomatic Rust omits `return` for the last expression

---

## 🎤 Interview Questions

1. **Q: What's the difference between a statement and an expression in Rust?**
   A: An expression evaluates to a value; a statement performs an action but doesn't return a value. `let x = 5;` is a statement containing the expression `5`. Unlike C++, `if`, `match`, and blocks are expressions in Rust.

2. **Q: What does removing the semicolon from the last line of a function do?**
   A: It makes that expression the return value of the function. `fn foo() -> i32 { 42 }` returns 42 because there's no semicolon.

3. **Q: Why is `match` preferable to a chain of `if/else if`?**
   A: `match` is exhaustive (compiler guarantees all cases are covered), supports pattern destructuring, works as an expression, and is generally more readable for multi-way branching.

4. **Q: What are the three closure traits and when is each used?**
   A: `Fn` (borrows immutably), `FnMut` (borrows mutably), `FnOnce` (takes ownership). The compiler picks the most permissive trait that works.
