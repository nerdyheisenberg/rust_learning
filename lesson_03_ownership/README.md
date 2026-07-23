# Lesson 3: Ownership — The Heart of Rust

## 🎯 Learning Objectives
- Understand the THREE ownership rules
- Master move semantics (default in Rust vs opt-in in C++)
- Distinguish `Copy` vs `Clone`
- Understand scope-based resource management
- Trace ownership through function calls

---

## 📚 Theory Deep-Dive

### 3.1 The Three Ownership Rules

Every value in Rust has exactly these properties:

```
┌─────────────────────────────────────────────────────────────────┐
│                  THE THREE OWNERSHIP RULES                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. Each value has exactly ONE owner (variable)                  │
│                                                                  │
│  2. There can only be ONE owner at a time                        │
│                                                                  │
│  3. When the owner goes out of scope, the value is DROPPED       │
│     (destructor runs, memory freed)                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

This is fundamentally different from C++:
- In C++: Multiple variables can point to the same memory (raw pointers)
- In C++: You must manually call `delete` or rely on RAII (smart pointers)
- In Rust: The compiler ENFORCES these rules at compile time

### 3.2 Stack vs Heap — Memory Layout

```
┌─────────────────────────────────────────────────────┐
│                    MEMORY LAYOUT                     │
├────────────────────┬────────────────────────────────┤
│    STACK            │    HEAP                        │
├────────────────────┼────────────────────────────────┤
│ • Fixed size        │ • Dynamic size                 │
│ • LIFO order        │ • Random access                │
│ • Very fast (ns)    │ • Slower (allocator overhead)  │
│ • Auto-freed on     │ • Must be explicitly freed     │
│   scope exit        │   (Rust does via Drop)         │
│                     │                                │
│ Types:              │ Types:                         │
│ • i32, f64, bool    │ • String (heap buffer)         │
│ • [i32; 5] (arrays) │ • Vec<T> (heap buffer)         │
│ • (i32, f64) tuples │ • Box<T> (heap pointer)        │
│ • &str (ptr+len)    │ • HashMap<K,V>                 │
│ • Struct (if small) │ • Any Box-allocated data       │
└────────────────────┴────────────────────────────────┘
```

Understanding where data lives is CRUCIAL for understanding ownership:

```
let x: i32 = 42;          // Stack: just 4 bytes on the stack

let s: String = String::from("hello");
// Stack: { ptr: 0x..., len: 5, cap: 5 }  ← 24 bytes on stack
// Heap:  [h, e, l, l, o]                 ← 5 bytes on heap

┌─── Stack ────┐     ┌─── Heap ───┐
│  ptr ────────────▶  │ h e l l o  │
│  len: 5      │     └────────────┘
│  cap: 5      │
└──────────────┘
```

### 3.3 Move Semantics — The Default in Rust

In C++, assignments COPY by default:
```cpp
std::string s1 = "hello";
std::string s2 = s1;         // COPIES the string — s1 and s2 are independent
std::cout << s1;              // ✅ Fine — s1 is still valid
```

In Rust, assignments MOVE by default (for heap types):
```rust
let s1 = String::from("hello");
let s2 = s1;                 // MOVES the string — s1 is now INVALID!
// println!("{}", s1);       // ❌ COMPILE ERROR: s1 was moved
println!("{}", s2);           // ✅ Fine — s2 is the new owner
```

Why? Because Rust prevents the "double free" problem:
```
AFTER MOVE: s1 → (invalidated)     s2 → heap["hello"]

If both s1 and s2 owned the same heap data, when they go out of scope,
BOTH would try to free the heap memory → DOUBLE FREE → undefined behavior!

Rust prevents this by making s1 invalid after the move.
```

### 3.4 Copy vs Clone

**Copy** — bitwise copy, automatic, for simple stack-only types:
```
Types that implement Copy (stack-only, cheap to copy):
• All integers (i32, u64, etc.)
• All floats (f32, f64)
• bool
• char
• Tuples of Copy types: (i32, f64)
• Fixed-size arrays of Copy types: [i32; 5]
• References (&T) — but NOT &mut T
```

**Clone** — explicit deep copy, can be expensive:
```
Types that implement Clone (may involve heap allocation):
• String — clones the heap buffer
• Vec<T> — clones the heap buffer and all elements
• Any type that derives Clone
```

```rust
// Copy types — assignment copies implicitly
let x = 42;
let y = x;           // COPY — x is still valid
println!("{} {}", x, y);  // Both work!

// Non-Copy types — assignment moves
let s1 = String::from("hello");
let s2 = s1;          // MOVE — s1 is invalidated
// println!("{}", s1); // ❌ ERROR

// Explicit clone — deep copy
let s3 = String::from("hello");
let s4 = s3.clone();  // CLONE — explicit deep copy
println!("{} {}", s3, s4);  // Both work!
```

### 3.5 Ownership Through Function Calls

```
Passing a value to a function MOVES it (same as assignment):

fn take_string(s: String) { }     // Takes ownership of s

let name = String::from("Rohit");
take_string(name);                 // name is MOVED into the function
// println!("{}", name);           // ❌ ERROR: name was moved

This is like passing a unique_ptr by value in C++:
void take(std::unique_ptr<std::string> s);  // Takes ownership
```

Return values transfer ownership OUT of functions:
```
fn give_string() -> String {
    let s = String::from("hello");
    s  // Ownership transferred to caller
}

let received = give_string();  // received now owns the String
```

### 3.6 The Drop Trait — Rust's Destructor

When a value goes out of scope, Rust calls its `Drop` trait implementation:

```rust
impl Drop for MyType {
    fn drop(&mut self) {
        // Cleanup code — like C++ destructor
    }
}
```

- Called automatically when the owner goes out of scope
- Cannot be called manually (use `std::mem::drop()` to force early drop)
- Runs in REVERSE order of creation (same as C++ destructors in a scope)

### 3.7 Comparison with C++ Smart Pointers

```
┌────────────────────────────────────────────────────────────────┐
│              Rust Ownership vs C++ Smart Pointers               │
├────────────────────┬───────────────────────────────────────────┤
│ Rust               │ C++ Equivalent                            │
├────────────────────┼───────────────────────────────────────────┤
│ let x = String::   │ auto x = std::make_unique<std::string>   │
│   from("hello");   │   ("hello");                              │
│ let y = x;         │ auto y = std::move(x);                   │
│ // x is invalid    │ // x is in "moved-from" state            │
│                    │                                           │
│ let z = x.clone(); │ auto z = std::make_unique<std::string>   │
│                    │   (*x); // deep copy                     │
│                    │                                           │
│ Rc<T>              │ std::shared_ptr<T>                       │
│ Arc<T>             │ std::shared_ptr<T> (thread-safe)         │
│ Box<T>             │ std::unique_ptr<T>                       │
│ &T                 │ const T* (but guaranteed non-null)       │
│ &mut T             │ T* (but guaranteed non-null + exclusive) │
└────────────────────┴───────────────────────────────────────────┘
```

The KEY difference: In C++, using a moved-from object is **undefined behavior** 
at runtime. In Rust, it's a **compile error**. Rust catches it BEFORE running.

---

## 🔑 Key Concepts Summary

1. **One owner per value** — ownership is linear
2. **Assignment = move** (for heap types) — unlike C++ where assignment = copy
3. **Copy types** live entirely on the stack and are automatically copied
4. **Clone** is explicit deep copy for heap types
5. **Functions take ownership** of their arguments (unless borrowing)
6. **Drop** runs automatically when owner goes out of scope

---

## ⚠️ Common Mistakes

1. **Using a value after moving it** — This is the #1 Rust beginner error
2. **Thinking all types move** — Copy types (integers, etc.) are automatically copied
3. **Cloning everything** — It works but defeats the performance purpose
4. **Not understanding String vs &str** — String owns, &str borrows
5. **Trying to return references to local variables** — Use owned types instead

---

## 🎤 Interview Questions

1. **Q: What are the three ownership rules?**
   A: (1) Each value has one owner, (2) Only one owner at a time, (3) Value is dropped when owner goes out of scope.

2. **Q: What's the difference between Copy and Clone in Rust?**
   A: `Copy` is an implicit bitwise copy for stack-only types (integers, bools). `Clone` is an explicit, potentially expensive deep copy that works for heap types too. A type can implement `Copy` only if it also implements `Clone` and contains only `Copy` fields.

3. **Q: How does Rust prevent double-free bugs?**
   A: Through ownership. When a value is assigned to a new variable, ownership moves and the original variable becomes invalid. Only one variable can own heap data at a time, so only one `drop` call happens.

4. **Q: How is Rust's move semantics different from C++ `std::move`?**
   A: In C++, `std::move` is a cast that enables the move constructor/assignment, but the source object remains in a valid "moved-from" state (using it is UB). In Rust, after a move, the source variable is completely invalid — the compiler prevents any use of it.
