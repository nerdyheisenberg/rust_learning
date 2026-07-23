// ============================================================================
// LESSON 11: Smart Pointers & Memory Management
// ============================================================================

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::ops::Deref;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 11: Smart Pointers & Memory Management         ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Box<T> — Heap Allocation
    // ========================================================================
    println!("═══ Section 1: Box<T> ═══\n");

    // Box<T> puts data on the heap (like C++ unique_ptr)
    let boxed = Box::new(42);
    println!("Boxed value: {}", boxed);
    println!("Box size: {} bytes (just a pointer)", std::mem::size_of::<Box<i32>>());

    // Use case 1: Recursive data structures
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("List: {:?}", list);
    println!("List size: {} bytes", std::mem::size_of::<List>());

    // Use case 2: Large data on heap to avoid stack overflow
    let _big_array = Box::new([0u8; 1_000_000]);  // 1MB on heap, not stack!
    println!("Large array boxed (1MB on heap)");

    // Box implements Deref, so you can use it like a reference:
    let x = Box::new(String::from("hello"));
    println!("Box deref: length = {}", x.len());  // Auto-deref to &String

    // ========================================================================
    // SECTION 2: Deref and DerefMut Traits
    // ========================================================================
    println!("\n═══ Section 2: Deref Trait ═══\n");

    // Custom smart pointer
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    let x = MyBox::new(5);
    assert_eq!(5, *x);  // *x desugars to *(x.deref())
    println!("MyBox deref: {}", *x);

    // Deref coercion chain: &MyBox<String> → &String → &str
    let name = MyBox::new(String::from("Rohit"));
    fn hello(name: &str) {
        println!("  Hello, {}!", name);
    }
    hello(&name);  // MyBox<String> → &String → &str (three deref steps!)

    // ========================================================================
    // SECTION 3: Rc<T> — Reference Counting (Deep Dive)
    // ========================================================================
    println!("\n═══ Section 3: Rc<T> Deep Dive ═══\n");

    // Rc<T> — multiple ownership, single-threaded, immutable
    // Like shared_ptr in C++ but without thread safety

    #[derive(Debug)]
    enum SharedList {
        Cons(i32, Rc<SharedList>),
        Nil,
    }

    // Two lists sharing a common tail:
    //  a → 5 → common_tail
    //  b → 3 → common_tail
    //  common_tail → 10 → 20 → Nil
    let common = Rc::new(SharedList::Cons(10,
        Rc::new(SharedList::Cons(20,
            Rc::new(SharedList::Nil)))));

    println!("Common ref count: {}", Rc::strong_count(&common));

    let a = SharedList::Cons(5, Rc::clone(&common));
    println!("After a: count = {}", Rc::strong_count(&common));

    let b = SharedList::Cons(3, Rc::clone(&common));
    println!("After b: count = {}", Rc::strong_count(&common));

    drop(b);
    println!("After drop b: count = {}", Rc::strong_count(&common));

    println!("List a: {:?}", a);

    // ========================================================================
    // SECTION 4: Rc<RefCell<T>> — Shared Mutable State
    // ========================================================================
    println!("\n═══ Section 4: Rc<RefCell<T>> ═══\n");

    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        children: RefCell<Vec<Rc<TreeNode>>>,
        parent: RefCell<Weak<TreeNode>>,
    }

    let leaf = Rc::new(TreeNode {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("Leaf strong={}, weak={}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    let branch = Rc::new(TreeNode {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    // Set leaf's parent to branch (using Weak to avoid cycles!)
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));
    println!("Branch children: {:?}", branch.children.borrow().iter().map(|c| c.value).collect::<Vec<_>>());
    println!("Leaf strong={}, weak={}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("Branch strong={}, weak={}", Rc::strong_count(&branch), Rc::weak_count(&branch));

    // ========================================================================
    // SECTION 5: Weak<T> — Breaking Reference Cycles
    // ========================================================================
    println!("\n═══ Section 5: Weak<T> ═══\n");

    // Without Weak, Rc cycles would cause memory leaks
    // Weak<T> doesn't increment strong count — doesn't prevent deallocation

    let strong = Rc::new(String::from("I exist"));
    let weak = Rc::downgrade(&strong);

    // Upgrade weak to strong (returns Option<Rc<T>>)
    match weak.upgrade() {
        Some(val) => println!("Weak upgraded: {}", val),
        None => println!("Value was dropped"),
    }

    drop(strong);  // Drops the last strong reference

    match weak.upgrade() {
        Some(val) => println!("Weak upgraded: {}", val),
        None => println!("Value was dropped — weak returns None"),
    }

    // ========================================================================
    // SECTION 6: Custom Smart Pointer with Drop
    // ========================================================================
    println!("\n═══ Section 6: Custom Smart Pointer ═══\n");

    struct SmartPointer<T: std::fmt::Debug> {
        data: T,
        name: String,
    }

    impl<T: std::fmt::Debug> SmartPointer<T> {
        fn new(name: &str, data: T) -> Self {
            println!("  SmartPointer '{}' created with {:?}", name, data);
            SmartPointer { data, name: name.to_string() }
        }
    }

    impl<T: std::fmt::Debug> Deref for SmartPointer<T> {
        type Target = T;
        fn deref(&self) -> &T { &self.data }
    }

    impl<T: std::fmt::Debug> Drop for SmartPointer<T> {
        fn drop(&mut self) {
            println!("  SmartPointer '{}' dropped (had {:?})", self.name, self.data);
        }
    }

    {
        let sp1 = SmartPointer::new("first", vec![1, 2, 3]);
        let sp2 = SmartPointer::new("second", "hello");
        println!("  sp1 length: {}", sp1.len());
        println!("  sp2 value: {}", *sp2);
        println!("  About to leave scope...");
    }  // sp2 dropped first, then sp1 (reverse order)

    println!("\n✅ Lesson 11 Complete!");
    println!("   Next: Lesson 12 — Concurrency");
}
