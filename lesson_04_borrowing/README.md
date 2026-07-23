# Lesson 4: References & Borrowing

## 🎯 Learning Objectives
- Master immutable and mutable references
- Understand the borrowing rules and WHY they exist
- Learn lifetime annotations
- Understand interior mutability (`Cell`, `RefCell`)
- Use `Rc<T>` for shared ownership

---

## 📚 Theory Deep-Dive

### 4.1 The Borrowing Rules

Instead of moving ownership, you can BORROW a value:

```
┌─────────────────────────────────────────────────────────────────┐
│                   THE TWO BORROWING RULES                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  At any given time, you can have EITHER:                         │
│                                                                  │
│  • ONE mutable reference (&mut T)                                │
│                        — OR —                                    │
│  • ANY number of immutable references (&T)                       │
│                                                                  │
│  Never both at the same time!                                    │
│                                                                  │
│  AND: References must ALWAYS be valid (no dangling!)             │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

This is the "Readers-Writers Lock" pattern enforced at COMPILE TIME:
```
Multiple readers  (&T)   = OK  (like shared_lock in C++)
One writer       (&mut T) = OK  (like unique_lock in C++)
Readers + Writer          = ❌  COMPILE ERROR
Multiple writers          = ❌  COMPILE ERROR
```

### 4.2 WHY These Rules Exist: Preventing Data Races

A data race occurs when:
1. Two or more pointers access the same data at the same time
2. At least one is writing
3. There's no synchronization

Rust's borrowing rules make data races **impossible at compile time**:

```
┌────────────────────────────────────────────────────────────────┐
│              Data Race Prevention                               │
├─────────────────────┬──────────────────────────────────────────┤
│ Scenario            │ Rust's Response                          │
├─────────────────────┼──────────────────────────────────────────┤
│ Read + Read         │ ✅ Allowed (many &T)                    │
│ Read + Write        │ ❌ Compile error (can't mix & and &mut) │
│ Write + Write       │ ❌ Compile error (only one &mut)        │
│ Sequential R then W │ ✅ Allowed (NLL sees non-overlap)       │
└─────────────────────┴──────────────────────────────────────────┘
```

### 4.3 Lifetimes — The Hidden Dimension

Every reference in Rust has a **lifetime** — the scope for which it's valid.
Usually, the compiler infers lifetimes (like it infers types).

```rust
fn longest(x: &str, y: &str) -> &str { ... }  // ❌ Which input's lifetime?

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }  // ✅ Explicit!
```

The `'a` syntax means: "The returned reference lives at least as long as
the shorter of the two input lifetimes."

```
Lifetime annotations DON'T change how long references live.
They DESCRIBE the relationships between lifetimes of references.
They help the compiler verify that references are always valid.
```

### 4.4 Lifetime Elision Rules

The compiler applies three rules to infer lifetimes automatically:

```
Rule 1: Each input reference parameter gets its own lifetime
   fn foo(x: &str, y: &str) → fn foo<'a, 'b>(x: &'a str, y: &'b str)

Rule 2: If there's ONE input lifetime, output gets the same
   fn foo(x: &str) -> &str → fn foo<'a>(x: &'a str) -> &'a str

Rule 3: If one param is &self or &mut self, output gets self's lifetime
   fn foo(&self, x: &str) -> &str → fn foo<'a>(&'a self, x: &str) -> &'a str
```

### 4.5 Interior Mutability Pattern

Sometimes you need to mutate data behind an immutable reference.
Rust provides "escape hatches" that move borrow checking to RUNTIME:

```
┌──────────┬───────────────────────────────────────────────────┐
│ Type     │ Description                                       │
├──────────┼───────────────────────────────────────────────────┤
│ Cell<T>  │ Get/set values through &self (Copy types only)    │
│          │ No runtime overhead, no borrowing at all           │
├──────────┼───────────────────────────────────────────────────┤
│ RefCell  │ Runtime borrow checking with borrow()/borrow_mut()│
│ <T>     │ Panics on violation instead of compile error       │
├──────────┼───────────────────────────────────────────────────┤
│ Rc<T>   │ Reference counting (like shared_ptr in C++)        │
│          │ Multiple owners, single-threaded only              │
├──────────┼───────────────────────────────────────────────────┤
│ Rc<     │ Shared ownership + interior mutability             │
│ RefCell │ The go-to combo for shared mutable data            │
│ <T>>    │                                                    │
└──────────┴───────────────────────────────────────────────────┘
```

---

## 🔑 Key Concepts Summary

1. **`&T`** — immutable reference (shared, read-only)
2. **`&mut T`** — mutable reference (exclusive, read-write)
3. **One `&mut` XOR many `&`** — never both simultaneously
4. **Lifetimes** describe how long references are valid
5. **NLL** (Non-Lexical Lifetimes) — lifetimes end at last use, not scope end
6. **Interior mutability** moves borrow checking from compile-time to runtime

---

## 🎤 Interview Questions

1. **Q: What are the borrowing rules in Rust?**
   A: At any time, you can have either one mutable reference OR any number of immutable references, but not both. All references must be valid (no dangling).

2. **Q: What is a lifetime in Rust?**
   A: A lifetime is the scope for which a reference is valid. Lifetime annotations (`'a`) don't change how long references live — they describe relationships between reference lifetimes so the compiler can verify safety.

3. **Q: What is interior mutability?**
   A: A design pattern that allows mutating data through an immutable reference by moving borrow checking from compile time to runtime. Implemented via `Cell<T>`, `RefCell<T>`, and `Mutex<T>`.

4. **Q: What's the difference between `Rc<T>` and `Arc<T>`?**
   A: Both provide reference-counted shared ownership. `Rc` is for single-threaded use (cheaper), `Arc` is for multi-threaded use (uses atomic reference counting).
