# Lesson 9: Error Handling — The Complete Picture

## 📚 Theory

### Rust's Error Philosophy
- **Recoverable errors** → `Result<T, E>` (file not found, parse failure)
- **Unrecoverable errors** → `panic!` (index out of bounds, assertion failure)
- **No exceptions** — errors are values in the type system

### The `?` Operator Desugaring
```rust
let value = some_operation()?;
// Desugars to:
let value = match some_operation() {
    Ok(v) => v,
    Err(e) => return Err(e.into()),  // Note: calls .into()!
};
```
The `.into()` call means `?` automatically converts error types if `From` is implemented.

### Error Hierarchy
```
std::error::Error (trait)
├── std::io::Error
├── std::num::ParseIntError
├── std::fmt::Error
├── Custom errors...
└── Box<dyn Error>  ← catch-all for any error type
```

### When to panic! vs return Result
- **Library code**: Almost always use `Result` — let the caller decide
- **Application code**: Can use `panic!` for truly unrecoverable situations
- **Prototyping**: `unwrap()` and `expect()` are fine temporarily
- **Tests**: `unwrap()` is fine — test failures are expected
