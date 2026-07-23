# 🦀 Mastering Rust: 20 Deep-Dive Lessons

A comprehensive Rust curriculum from absolute beginner to production-level expert.

## Getting Started

```bash
# Source Rust environment
source "$HOME/.cargo/env"

# Run any lesson
cd lesson_01_fundamentals && cargo run
```

## Curriculum

| # | Lesson | Tier | Key Topics |
|---|--------|------|------------|
| 01 | [Fundamentals](lesson_01_fundamentals/) | 🟢 Foundation | Setup, types, variables, mutability |
| 02 | [Control Flow](lesson_02_control_flow/) | 🟢 Foundation | Expressions, match, closures, functions |
| 03 | [Ownership](lesson_03_ownership/) | 🟢 Foundation | Move, Copy, Clone, Drop |
| 04 | [Borrowing](lesson_04_borrowing/) | 🟢 Foundation | References, lifetimes, Rc, RefCell |
| 05 | [Structs & Enums](lesson_05_structs_enums/) | 🟢 Foundation | ADTs, Option, Result, Builder |
| 06 | [Traits](lesson_06_traits/) | 🟡 Intermediate | Polymorphism, dispatch, operators |
| 07 | [Generics](lesson_07_generics/) | 🟡 Intermediate | Const generics, PhantomData, typestate |
| 08 | [Iterators](lesson_08_collections_iterators/) | 🟡 Intermediate | Collections, functional programming |
| 09 | [Error Handling](lesson_09_error_handling/) | 🟡 Intermediate | Custom errors, ?, combinators |
| 10 | [Modules](lesson_10_modules_crates/) | 🟡 Intermediate | Crates, visibility, testing |
| 11 | [Smart Pointers](lesson_11_smart_pointers/) | 🟠 Advanced | Box, Rc, Weak, Deref |
| 12 | [Concurrency](lesson_12_concurrency/) | 🟠 Advanced | Threads, Mutex, channels, atomics |
| 13 | [Async/Await](lesson_13_async_await/) | 🟠 Advanced | Futures, executors, Pin |
| 14 | [Unsafe & FFI](lesson_14_unsafe_ffi/) | 🟠 Advanced | Raw pointers, C interop |
| 15 | [Lifetimes](lesson_15_lifetimes/) | 🟠 Advanced | HRTBs, 'static, zero-copy |
| 16 | [Macros](lesson_16_macros/) | 🔴 Expert | Declarative, procedural macros |
| 17 | [Design Patterns](lesson_17_design_patterns/) | 🔴 Expert | Typestate, builder, strategy |
| 18 | [Networking](lesson_18_networking_web/) | 🔴 Expert | TCP, HTTP server, file I/O |
| 19 | [Testing & Perf](lesson_19_testing_perf/) | 🔴 Expert | Unit tests, benchmarking |
| 20 | [Production](lesson_20_production/) | 🔴 Expert | KV store capstone project |

## Each Lesson Contains
- **README.md** — Deep theory with diagrams, C++ comparisons
- **src/main.rs** — Extensively commented hands-on code
- **Run with:** `cargo run` in each lesson directory
- **Test with:** `cargo test` (for lessons with tests)
