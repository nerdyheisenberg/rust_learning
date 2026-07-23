// ============================================================================
// LESSON 8: Collections, Iterators & Functional Programming
// ============================================================================

use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 8: Collections, Iterators & FP                 ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Vec<T> — The Workhorse Collection
    // ========================================================================
    println!("═══ Section 1: Vec<T> ═══\n");

    // Creating vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Push: {:?}", v);

    let v2 = vec![10, 20, 30, 40, 50];  // vec! macro
    println!("Macro: {:?}", v2);

    // Accessing elements
    let third = &v2[2];          // Panics if out of bounds
    let maybe = v2.get(10);      // Returns Option — safe!
    println!("Index: {}, Safe get: {:?}", third, maybe);

    // Vec methods
    let mut v3 = vec![5, 3, 1, 4, 2];
    v3.sort();
    println!("Sorted: {:?}", v3);
    v3.reverse();
    println!("Reversed: {:?}", v3);
    v3.retain(|&x| x > 2);
    println!("Retain > 2: {:?}", v3);
    v3.dedup();
    println!("Deduped: {:?}", v3);

    // Vec capacity vs length
    let mut v4 = Vec::with_capacity(10);
    v4.push(1);
    println!("Len: {}, Capacity: {}", v4.len(), v4.capacity());

    // Slicing
    let slice = &v2[1..4];
    println!("Slice [1..4]: {:?}", slice);

    // ========================================================================
    // SECTION 2: HashMap<K, V>
    // ========================================================================
    println!("\n═══ Section 2: HashMap<K, V> ═══\n");

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    scores.insert(String::from("Charlie"), 92);

    println!("Scores: {:?}", scores);
    println!("Alice: {:?}", scores.get("Alice"));

    // Entry API — insert only if absent
    scores.entry(String::from("Alice")).or_insert(100);    // Won't change (exists)
    scores.entry(String::from("David")).or_insert(78);     // Will insert (new)
    println!("After entry: {:?}", scores);

    // Entry API — update based on old value
    let text = "hello world wonderful world hello rust hello";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:?}", word_count);

    // Iterating over HashMap
    for (key, value) in &scores {
        println!("  {} → {}", key, value);
    }

    // Create from iterators
    let names = vec!["Alice", "Bob"];
    let ages = vec![30, 25];
    let name_ages: HashMap<_, _> = names.iter().zip(ages.iter()).collect();
    println!("Zipped: {:?}", name_ages);

    // ========================================================================
    // SECTION 3: Other Collections
    // ========================================================================
    println!("\n═══ Section 3: Other Collections ═══\n");

    // HashSet<T> — unique values
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2);  // Duplicate — ignored
    println!("HashSet: {:?}", set);

    let set_a: HashSet<_> = vec![1, 2, 3, 4].into_iter().collect();
    let set_b: HashSet<_> = vec![3, 4, 5, 6].into_iter().collect();
    println!("Union: {:?}", &set_a | &set_b);
    println!("Intersection: {:?}", &set_a & &set_b);
    println!("Difference: {:?}", &set_a - &set_b);

    // BTreeMap<K, V> — sorted by key (like C++ std::map)
    let mut btree = BTreeMap::new();
    btree.insert("Zebra", 26);
    btree.insert("Apple", 1);
    btree.insert("Mango", 13);
    println!("BTreeMap (sorted): {:?}", btree);

    // VecDeque<T> — double-ended queue
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("VecDeque: {:?}", deque);
    println!("Pop front: {:?}, Pop back: {:?}", deque.pop_front(), deque.pop_back());

    // ========================================================================
    // SECTION 4: The Iterator Trait
    // ========================================================================
    println!("\n═══ Section 4: Iterator Trait ═══\n");

    // iter() — borrows elements (&T)
    let v = vec![1, 2, 3, 4, 5];
    let mut iter = v.iter();
    println!("next: {:?}", iter.next());  // Some(&1)
    println!("next: {:?}", iter.next());  // Some(&2)
    println!("next: {:?}", iter.next());  // Some(&3)

    // into_iter() — takes ownership (T) — consumes the collection
    let v2 = vec![String::from("a"), String::from("b")];
    for s in v2.into_iter() {
        println!("  Owned: {}", s);
    }
    // v2 is consumed — can't use it anymore

    // iter_mut() — mutable references (&mut T)
    let mut v3 = vec![1, 2, 3, 4, 5];
    for x in v3.iter_mut() {
        *x *= 10;
    }
    println!("After iter_mut: {:?}", v3);

    // ========================================================================
    // SECTION 5: Iterator Adapters (Lazy Transformations)
    // ========================================================================
    println!("\n═══ Section 5: Iterator Adapters ═══\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map — transform each element
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("map (*2): {:?}", doubled);

    // filter — keep elements matching predicate
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter (even): {:?}", evens);

    // Chain multiple adapters (all lazy until collect!)
    let result: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)   // Keep evens: [2, 4, 6, 8, 10]
        .map(|&x| x * x)             // Square: [4, 16, 36, 64, 100]
        .take(3)                      // First 3: [4, 16, 36]
        .collect();
    println!("filter→map→take: {:?}", result);

    // flat_map — map and flatten
    let sentences = vec!["hello world", "foo bar baz"];
    let words: Vec<&str> = sentences.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("flat_map words: {:?}", words);

    // enumerate — get (index, value) pairs
    for (i, val) in numbers.iter().enumerate().take(5) {
        print!("[{}]={} ", i, val);
    }
    println!();

    // zip — combine two iterators
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];
    let paired: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("zip: {:?}", paired);

    // skip and take
    let middle: Vec<&i32> = numbers.iter().skip(2).take(5).collect();
    println!("skip(2).take(5): {:?}", middle);

    // chain — concatenate iterators
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let combined: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("chain: {:?}", combined);

    // scan — stateful mapping
    let running_sum: Vec<i32> = numbers.iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();
    println!("scan (running sum): {:?}", running_sum);

    // ========================================================================
    // SECTION 6: Iterator Consumers
    // ========================================================================
    println!("\n═══ Section 6: Iterator Consumers ═══\n");

    let numbers = vec![1, 2, 3, 4, 5];

    // sum, product
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    println!("sum={}, product={}", sum, product);

    // min, max
    println!("min={:?}, max={:?}", numbers.iter().min(), numbers.iter().max());

    // count
    let even_count = numbers.iter().filter(|&&x| x % 2 == 0).count();
    println!("Even count: {}", even_count);

    // any, all
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("any even: {}, all positive: {}", has_even, all_positive);

    // find — returns first match
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("find first even: {:?}", first_even);

    // position — returns index of first match
    let pos = numbers.iter().position(|&x| x == 3);
    println!("position of 3: {:?}", pos);

    // fold — accumulate (like C++ std::accumulate / reduce)
    let sentence = vec!["Hello", "World", "from", "Rust"];
    let joined = sentence.iter().fold(String::new(), |mut acc, &word| {
        if !acc.is_empty() { acc.push(' '); }
        acc.push_str(word);
        acc
    });
    println!("fold: {}", joined);

    // ========================================================================
    // SECTION 7: Custom Iterator Implementation
    // ========================================================================
    println!("\n═══ Section 7: Custom Iterator ═══\n");

    // Fibonacci iterator
    struct Fibonacci {
        a: u64,
        b: u64,
    }

    impl Fibonacci {
        fn new() -> Self {
            Fibonacci { a: 0, b: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            let result = self.a;
            let new_b = self.a + self.b;
            self.a = self.b;
            self.b = new_b;
            Some(result)  // Infinite iterator!
        }
    }

    // First 10 Fibonacci numbers
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("Fibonacci: {:?}", fibs);

    // Fibonacci sum of first 20 numbers
    let fib_sum: u64 = Fibonacci::new().take(20).sum();
    println!("Sum of first 20 Fibs: {}", fib_sum);

    // Find first Fibonacci > 1000
    let big_fib = Fibonacci::new().find(|&x| x > 1000);
    println!("First Fibonacci > 1000: {:?}", big_fib);

    // ========================================================================
    // SECTION 8: Cow<T> — Clone-on-Write
    // ========================================================================
    println!("\n═══ Section 8: Cow<T> ═══\n");

    use std::borrow::Cow;

    // Cow<'a, str> can be either Borrowed(&str) or Owned(String)
    // It only clones when mutation is needed
    fn ensure_lowercase(input: &str) -> Cow<str> {
        if input.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
            Cow::Borrowed(input)  // No allocation needed!
        } else {
            Cow::Owned(input.to_lowercase())  // Must allocate
        }
    }

    let already_lower = "hello world";
    let needs_change = "Hello World";

    let r1 = ensure_lowercase(already_lower);
    let r2 = ensure_lowercase(needs_change);
    println!("'{}' → '{}' (borrowed: {})", already_lower, r1,
             matches!(r1, Cow::Borrowed(_)));
    println!("'{}' → '{}' (borrowed: {})", needs_change, r2,
             matches!(r2, Cow::Borrowed(_)));

    println!("\n✅ Lesson 8 Complete!");
    println!("   Next: Lesson 9 — Error Handling");
}
