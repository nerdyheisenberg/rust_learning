# Lesson 7: Generics & Type System Mastery

## 📚 Theory Deep-Dive

### 7.1 Monomorphization — Rust's Approach to Generics
Rust creates SEPARATE compiled code for each concrete type used with a generic function.
`fn largest<T>(list: &[T]) -> &T` with `i32` and `f64` produces TWO functions in the binary.
This is like C++ templates: zero runtime cost, but potentially larger binary.

### 7.2 Const Generics
Values (not just types) as generic parameters: `struct Array<T, const N: usize>`.

### 7.3 Phantom Types
Use `PhantomData<T>` when a type parameter doesn't appear in fields but carries type-level meaning.

### 7.4 Typestate Pattern
Encode valid state transitions in the type system so invalid states are compile errors.

## 🎤 Interview Questions
1. **Q: How do Rust generics differ from Java generics?**
   A: Rust uses monomorphization (generates specialized code), Java uses type erasure (single code, casts at runtime). Rust has zero overhead, Java has slight overhead.

2. **Q: What is a PhantomData and when do you use it?**
   A: `PhantomData<T>` is a zero-sized type marker that tells the compiler the struct logically contains or refers to `T` without actually storing it. Used for variance, drop checking, and typestate patterns.
