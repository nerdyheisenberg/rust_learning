// ============================================================================
// LESSON 15: Lifetimes — The Complete Mastery
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 15: Lifetimes — The Complete Mastery           ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Lifetime Elision Rules Revisited
    // ========================================================================
    println!("═══ Section 1: Lifetime Elision ═══\n");

    // The 3 elision rules the compiler applies:
    //
    // Rule 1: Each reference parameter gets its own lifetime
    //   fn foo(x: &str, y: &str) → fn foo<'a, 'b>(x: &'a str, y: &'b str)
    //
    // Rule 2: If there's exactly ONE input lifetime, all outputs use it
    //   fn foo(x: &str) -> &str → fn foo<'a>(x: &'a str) -> &'a str
    //
    // Rule 3: If &self or &mut self, output lifetime = self's lifetime
    //   fn foo(&self) -> &str → fn foo<'a>(&'a self) -> &'a str

    // Examples where elision works:
    fn first_word(s: &str) -> &str {  // Rule 1+2: one input → output uses it
        s.split_whitespace().next().unwrap_or("")
    }
    println!("First word: {}", first_word("hello world"));

    // Example where elision FAILS — need explicit annotations:
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    let result = longest("hello", "world!");
    println!("Longest: {}", result);

    // ========================================================================
    // SECTION 2: Multiple Lifetimes
    // ========================================================================
    println!("\n═══ Section 2: Multiple Lifetimes ═══\n");

    // Different lifetimes for different parameters
    fn first_ref<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
        x  // Always returns x, so only needs 'a
    }

    let string1 = String::from("long");
    let result;
    {
        let string2 = String::from("short");
        result = first_ref(&string1, &string2);  // OK: result lives as long as string1
    }
    println!("result: {}", result);  // ✅ Fine — result references string1

    // ========================================================================
    // SECTION 3: Struct Lifetimes
    // ========================================================================
    println!("\n═══ Section 3: Struct Lifetimes ═══\n");

    // A struct holding a reference must declare a lifetime
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        // Rule 3 applies: &self's lifetime used for output
        fn level(&self) -> i32 { 3 }

        fn announce(&self, announcement: &str) -> &str {
            println!("Attention: {}", announcement);
            self.part  // Returns with 'a lifetime (from self)
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt {
        part: &novel[..14],
    };
    println!("Excerpt: {:?}, level: {}", excerpt, excerpt.level());

    let text = excerpt.announce("Important!");
    println!("Announced text: {}", text);

    // ========================================================================
    // SECTION 4: Lifetime Subtyping ('a: 'b)
    // ========================================================================
    println!("\n═══ Section 4: Lifetime Subtyping ═══\n");

    // 'a: 'b means "'a lives at least as long as 'b"
    // Think: 'a is a "wider" lifetime

    #[derive(Debug)]
    struct Parser<'input> {
        input: &'input str,
        position: usize,
    }

    impl<'input> Parser<'input> {
        fn new(input: &'input str) -> Self {
            Parser { input, position: 0 }
        }

        fn next_word(&mut self) -> Option<&'input str> {
            let remaining = &self.input[self.position..];
            let trimmed = remaining.trim_start();
            if trimmed.is_empty() { return None; }

            let word_end = trimmed.find(' ').unwrap_or(trimmed.len());
            let word_start = self.input.len() - remaining.len() + (remaining.len() - trimmed.len());
            self.position = word_start + word_end;
            Some(&self.input[word_start..word_start + word_end])
        }
    }

    let text = String::from("hello world from rust");
    let mut parser = Parser::new(&text);
    while let Some(word) = parser.next_word() {
        print!("[{}] ", word);
    }
    println!();

    // ========================================================================
    // SECTION 5: 'static Lifetime — Myths and Reality
    // ========================================================================
    println!("\n═══ Section 5: 'static Lifetime ═══\n");

    // MYTH: 'static means "lives forever"
    // REALITY: 'static means "CAN live for the entire program duration"
    //          (but can be dropped earlier!)

    // String literals are 'static — baked into the binary
    let s: &'static str = "I live forever";
    println!("Static string: {}", s);

    // Owned types satisfy 'static bound too!
    // String: 'static means String doesn't contain any non-'static references
    fn takes_static<T: 'static>(val: T) -> T {
        val
    }

    let owned = String::from("I'm 'static because I own my data");
    let result = takes_static(owned);  // ✅ String is 'static!
    println!("Static-bounded: {}", result);

    // This DOESN'T work:
    // let borrowed = &String::from("temp");
    // takes_static(borrowed);  // ❌ &String is NOT 'static

    // Thread spawning requires 'static because thread may outlive caller
    let handle = std::thread::spawn(move || {
        let s: &'static str = "Thread-safe static";
        println!("  From thread: {}", s);
    });
    handle.join().unwrap();

    // ========================================================================
    // SECTION 6: Higher-Rank Trait Bounds (HRTBs)
    // ========================================================================
    println!("\n═══ Section 6: Higher-Rank Trait Bounds ═══\n");

    // for<'a> means "for ALL possible lifetimes 'a"
    // Used when a closure must work with ANY lifetime input

    fn apply_to_str<F>(f: F, s: &str) -> String
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        f(s).to_string()
    }

    let result = apply_to_str(|s| &s[..5], "hello world");
    println!("HRTB result: {}", result);

    // Most closures taking references implicitly use HRTBs
    fn call_with_ref<F>(f: F) -> i32
    where
        F: for<'a> Fn(&'a i32) -> i32,
    {
        let value = 42;
        f(&value)
    }

    let doubled = call_with_ref(|x| x * 2);
    println!("HRTB doubled: {}", doubled);

    // ========================================================================
    // SECTION 7: Common Lifetime Patterns
    // ========================================================================
    println!("\n═══ Section 7: Common Patterns ═══\n");

    // Pattern 1: Container returning reference to element
    struct StringPool {
        strings: Vec<String>,
    }

    impl StringPool {
        fn new() -> Self {
            StringPool { strings: Vec::new() }
        }

        fn add(&mut self, s: &str) -> &str {
            self.strings.push(s.to_string());
            self.strings.last().unwrap()
        }

        fn get(&self, index: usize) -> Option<&str> {
            self.strings.get(index).map(|s| s.as_str())
        }
    }

    let mut pool = StringPool::new();
    pool.add("hello");
    pool.add("world");
    println!("Pool[0]: {:?}", pool.get(0));
    println!("Pool[1]: {:?}", pool.get(1));

    // Pattern 2: Iterator returning references
    struct Words<'a> {
        text: &'a str,
        position: usize,
    }

    impl<'a> Iterator for Words<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<&'a str> {
            let remaining = &self.text[self.position..].trim_start();
            if remaining.is_empty() { return None; }

            let end = remaining.find(' ').unwrap_or(remaining.len());
            let start = self.text.len() - remaining.len();
            self.position = start + end;
            Some(&self.text[start..start + end])
        }
    }

    let text = "the quick brown fox";
    let words = Words { text, position: 0 };
    let collected: Vec<&str> = words.collect();
    println!("Words iterator: {:?}", collected);

    // Pattern 3: Zero-copy parsing
    #[derive(Debug)]
    struct Header<'a> {
        name: &'a str,
        value: &'a str,
    }

    fn parse_header(line: &str) -> Option<Header> {
        let colon = line.find(':')?;
        Some(Header {
            name: line[..colon].trim(),
            value: line[colon + 1..].trim(),
        })
    }

    let line = "Content-Type: application/json";
    if let Some(header) = parse_header(line) {
        println!("Zero-copy header: {:?}", header);
        println!("  Name points into original string!");
    }

    // ========================================================================
    // SECTION 8: Anti-Patterns and Solutions
    // ========================================================================
    println!("\n═══ Section 8: Anti-Patterns ═══\n");

    // Anti-pattern 1: Fighting the borrow checker with cloning
    // BAD: cloning everything
    // GOOD: restructure to use references properly

    // Anti-pattern 2: Trying to store references in structs when owned types work
    // BAD: struct Config<'a> { name: &'a str }
    // GOOD: struct Config { name: String }  (for owned configuration)

    // Anti-pattern 3: Self-referential structs
    // IMPOSSIBLE in safe Rust:
    // struct SelfRef {
    //     data: String,
    //     reference: &str,  // Can't point to data!
    // }
    // Solutions: use Rc, indices, or the `ouroboros` crate

    println!("  Anti-pattern 1: Excessive cloning → use references");
    println!("  Anti-pattern 2: Refs in structs → use owned types for config");
    println!("  Anti-pattern 3: Self-referential → use Rc, indices, or ouroboros");

    println!("\n✅ Lesson 15 Complete!");
    println!("   Next: Lesson 16 — Macros (Metaprogramming)");
}
