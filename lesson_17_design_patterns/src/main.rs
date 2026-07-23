// ============================================================================
// LESSON 17: Advanced Type System & Design Patterns
// ============================================================================

use std::marker::PhantomData;
use std::fmt;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 17: Advanced Type System & Design Patterns     ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Typestate Pattern — Compile-Time State Machines
    // ========================================================================
    println!("═══ Section 1: Typestate Pattern ═══\n");

    // TCP-like connection state machine
    struct Closed;
    struct Listening;
    struct Connected;

    struct TcpConnection<State> {
        address: String,
        _state: PhantomData<State>,
    }

    impl TcpConnection<Closed> {
        fn new(addr: &str) -> Self {
            TcpConnection { address: addr.to_string(), _state: PhantomData }
        }

        fn listen(self) -> TcpConnection<Listening> {
            println!("  Listening on {}", self.address);
            TcpConnection { address: self.address, _state: PhantomData }
        }
    }

    impl TcpConnection<Listening> {
        fn accept(self) -> TcpConnection<Connected> {
            println!("  Accepted connection on {}", self.address);
            TcpConnection { address: self.address, _state: PhantomData }
        }
    }

    impl TcpConnection<Connected> {
        fn send(&self, data: &str) {
            println!("  Sending '{}' to {}", data, self.address);
        }

        fn close(self) -> TcpConnection<Closed> {
            println!("  Connection closed");
            TcpConnection { address: self.address, _state: PhantomData }
        }
    }

    let conn = TcpConnection::<Closed>::new("127.0.0.1:8080");
    // conn.send("hello");  // ❌ COMPILE ERROR: can't send on closed connection!
    let conn = conn.listen();
    // conn.send("hello");  // ❌ COMPILE ERROR: can't send while listening!
    let conn = conn.accept();
    conn.send("hello");     // ✅ Can send on connected!
    let _closed = conn.close();

    // ========================================================================
    // SECTION 2: Builder Pattern with Compile-Time Validation
    // ========================================================================
    println!("\n═══ Section 2: Type-Safe Builder ═══\n");

    // Builder that requires certain fields at compile time
    struct Yes;
    struct No;

    #[derive(Debug)]
    struct HttpRequest {
        url: String,
        method: String,
        body: Option<String>,
    }

    struct RequestBuilder<HasUrl, HasMethod> {
        url: Option<String>,
        method: Option<String>,
        body: Option<String>,
        _url: PhantomData<HasUrl>,
        _method: PhantomData<HasMethod>,
    }

    impl RequestBuilder<No, No> {
        fn new() -> Self {
            RequestBuilder {
                url: None, method: None, body: None,
                _url: PhantomData, _method: PhantomData,
            }
        }
    }

    impl<HasMethod> RequestBuilder<No, HasMethod> {
        fn url(self, url: &str) -> RequestBuilder<Yes, HasMethod> {
            RequestBuilder {
                url: Some(url.to_string()), method: self.method, body: self.body,
                _url: PhantomData, _method: PhantomData,
            }
        }
    }

    impl<HasUrl> RequestBuilder<HasUrl, No> {
        fn method(self, method: &str) -> RequestBuilder<HasUrl, Yes> {
            RequestBuilder {
                url: self.url, method: Some(method.to_string()), body: self.body,
                _url: PhantomData, _method: PhantomData,
            }
        }
    }

    impl<HasUrl, HasMethod> RequestBuilder<HasUrl, HasMethod> {
        fn body(mut self, body: &str) -> Self {
            self.body = Some(body.to_string());
            self
        }
    }

    // build() only available when BOTH url and method are set!
    impl RequestBuilder<Yes, Yes> {
        fn build(self) -> HttpRequest {
            HttpRequest {
                url: self.url.unwrap(),
                method: self.method.unwrap(),
                body: self.body,
            }
        }
    }

    let request = RequestBuilder::new()
        .url("https://api.example.com/data")
        .method("POST")
        .body(r#"{"key": "value"}"#)
        .build();

    println!("  Request: {:#?}", request);
    // RequestBuilder::new().build();  // ❌ COMPILE ERROR: missing url and method!

    // ========================================================================
    // SECTION 3: Strategy Pattern with Trait Objects
    // ========================================================================
    println!("\n═══ Section 3: Strategy Pattern ═══\n");

    trait SortStrategy {
        fn sort(&self, data: &mut Vec<i32>);
        fn name(&self) -> &str;
    }

    struct BubbleSort;
    impl SortStrategy for BubbleSort {
        fn sort(&self, data: &mut Vec<i32>) {
            let len = data.len();
            for i in 0..len {
                for j in 0..len - 1 - i {
                    if data[j] > data[j + 1] { data.swap(j, j + 1); }
                }
            }
        }
        fn name(&self) -> &str { "BubbleSort" }
    }

    struct QuickSort;
    impl SortStrategy for QuickSort {
        fn sort(&self, data: &mut Vec<i32>) {
            data.sort(); // Use Rust's built-in (which is quicksort-based)
        }
        fn name(&self) -> &str { "QuickSort" }
    }

    struct Sorter {
        strategy: Box<dyn SortStrategy>,
    }

    impl Sorter {
        fn new(strategy: Box<dyn SortStrategy>) -> Self {
            Sorter { strategy }
        }

        fn sort(&self, data: &mut Vec<i32>) {
            println!("  Sorting with {}", self.strategy.name());
            self.strategy.sort(data);
        }

        fn set_strategy(&mut self, strategy: Box<dyn SortStrategy>) {
            self.strategy = strategy;
        }
    }

    let mut data = vec![5, 3, 8, 1, 9, 2, 7];
    let mut sorter = Sorter::new(Box::new(BubbleSort));
    sorter.sort(&mut data);
    println!("  BubbleSort result: {:?}", data);

    let mut data = vec![5, 3, 8, 1, 9, 2, 7];
    sorter.set_strategy(Box::new(QuickSort));
    sorter.sort(&mut data);
    println!("  QuickSort result: {:?}", data);

    // ========================================================================
    // SECTION 4: Observer Pattern with Closures
    // ========================================================================
    println!("\n═══ Section 4: Observer Pattern ═══\n");

    struct EventEmitter<T: Clone> {
        listeners: Vec<Box<dyn Fn(&T)>>,
    }

    impl<T: Clone> EventEmitter<T> {
        fn new() -> Self {
            EventEmitter { listeners: Vec::new() }
        }

        fn on(&mut self, listener: impl Fn(&T) + 'static) {
            self.listeners.push(Box::new(listener));
        }

        fn emit(&self, event: &T) {
            for listener in &self.listeners {
                listener(event);
            }
        }
    }

    let mut emitter = EventEmitter::new();
    emitter.on(|msg: &String| println!("  Logger: {}", msg));
    emitter.on(|msg: &String| println!("  Analytics: tracked '{}'", msg));
    emitter.on(|msg: &String| println!("  Notification: '{}'", msg.to_uppercase()));

    emitter.emit(&String::from("user_login"));
    emitter.emit(&String::from("purchase_complete"));

    // ========================================================================
    // SECTION 5: Sealed Traits
    // ========================================================================
    println!("\n═══ Section 5: Sealed Traits ═══\n");

    // A sealed trait can't be implemented outside its defining module
    mod sealed {
        pub trait Sealed {}  // Private to module

        pub trait DatabaseDriver: Sealed {
            fn connect(&self) -> String;
        }

        pub struct Postgres;
        impl Sealed for Postgres {}
        impl DatabaseDriver for Postgres {
            fn connect(&self) -> String { "Connected to PostgreSQL".into() }
        }

        pub struct MySQL;
        impl Sealed for MySQL {}
        impl DatabaseDriver for MySQL {
            fn connect(&self) -> String { "Connected to MySQL".into() }
        }
    }

    // Users can USE the trait but NOT implement it for their types:
    // struct MyDB;
    // impl sealed::DatabaseDriver for MyDB { }  // ❌ Can't implement Sealed!

    let pg = sealed::Postgres;
    println!("  {}", sealed::DatabaseDriver::connect(&pg));

    // ========================================================================
    // SECTION 6: Extension Traits
    // ========================================================================
    println!("\n═══ Section 6: Extension Traits ═══\n");

    // Add methods to types you don't own
    trait StringExt {
        fn is_palindrome(&self) -> bool;
        fn word_count(&self) -> usize;
    }

    impl StringExt for str {
        fn is_palindrome(&self) -> bool {
            let cleaned: String = self.chars()
                .filter(|c| c.is_alphanumeric())
                .map(|c| c.to_lowercase().next().unwrap())
                .collect();
            cleaned == cleaned.chars().rev().collect::<String>()
        }

        fn word_count(&self) -> usize {
            self.split_whitespace().count()
        }
    }

    println!("  'racecar'.is_palindrome(): {}", "racecar".is_palindrome());
    println!("  'hello'.is_palindrome(): {}", "hello".is_palindrome());
    println!("  'A man a plan a canal Panama'.is_palindrome(): {}",
             "A man a plan a canal Panama".is_palindrome());
    println!("  'hello world foo'.word_count(): {}", "hello world foo".word_count());

    // ========================================================================
    // SECTION 7: Command Pattern
    // ========================================================================
    println!("\n═══ Section 7: Command Pattern ═══\n");

    trait Command {
        fn execute(&mut self);
        fn undo(&mut self);
        fn description(&self) -> String;
    }

    struct TextEditor {
        content: String,
        history: Vec<Box<dyn Command>>,
    }

    struct InsertCommand {
        text: String,
        position: usize,
        content_ref: *mut String,  // Simplified — in practice use Rc<RefCell>
    }

    impl Command for InsertCommand {
        fn execute(&mut self) {
            unsafe {
                (*self.content_ref).insert_str(self.position, &self.text);
            }
        }

        fn undo(&mut self) {
            unsafe {
                let end = self.position + self.text.len();
                (*self.content_ref).drain(self.position..end);
            }
        }

        fn description(&self) -> String {
            format!("Insert '{}' at {}", self.text, self.position)
        }
    }

    // Simplified demo without the full editor
    let mut text = String::from("Hello World");
    let mut cmd = InsertCommand {
        text: " Beautiful".to_string(),
        position: 5,
        content_ref: &mut text as *mut String,
    };

    println!("  Before: {}", text);
    cmd.execute();
    println!("  After insert: {}", text);
    cmd.undo();
    println!("  After undo: {}", text);

    // ========================================================================
    // SECTION 8: Newtype + Deref for Extension
    // ========================================================================
    println!("\n═══ Section 8: Newtype for Extension ═══\n");

    // Newtype to add Display to Vec<String>
    struct PrettyVec(Vec<String>);

    impl fmt::Display for PrettyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    impl std::ops::Deref for PrettyVec {
        type Target = Vec<String>;
        fn deref(&self) -> &Vec<String> { &self.0 }
    }

    let v = PrettyVec(vec!["alice".into(), "bob".into(), "charlie".into()]);
    println!("  Display: {}", v);
    println!("  Length (via Deref): {}", v.len());

    println!("\n✅ Lesson 17 Complete!");
    println!("   Next: Lesson 18 — Networking, Web & Systems Programming");
}
