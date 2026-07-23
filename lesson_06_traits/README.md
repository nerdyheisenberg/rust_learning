# Lesson 6: Traits — Rust's Polymorphism

## 🎯 Learning Objectives
- Understand traits as Rust's polymorphism mechanism
- Master static vs dynamic dispatch
- Implement standard library traits
- Use trait bounds, supertraits, and associated types

---

## 📚 Theory Deep-Dive

### 6.1 Traits vs Interfaces vs Virtual Functions

```
┌────────────────────────────────────────────────────────────────────┐
│            Polymorphism Comparison                                  │
├──────────────┬──────────────┬──────────────┬──────────────────────┤
│ Feature       │ C++ Virtual  │ Go Interface │ Rust Traits          │
├──────────────┼──────────────┼──────────────┼──────────────────────┤
│ Dispatch      │ Dynamic      │ Dynamic      │ BOTH static & dyn   │
│ Inheritance   │ Yes          │ No           │ No (composition)    │
│ Default impl  │ Yes          │ No           │ Yes                  │
│ Generics      │ Templates    │ Type params  │ Trait bounds         │
│ Vtable cost   │ Always       │ Always       │ Only for dyn Trait   │
│ Data in trait │ Member vars  │ No           │ Associated types     │
│ Orphan rule   │ No           │ No           │ Yes (coherence)      │
│ Operator OL   │ Separate     │ No           │ Via traits           │
└──────────────┴──────────────┴──────────────┴──────────────────────┘
```

### 6.2 Static vs Dynamic Dispatch

**Static dispatch (monomorphization)** — like C++ templates:
```rust
fn print_it<T: Display>(item: T) { ... }
// Compiler generates: print_it_i32(), print_it_String(), etc.
// Zero runtime cost — the function is specialized at compile time
```

**Dynamic dispatch (trait objects)** — like C++ virtual functions:
```rust
fn print_it(item: &dyn Display) { ... }
// Single function — uses vtable pointer at runtime
// Small runtime cost: indirection through vtable

// A trait object is a "FAT POINTER":
┌─────────────────────┐
│ data pointer    ──────▶ actual object data
│ vtable pointer  ──────▶ [drop_fn, size, align, method1, method2, ...]
└─────────────────────┘
```

### 6.3 The Orphan Rule

You can implement a trait for a type only if EITHER:
- The trait is defined in YOUR crate, OR
- The type is defined in YOUR crate

You CANNOT implement a foreign trait on a foreign type:
```rust
impl Display for Vec<i32> { }  // ❌ Both Display and Vec are from std
```
This prevents conflicting implementations across crates.

### 6.4 Auto Traits: Send, Sync, Sized

```
Send:  Type can be transferred across thread boundaries
Sync:  Type can be shared between threads via references
Sized: Type has a known size at compile time

Most types are Send + Sync + Sized automatically.
Rc<T> is NOT Send/Sync (single-threaded).
dyn Trait is NOT Sized (unknown concrete type).
```

---

## 🎤 Interview Questions

1. **Q: What's the difference between static and dynamic dispatch?**
   A: Static dispatch uses monomorphization (generates code for each concrete type, zero cost). Dynamic dispatch uses vtable pointers at runtime (one function for all types, small overhead).

2. **Q: When would you use `dyn Trait` vs generics?**
   A: Use generics when performance matters and types are known at compile time. Use `dyn Trait` when you need heterogeneous collections, reduced binary size, or types determined at runtime.

3. **Q: What is the orphan rule?**
   A: You can only implement a trait for a type if either the trait or the type is defined in your own crate. This prevents conflicting implementations.

4. **Q: What's an associated type vs a generic parameter on a trait?**
   A: Associated types make the trait have one implementation per type (like Iterator), while generic parameters allow multiple implementations (like `From<T>`).
