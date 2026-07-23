// ============================================================================
// LESSON 14: Unsafe Rust & FFI
// ============================================================================
// Unsafe Rust = a contract with the compiler. You promise to uphold invariants
// that the compiler can't verify. The compiler trusts you — but if you're wrong,
// it's undefined behavior.
// ============================================================================

use std::slice;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 14: Unsafe Rust & FFI                          ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: The Five Unsafe Superpowers
    // ========================================================================
    println!("═══ Section 1: Five Unsafe Superpowers ═══\n");

    println!("  1. Dereference raw pointers (*const T, *mut T)");
    println!("  2. Call unsafe functions/methods");
    println!("  3. Access/modify mutable static variables");
    println!("  4. Implement unsafe traits");
    println!("  5. Access fields of unions\n");

    // ========================================================================
    // SECTION 2: Raw Pointers
    // ========================================================================
    println!("═══ Section 2: Raw Pointers ═══\n");

    // Creating raw pointers is safe — dereferencing them requires unsafe
    let mut value = 42;

    let ptr_const: *const i32 = &value;     // Raw const pointer
    let ptr_mut: *mut i32 = &mut value;     // Raw mutable pointer

    println!("Raw const ptr: {:?}", ptr_const);
    println!("Raw mut ptr: {:?}", ptr_mut);

    // DEREFERENCING requires unsafe
    unsafe {
        println!("Dereferenced const: {}", *ptr_const);
        *ptr_mut = 100;
        println!("After unsafe mutation: {}", *ptr_mut);
    }
    println!("Value after unsafe: {}", value);

    // Raw pointers from arbitrary addresses (VERY dangerous in practice)
    let address = 0x012345usize;
    let _dangerous_ptr = address as *const i32;
    // Dereferencing _dangerous_ptr would be UB — the memory isn't ours!

    // Raw pointers CAN be null (unlike references)
    let null_ptr: *const i32 = std::ptr::null();
    println!("Null pointer: {:?}", null_ptr);
    println!("Is null: {}", null_ptr.is_null());

    // ========================================================================
    // SECTION 3: Calling Unsafe Functions
    // ========================================================================
    println!("\n═══ Section 3: Unsafe Functions ═══\n");

    // Some standard library functions are unsafe
    unsafe {
        // slice::from_raw_parts — creates a slice from pointer + length
        let data = vec![1, 2, 3, 4, 5];
        let ptr = data.as_ptr();
        let len = data.len();

        let slice = slice::from_raw_parts(ptr, len);
        println!("Slice from raw parts: {:?}", slice);
    }

    // Safe wrapper around unsafe code — THE pattern to follow
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len, "mid must be within bounds");

        unsafe {
            // We KNOW this is safe because:
            // 1. ptr is valid (came from a valid slice)
            // 2. The two slices don't overlap (one is [0..mid], other [mid..len])
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut data = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut data, 3);
    println!("Left: {:?}, Right: {:?}", left, right);
    left[0] = 100;
    right[0] = 200;
    println!("Modified: {:?}", data);

    // ========================================================================
    // SECTION 4: Mutable Static Variables
    // ========================================================================
    println!("\n═══ Section 4: Mutable Statics ═══\n");

    static mut COUNTER: u32 = 0;

    // Accessing mutable statics is unsafe (potential data race)
    unsafe {
        COUNTER += 1;
        COUNTER += 1;
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }

    // Better alternative: use atomics
    use std::sync::atomic::{AtomicU32, Ordering};
    static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);

    ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
    ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("Atomic counter: {}", ATOMIC_COUNTER.load(Ordering::SeqCst));

    // ========================================================================
    // SECTION 5: Unsafe Traits
    // ========================================================================
    println!("\n═══ Section 5: Unsafe Traits ═══\n");

    // Send and Sync are unsafe traits — they make promises about thread safety
    // that the compiler can't verify

    struct MyType {
        data: *mut i32,  // Raw pointer — NOT Send/Sync by default
    }

    // Promising the compiler this is safe to send across threads
    // YOU must ensure this is actually true!
    unsafe impl Send for MyType {}
    unsafe impl Sync for MyType {}

    println!("  unsafe impl Send: promise type can be transferred between threads");
    println!("  unsafe impl Sync: promise type can be shared between threads");

    // ========================================================================
    // SECTION 6: Unions (C-compatible)
    // ========================================================================
    println!("\n═══ Section 6: Unions ═══\n");

    // Unions — like C unions, all fields share memory
    #[repr(C)]
    union IntOrFloat {
        i: i32,
        f: f32,
    }

    let u = IntOrFloat { i: 42 };
    unsafe {
        println!("As int: {}", u.i);
        println!("As float: {} (reinterpreted bits!)", u.f);
    }

    let u = IntOrFloat { f: 3.14 };
    unsafe {
        println!("As float: {}", u.f);
        println!("As int: {} (reinterpreted bits!)", u.i);
    }

    println!("Union size: {} bytes (max of i32/f32)", std::mem::size_of::<IntOrFloat>());

    // ========================================================================
    // SECTION 7: FFI — Calling C from Rust
    // ========================================================================
    println!("\n═══ Section 7: FFI — Calling C ═══\n");

    // extern "C" declares functions with C ABI
    extern "C" {
        fn abs(input: i32) -> i32;       // C standard library
        fn sqrt(input: f64) -> f64;      // C math library
    }

    unsafe {
        println!("C abs(-42) = {}", abs(-42));
        println!("C sqrt(144.0) = {}", sqrt(144.0));
    }

    // ========================================================================
    // SECTION 8: Exposing Rust Functions to C
    // ========================================================================
    println!("\n═══ Section 8: Exposing Rust to C ═══\n");

    // #[no_mangle] prevents Rust from mangling the function name
    // extern "C" uses the C calling convention
    #[no_mangle]
    pub extern "C" fn rust_function(x: i32) -> i32 {
        x * 2
    }

    // This function could be called from C code
    println!("rust_function(21) = {}", rust_function(21));

    // ========================================================================
    // SECTION 9: #[repr(C)] — C-compatible Layout
    // ========================================================================
    println!("\n═══ Section 9: repr(C) ═══\n");

    // Rust can reorder struct fields for optimization
    // #[repr(C)] forces C-compatible layout
    #[repr(C)]
    struct CCompatible {
        x: i32,
        y: f64,
        z: i32,
    }

    println!("CCompatible size: {} bytes", std::mem::size_of::<CCompatible>());
    println!("CCompatible align: {} bytes", std::mem::align_of::<CCompatible>());

    // Without repr(C), Rust might reorder fields:
    struct RustOptimized {
        x: i32,
        y: f64,
        z: i32,
    }

    println!("RustOptimized size: {} bytes", std::mem::size_of::<RustOptimized>());

    // ========================================================================
    // SECTION 10: Safe Abstractions Pattern
    // ========================================================================
    println!("\n═══ Section 10: Safe Abstractions ═══\n");

    // The golden rule: Use unsafe internally, expose a safe API

    struct SafeBuffer {
        ptr: *mut u8,
        len: usize,
        cap: usize,
    }

    impl SafeBuffer {
        fn new(cap: usize) -> Self {
            let layout = std::alloc::Layout::array::<u8>(cap).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            SafeBuffer { ptr, len: 0, cap }
        }

        // Safe public API
        fn push(&mut self, value: u8) {
            assert!(self.len < self.cap, "Buffer full");
            unsafe {
                self.ptr.add(self.len).write(value);
            }
            self.len += 1;
        }

        fn get(&self, index: usize) -> Option<u8> {
            if index < self.len {
                unsafe { Some(self.ptr.add(index).read()) }
            } else {
                None
            }
        }

        fn len(&self) -> usize {
            self.len
        }
    }

    impl Drop for SafeBuffer {
        fn drop(&mut self) {
            unsafe {
                let layout = std::alloc::Layout::array::<u8>(self.cap).unwrap();
                std::alloc::dealloc(self.ptr, layout);
            }
        }
    }

    let mut buf = SafeBuffer::new(10);
    buf.push(b'H');
    buf.push(b'i');
    buf.push(b'!');
    println!("Buffer length: {}", buf.len());
    println!("Buffer[0]: {:?} = '{}'", buf.get(0), buf.get(0).unwrap() as char);
    println!("Buffer[1]: {:?} = '{}'", buf.get(1), buf.get(1).unwrap() as char);
    println!("Buffer[5]: {:?} (out of bounds = None)", buf.get(5));

    println!("\n✅ Lesson 14 Complete!");
    println!("   Next: Lesson 15 — Lifetimes (The Complete Mastery)");
}
