// ============================================================================
// LESSON 3: Ownership — The Heart of Rust
// ============================================================================
// This lesson covers:
//   - The three ownership rules in action
//   - Move semantics for heap types
//   - Copy types vs Clone types
//   - Ownership through function calls
//   - Scope-based resource management (Drop)
//   - String vs &str ownership
//   - Practical patterns for ownership transfer
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 3: Ownership — The Heart of Rust               ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Basic Ownership — Scope and Drop
    // ========================================================================
    println!("═══ Section 1: Scope and Drop ═══\n");

    {
        // `s` comes into scope
        let s = String::from("hello");
        println!("Inside scope: {}", s);
        // `s` goes out of scope here — Drop is called, memory is freed
    }
    // println!("{}", s);  // ❌ ERROR: `s` is no longer in scope

    // Stack types are also scoped, but no heap deallocation needed
    {
        let x = 42;
        let y = 3.14;
        println!("Stack values: {}, {}", x, y);
    }
    // x, y are gone but no heap cleanup was needed

    // Nested scopes — Drop runs in REVERSE order (like C++ destructors)
    {
        let a = String::from("first");
        let b = String::from("second");
        let c = String::from("third");
        println!("a={}, b={}, c={}", a, b, c);
        // Drop order: c, b, a (LIFO — last created, first dropped)
    }

    // ========================================================================
    // SECTION 2: Move Semantics — The Default for Heap Types
    // ========================================================================
    println!("\n═══ Section 2: Move Semantics ═══\n");

    // For heap-allocated types, assignment MOVES ownership:
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2 — s1 is now INVALID!

    // println!("{}", s1);  // ❌ COMPILE ERROR: value used here after move
    println!("s2 (after move from s1): {}", s2);

    // What happens in memory:
    // BEFORE move:
    //   s1 → stack{ptr, len:5, cap:5} → heap["hello"]
    //
    // AFTER move:
    //   s1 → (invalidated — compiler marks as unusable)
    //   s2 → stack{ptr, len:5, cap:5} → heap["hello"]
    //
    // The heap data wasn't copied! Only the stack metadata was moved.

    // Vec<T> also moves (any heap type does):
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = v1;  // MOVED!
    // println!("{:?}", v1);  // ❌ ERROR: v1 was moved
    println!("v2 (moved from v1): {:?}", v2);

    // ========================================================================
    // SECTION 3: Copy Types — No Move, Just Copy
    // ========================================================================
    println!("\n═══ Section 3: Copy Types ═══\n");

    // For types that implement the `Copy` trait, assignment COPIES:
    let x = 42;
    let y = x;  // COPY — x is still valid!
    println!("x={}, y={} (both valid — integer is Copy)", x, y);

    let a = 3.14;
    let b = a;  // COPY
    println!("a={}, b={} (both valid — float is Copy)", a, b);

    let c = true;
    let d = c;  // COPY
    println!("c={}, d={} (both valid — bool is Copy)", c, d);

    let e = 'Z';
    let f = e;  // COPY
    println!("e={}, f={} (both valid — char is Copy)", e, f);

    // Tuples of Copy types are also Copy:
    let t1 = (1, 2.0, true);
    let t2 = t1;  // COPY
    println!("t1={:?}, t2={:?} (tuple of Copy types is Copy)", t1, t2);

    // Arrays of Copy types are also Copy:
    let arr1 = [1, 2, 3];
    let arr2 = arr1;  // COPY
    println!("arr1={:?}, arr2={:?} (array of Copy types is Copy)", arr1, arr2);

    // BUT: If a tuple/struct contains a non-Copy type, it's NOT Copy:
    let mixed = (1, String::from("hello"));
    let mixed2 = mixed;  // MOVE! Because String is not Copy
    // println!("{:?}", mixed);  // ❌ ERROR: mixed was moved
    println!("mixed2 (moved): {:?}", mixed2);

    // ========================================================================
    // SECTION 4: Clone — Explicit Deep Copy
    // ========================================================================
    println!("\n═══ Section 4: Clone — Explicit Deep Copy ═══\n");

    // When you NEED a copy of a heap type, use .clone()
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Deep copy — new heap allocation!
    println!("s1={}, s2={} (both valid after clone)", s1, s2);

    // In memory:
    // s1 → stack{ptr_A, len:5, cap:5} → heap_A["hello"]
    // s2 → stack{ptr_B, len:5, cap:5} → heap_B["hello"]  ← NEW allocation!

    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();
    println!("v1={:?}, v2={:?} (both valid after clone)", v1, v2);

    // Clone can be expensive! Each element is cloned:
    let strings = vec![
        String::from("hello"),
        String::from("world"),
        String::from("rust"),
    ];
    let strings2 = strings.clone();  // Clones 3 Strings = 3 heap allocations
    println!("Original: {:?}", strings);
    println!("Cloned:   {:?}", strings2);

    // ========================================================================
    // SECTION 5: Ownership and Functions
    // ========================================================================
    println!("\n═══ Section 5: Ownership and Functions ═══\n");

    // Passing a heap type to a function MOVES it:
    let name = String::from("Rohit");
    take_ownership(name);
    // println!("{}", name);  // ❌ ERROR: name was moved into function

    // Passing a Copy type to a function COPIES it:
    let age = 30;
    take_copy(age);
    println!("age is still valid: {}", age);  // ✅ Fine — age was copied

    // Getting ownership back by returning:
    let s = give_ownership();
    println!("Received ownership: {}", s);

    // Taking and giving back (awkward pattern — borrowing is better):
    let s = String::from("hello");
    let (s, len) = calculate_length_awkward(s);  // Give and take back
    println!("'{}' has length {} (took ownership and returned)", s, len);

    // ========================================================================
    // SECTION 6: String vs &str — Ownership in Practice
    // ========================================================================
    println!("\n═══ Section 6: String vs &str ═══\n");

    // &str — a BORROWED string slice (does NOT own the data)
    let literal: &str = "I am a string literal";  // Stored in the binary
    println!("Literal: {}", literal);

    // String — an OWNED, heap-allocated string
    let mut owned = String::from("I am owned");
    owned.push_str(" and mutable");
    println!("Owned: {}", owned);

    // Converting between them:
    // &str → String (allocates on heap)
    let from_literal: String = literal.to_string();
    let from_literal2: String = String::from(literal);
    let from_literal3: String = literal.to_owned();
    println!("Conversions: {}", from_literal);
    println!("           : {}", from_literal2);
    println!("           : {}", from_literal3);

    // String → &str (just borrows, no allocation)
    let owned = String::from("hello");
    let borrowed: &str = &owned;  // Deref coercion: &String → &str
    println!("Borrowed from owned: {}", borrowed);

    // Function parameters should usually take &str (accepts both)
    print_greeting("literal &str");              // ✅ &str works
    print_greeting(&String::from("owned String"));  // ✅ &String coerces to &str

    // ========================================================================
    // SECTION 7: Partial Moves
    // ========================================================================
    println!("\n═══ Section 7: Partial Moves ═══\n");

    // Struct fields can be moved independently:
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Rohit"),
        age: 30,
    };

    // Move just the name field:
    let name = person.name;  // Partial move — name field is moved
    println!("Name: {}", name);
    println!("Age: {}", person.age);  // ✅ age is Copy, still accessible
    // println!("{:?}", person);  // ❌ ERROR: person is partially moved
    // println!("{}", person.name);  // ❌ ERROR: name was moved

    // ========================================================================
    // SECTION 8: Custom Types — Implementing Copy and Clone
    // ========================================================================
    println!("\n═══ Section 8: Custom Copy and Clone ═══\n");

    // A struct is automatically Copy if ALL its fields are Copy
    // AND you derive Copy + Clone:
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1;  // COPY — p1 is still valid!
    println!("p1={:?}, p2={:?} (Point is Copy)", p1, p2);

    // A struct with a String field CANNOT be Copy (only Clone):
    #[derive(Debug, Clone)]
    struct Player {
        name: String,
        score: u32,
    }

    let player1 = Player {
        name: String::from("Alice"),
        score: 100,
    };
    // let player2 = player1;  // Would MOVE

    let player2 = player1.clone();  // Must explicitly clone
    println!("player1={:?}", player1);
    println!("player2={:?}", player2);

    // ========================================================================
    // SECTION 9: Drop Trait — Rust's Destructor
    // ========================================================================
    println!("\n═══ Section 9: Drop Trait ═══\n");

    struct ResourceHolder {
        name: String,
    }

    impl Drop for ResourceHolder {
        fn drop(&mut self) {
            println!("  📦 Dropping ResourceHolder: {}", self.name);
        }
    }

    println!("Creating resources...");
    {
        let _r1 = ResourceHolder { name: String::from("Database Connection") };
        let _r2 = ResourceHolder { name: String::from("File Handle") };
        let _r3 = ResourceHolder { name: String::from("Network Socket") };
        println!("  Resources created. About to leave scope...");
        // Drop runs in REVERSE order: r3, r2, r1
    }
    println!("All resources dropped.\n");

    // Early drop with std::mem::drop()
    println!("Early drop example:");
    let resource = ResourceHolder { name: String::from("Temporary") };
    println!("  Resource exists");
    drop(resource);  // Explicitly drop early
    println!("  Resource was dropped early (before scope end)");
    // println!("{}", resource.name);  // ❌ ERROR: value used after drop/move

    // ========================================================================
    // SECTION 10: Ownership Patterns and Idioms
    // ========================================================================
    println!("\n═══ Section 10: Ownership Patterns ═══\n");

    // Pattern 1: Builder pattern with ownership chaining
    let sentence = OwnedBuilder::new()
        .add("Hello")
        .add("World")
        .add("from")
        .add("Rust")
        .build();
    println!("Builder: {}", sentence);

    // Pattern 2: Taking ownership to ensure cleanup
    let file_data = read_and_process(String::from("data.txt"));
    println!("Processed: {}", file_data);

    // Pattern 3: Temporary ownership in a scope
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let sorted = {
        let mut temp = data.clone();
        temp.sort();
        temp  // Return sorted copy, original unchanged
    };
    println!("Original: {:?}", data);
    println!("Sorted:   {:?}", sorted);

    println!("\n✅ Lesson 3 Complete! Run: cargo run");
    println!("   Next: Lesson 4 — References & Borrowing");
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

// Takes ownership — the String is MOVED into this function
fn take_ownership(s: String) {
    println!("Took ownership of: {}", s);
    // s is dropped when this function ends
}

// Takes a Copy — the i32 is COPIED
fn take_copy(x: i32) {
    println!("Got a copy: {}", x);
}

// Gives ownership — creates a String and transfers it to the caller
fn give_ownership() -> String {
    let s = String::from("I was born in a function");
    s  // Ownership transferred to caller
}

// Awkward pattern: take ownership and give it back (use borrowing instead!)
fn calculate_length_awkward(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // Return the String AND the computed value
}

// Best practice: accept &str to be flexible (both &str and &String work)
fn print_greeting(name: &str) {
    println!("  Greeting: Hello, {}!", name);
}

// Simulated file processing — takes ownership to ensure exclusive access
fn read_and_process(filename: String) -> String {
    format!("Processed contents of '{}'", filename)
    // filename is dropped here — we're done with it
}

// Builder pattern using ownership (method chaining with self)
struct OwnedBuilder {
    words: Vec<String>,
}

impl OwnedBuilder {
    fn new() -> Self {
        OwnedBuilder { words: Vec::new() }
    }

    // Takes self by value (ownership) and returns it — enables chaining
    fn add(mut self, word: &str) -> Self {
        self.words.push(String::from(word));
        self
    }

    fn build(self) -> String {
        self.words.join(" ")
    }
}
