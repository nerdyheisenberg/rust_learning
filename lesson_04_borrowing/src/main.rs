// ============================================================================
// LESSON 4: References & Borrowing
// ============================================================================
// This lesson covers:
//   - Immutable references (&T)
//   - Mutable references (&mut T)
//   - Borrowing rules in action
//   - Dangling reference prevention
//   - Lifetime annotations
//   - Struct lifetimes
//   - Interior mutability (Cell, RefCell)
//   - Rc<T> reference counting
//   - Building a graph with Rc<RefCell<T>>
// ============================================================================

use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 4: References & Borrowing                      ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Immutable References (&T)
    // ========================================================================
    println!("═══ Section 1: Immutable References ═══\n");

    let s1 = String::from("hello");

    // Instead of MOVING s1 into a function, we BORROW it:
    let len = calculate_length(&s1);  // &s1 = immutable reference
    println!("'{}' has length {}", s1, len);  // s1 is still valid!

    // Multiple immutable references are OK:
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;
    println!("Three references: {}, {}, {}", r1, r2, r3);
    // All references can coexist — read-only access is safe to share

    // ========================================================================
    // SECTION 2: Mutable References (&mut T)
    // ========================================================================
    println!("\n═══ Section 2: Mutable References ═══\n");

    let mut s = String::from("hello");
    println!("Before: {}", s);

    // Create a mutable reference
    modify_string(&mut s);
    println!("After modify: {}", s);

    // RULE: Only ONE mutable reference at a time
    let r1 = &mut s;
    // let r2 = &mut s;  // ❌ ERROR: cannot borrow `s` as mutable more than once
    r1.push_str("!!");
    println!("After r1: {}", r1);

    // RULE: Cannot mix mutable and immutable references
    let mut data = String::from("hello");
    let r1 = &data;     // Immutable borrow
    let r2 = &data;     // Another immutable borrow — OK
    println!("Immutable borrows: {}, {}", r1, r2);
    // r1 and r2 are no longer used after this point (NLL)

    // NOW we can create a mutable reference (NLL saves us!)
    let r3 = &mut data;  // ✅ OK because r1, r2 are done being used
    r3.push_str(" world");
    println!("Mutable borrow: {}", r3);

    // NLL (Non-Lexical Lifetimes) example:
    // References end at their LAST USE, not at scope end
    let mut v = vec![1, 2, 3];
    let first = &v[0];       // Immutable borrow of v
    println!("First: {}", first);  // Last use of `first`
    // `first` is no longer borrowed after this line (NLL)
    v.push(4);               // ✅ OK — mutable borrow starts after immutable ends
    println!("After push: {:?}", v);

    // ========================================================================
    // SECTION 3: Dangling References — Prevented!
    // ========================================================================
    println!("\n═══ Section 3: Dangling Reference Prevention ═══\n");

    // In C++, you can return a reference to a local variable:
    // int& dangle() { int x = 42; return x; }  // UB! Dangling reference!

    // In Rust, the compiler PREVENTS this:
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // ❌ ERROR: `s` does not live long enough
    // }

    // Solution: return the owned value instead
    let s = no_dangle();
    println!("No dangle: {}", s);

    // ========================================================================
    // SECTION 4: Borrowing Rules in Complex Scenarios
    // ========================================================================
    println!("\n═══ Section 4: Complex Borrowing Scenarios ═══\n");

    // Scenario 1: Borrowing parts of a struct
    struct Config {
        width: u32,
        height: u32,
        title: String,
    }

    let config = Config {
        width: 1920,
        height: 1080,
        title: String::from("My App"),
    };

    // Can borrow different fields simultaneously:
    let w = &config.width;
    let t = &config.title;
    println!("Width: {}, Title: {}", w, t);

    // Scenario 2: Borrowing in loops
    let v = vec![String::from("a"), String::from("b"), String::from("c")];

    // Iterate by reference — doesn't consume the vector:
    for s in &v {
        print!("{} ", s);
    }
    println!();
    println!("Vector still exists: {:?}", v);  // ✅ v is still valid

    // Iterate by mutable reference:
    let mut v2 = vec![1, 2, 3, 4, 5];
    for item in &mut v2 {
        *item *= 2;  // Dereference to modify
    }
    println!("Doubled: {:?}", v2);

    // Scenario 3: Reborrowing
    let mut data = String::from("hello");
    let r = &mut data;
    // Functions can "reborrow" from a mutable reference:
    takes_immutable_ref(r);  // Implicitly reborrows as &String
    r.push_str("!");         // Still has the mutable reference
    println!("After reborrow: {}", r);

    // ========================================================================
    // SECTION 5: Lifetime Annotations
    // ========================================================================
    println!("\n═══ Section 5: Lifetime Annotations ═══\n");

    // The compiler needs help when a function returns a reference
    // because it doesn't know which input lifetime to use:

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest: {}", result);
        // result must be used while string2 is still alive,
        // because the lifetime is the SHORTER of the two inputs
    }
    // println!("{}", result);  // ❌ Would fail if longest returned string2

    // Lifetime in only one parameter:
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("First word: {}", word);

    // ========================================================================
    // SECTION 6: Lifetimes in Structs
    // ========================================================================
    println!("\n═══ Section 6: Struct Lifetimes ═══\n");

    // A struct can hold references, but needs lifetime annotations:
    #[derive(Debug)]
    struct Excerpt<'a> {
        text: &'a str,  // This reference must live at least as long as the struct
    }

    impl<'a> Excerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        // Lifetime elision rule 3: &self's lifetime is used for return
        fn announce_and_return(&self, announcement: &str) -> &str {
            println!("Attention: {}", announcement);
            self.text  // Returns with self's lifetime
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = Excerpt { text: &novel[..i] };
    }
    println!("Excerpt: {:?}, level: {}", first_sentence, first_sentence.level());

    let result = first_sentence.announce_and_return("Important!");
    println!("Returned: {}", result);

    // ========================================================================
    // SECTION 7: Static Lifetime
    // ========================================================================
    println!("\n═══ Section 7: Static Lifetime ═══\n");

    // 'static means the reference lives for the ENTIRE program duration
    let s: &'static str = "I live forever";
    println!("Static: {}", s);

    // String literals are always &'static str — they're baked into the binary

    // You can FORCE 'static with leaking (not recommended usually):
    let leaked: &'static str = Box::leak(String::from("leaked").into_boxed_str());
    println!("Leaked static: {}", leaked);

    // ========================================================================
    // SECTION 8: Cell<T> — Interior Mutability (Simple)
    // ========================================================================
    println!("\n═══ Section 8: Cell<T> ═══\n");

    // Cell<T> lets you mutate through &self (immutable reference)
    // Works only for Copy types — get/set, no borrowing of contents

    struct CachedValue {
        value: i32,
        access_count: Cell<u32>,  // Can be mutated even through &self
    }

    let cached = CachedValue {
        value: 42,
        access_count: Cell::new(0),
    };

    // Access through immutable reference:
    let val = cached.value;
    cached.access_count.set(cached.access_count.get() + 1);  // Mutation via &self!

    let val2 = cached.value;
    cached.access_count.set(cached.access_count.get() + 1);

    println!("Value: {}, Access count: {}", val + val2, cached.access_count.get());
    // This would be impossible without Cell — you'd need &mut self to increment

    // ========================================================================
    // SECTION 9: RefCell<T> — Runtime Borrow Checking
    // ========================================================================
    println!("\n═══ Section 9: RefCell<T> ═══\n");

    // RefCell<T> moves borrow checking from compile time to runtime
    // It PANICS if you violate borrowing rules at runtime

    let data = RefCell::new(vec![1, 2, 3]);

    // borrow() returns Ref<T> (immutable borrow)
    println!("Data: {:?}", data.borrow());

    // borrow_mut() returns RefMut<T> (mutable borrow)
    data.borrow_mut().push(4);
    println!("After push: {:?}", data.borrow());

    // Multiple immutable borrows — OK at runtime:
    {
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!("Two borrows: {:?} {:?}", r1, r2);
        // r1 and r2 dropped here
    }

    // Mutable borrow — must be exclusive:
    {
        let mut r = data.borrow_mut();
        r.push(5);
        // let r2 = data.borrow();  // Would PANIC at runtime!
    }
    println!("Final: {:?}", data.borrow());

    // ========================================================================
    // SECTION 10: Rc<T> — Reference Counting
    // ========================================================================
    println!("\n═══ Section 10: Rc<T> and Rc<RefCell<T>> ═══\n");

    // Rc<T> = Reference Counted pointer (like shared_ptr in C++)
    // Multiple owners of the same data — single-threaded only

    let shared_data = Rc::new(String::from("shared"));
    println!("Reference count: {}", Rc::strong_count(&shared_data));

    let clone1 = Rc::clone(&shared_data);  // Increments ref count (no deep copy!)
    println!("After clone1: count = {}", Rc::strong_count(&shared_data));

    let clone2 = Rc::clone(&shared_data);
    println!("After clone2: count = {}", Rc::strong_count(&shared_data));

    drop(clone1);
    println!("After drop clone1: count = {}", Rc::strong_count(&shared_data));

    println!("Data: {}", shared_data);

    // Rc<RefCell<T>> — shared ownership WITH interior mutability
    // This is the go-to pattern for shared mutable data structures

    println!("\nRc<RefCell<T>> — Shared Mutable Data:");

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<RefCell<Node>>>,
    }

    // Create shared mutable nodes
    let leaf = Rc::new(RefCell::new(Node {
        value: 3,
        children: vec![],
    }));

    let branch = Rc::new(RefCell::new(Node {
        value: 5,
        children: vec![Rc::clone(&leaf)],  // Shared ownership of leaf
    }));

    // Mutate leaf through shared reference
    leaf.borrow_mut().value = 30;

    println!("Branch: {:?}", branch.borrow());
    println!("Leaf value (from branch): {}", branch.borrow().children[0].borrow().value);
    println!("Leaf ref count: {}", Rc::strong_count(&leaf));  // 2 (branch + leaf)

    // ========================================================================
    // SECTION 11: Practical Patterns
    // ========================================================================
    println!("\n═══ Section 11: Practical Patterns ═══\n");

    // Pattern 1: Slice borrowing
    let data = vec![10, 20, 30, 40, 50];
    let middle = &data[1..4];  // Borrow a slice
    println!("Middle elements: {:?}", middle);

    // Pattern 2: Option and references
    let names = vec!["Alice", "Bob", "Charlie"];
    let found = find_name(&names, "Bob");
    println!("Found Bob? {:?}", found);
    let not_found = find_name(&names, "David");
    println!("Found David? {:?}", not_found);

    // Pattern 3: Mutable reference in function chains
    let mut numbers = vec![5, 3, 1, 4, 2];
    sort_and_dedup(&mut numbers);
    println!("Sorted & deduped: {:?}", numbers);

    println!("\n✅ Lesson 4 Complete! Run: cargo run");
    println!("   Next: Lesson 5 — Structs, Enums & Pattern Matching");
}

// ============================================================================
// FUNCTION DEFINITIONS
// ============================================================================

// Borrows a String — doesn't take ownership
fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope here, but since it doesn't own the String,
    // nothing happens (no Drop called)
}

// Modifies through a mutable reference
fn modify_string(s: &mut String) {
    s.push_str(" world");
}

// Cannot return reference to local — must return owned type
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Ownership transferred to caller — no dangling!
}

// Takes an immutable reference (reborrowing from &mut works too)
fn takes_immutable_ref(s: &String) {
    println!("  Reborrowed: {}", s);
}

// Lifetime annotation: return value lives as long as the SHORTER input lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime elision: one input reference → output gets the same lifetime
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

// Borrowing a slice for searching
fn find_name<'a>(names: &'a [&str], target: &str) -> Option<&'a str> {
    for &name in names {
        if name == target {
            return Some(name);
        }
    }
    None
}

// Mutable reference for in-place modification
fn sort_and_dedup(v: &mut Vec<i32>) {
    v.sort();
    v.dedup();
}
