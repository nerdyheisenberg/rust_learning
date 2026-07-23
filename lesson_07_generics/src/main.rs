// ============================================================================
// LESSON 7: Generics & Type System Mastery
// ============================================================================

use std::fmt;
use std::marker::PhantomData;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 7: Generics & Type System Mastery              ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Generic Functions
    // ========================================================================
    println!("═══ Section 1: Generic Functions ═══\n");

    // Generic function with trait bound
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in &list[1..] {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", chars.iter().max().unwrap());

    // Multiple type parameters
    fn zip_with<A, B, C, F>(a: A, b: B, f: F) -> C
    where
        F: Fn(A, B) -> C,
    {
        f(a, b)
    }

    let result = zip_with(5, 3.0, |a: i32, b: f64| format!("{} + {} = {}", a, b, a as f64 + b));
    println!("zip_with: {}", result);

    // ========================================================================
    // SECTION 2: Generic Structs
    // ========================================================================
    println!("\n═══ Section 2: Generic Structs ═══\n");

    #[derive(Debug)]
    struct Pair<T, U> {
        first: T,
        second: U,
    }

    impl<T: fmt::Display, U: fmt::Display> Pair<T, U> {
        fn new(first: T, second: U) -> Self {
            Pair { first, second }
        }

        fn display(&self) {
            println!("  ({}, {})", self.first, self.second);
        }
    }

    // Conditional implementation — only for Pairs where T: PartialOrd
    impl<T: fmt::Display + PartialOrd, U: fmt::Display> Pair<T, U> {
        fn cmp_display_first(&self, other: &T) {
            if self.first > *other {
                println!("  First ({}) is greater than {}", self.first, other);
            } else {
                println!("  First ({}) is NOT greater than {}", self.first, other);
            }
        }
    }

    let pair = Pair::new(42, "hello");
    pair.display();
    pair.cmp_display_first(&50);

    // Generic enum
    #[derive(Debug)]
    enum Either<L, R> {
        Left(L),
        Right(R),
    }

    impl<L: fmt::Display, R: fmt::Display> fmt::Display for Either<L, R> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Either::Left(val) => write!(f, "Left({})", val),
                Either::Right(val) => write!(f, "Right({})", val),
            }
        }
    }

    let e1: Either<i32, String> = Either::Left(42);
    let e2: Either<i32, String> = Either::Right(String::from("hello"));
    println!("  e1: {}", e1);
    println!("  e2: {}", e2);

    // ========================================================================
    // SECTION 3: Const Generics
    // ========================================================================
    println!("\n═══ Section 3: Const Generics ═══\n");

    // Const generics — parameterize by VALUES, not just types
    // Like C++ template<int N>
    #[derive(Debug)]
    struct FixedArray<T, const N: usize> {
        data: [T; N],
    }

    impl<T: Default + Copy + fmt::Debug, const N: usize> FixedArray<T, N> {
        fn new() -> Self {
            FixedArray {
                data: [T::default(); N],
            }
        }

        fn set(&mut self, index: usize, value: T) {
            assert!(index < N, "Index out of bounds");
            self.data[index] = value;
        }

        fn get(&self, index: usize) -> &T {
            &self.data[index]
        }

        fn size(&self) -> usize {
            N
        }
    }

    let mut arr: FixedArray<i32, 5> = FixedArray::new();
    arr.set(0, 10);
    arr.set(1, 20);
    arr.set(2, 30);
    println!("FixedArray<i32, 5>: {:?} (size: {})", arr.data, arr.size());
    println!("  Element 1: {}", arr.get(1));

    // Const generics in functions
    fn print_array<T: fmt::Debug, const N: usize>(arr: &[T; N]) {
        println!("  Array of {} elements: {:?}", N, arr);
    }

    print_array(&[1, 2, 3]);
    print_array(&[1, 2, 3, 4, 5]);

    // Matrix with const generics
    fn dot_product<const N: usize>(a: &[f64; N], b: &[f64; N]) -> f64 {
        let mut sum = 0.0;
        for i in 0..N {
            sum += a[i] * b[i];
        }
        sum
    }

    let a = [1.0, 2.0, 3.0];
    let b = [4.0, 5.0, 6.0];
    println!("  Dot product: {}", dot_product(&a, &b));

    // ========================================================================
    // SECTION 4: PhantomData & Phantom Types
    // ========================================================================
    println!("\n═══ Section 4: PhantomData ═══\n");

    // PhantomData is a zero-sized marker type
    // It tells the compiler "I logically own/refer to this type"

    // Example: Type-safe distance units
    struct Meters;
    struct Feet;

    #[derive(Debug)]
    struct Distance<Unit> {
        value: f64,
        _unit: PhantomData<Unit>,  // Zero-sized! No runtime cost
    }

    impl<Unit> Distance<Unit> {
        fn new(value: f64) -> Self {
            Distance {
                value,
                _unit: PhantomData,
            }
        }
    }

    impl Distance<Meters> {
        fn to_feet(self) -> Distance<Feet> {
            Distance::new(self.value * 3.28084)
        }
    }

    impl Distance<Feet> {
        fn to_meters(self) -> Distance<Meters> {
            Distance::new(self.value / 3.28084)
        }
    }

    let d_meters = Distance::<Meters>::new(100.0);
    let d_feet = d_meters.to_feet();
    println!("100 meters = {:.2} feet", d_feet.value);
    let back = d_feet.to_meters();
    println!("Back to meters: {:.2}", back.value);
    // COMPILE ERROR if you try to add Meters to Feet!

    println!("Size of Distance<Meters>: {} bytes (PhantomData is zero-sized!)",
             std::mem::size_of::<Distance<Meters>>());

    // ========================================================================
    // SECTION 5: Typestate Pattern
    // ========================================================================
    println!("\n═══ Section 5: Typestate Pattern ═══\n");

    // Encode state machine in the type system — invalid states are compile errors!

    struct Locked;
    struct Unlocked;

    struct Door<State> {
        _state: PhantomData<State>,
    }

    impl Door<Locked> {
        fn new() -> Self {
            println!("  Created a locked door");
            Door { _state: PhantomData }
        }

        fn unlock(self) -> Door<Unlocked> {
            println!("  🔓 Door unlocked");
            Door { _state: PhantomData }
        }
    }

    impl Door<Unlocked> {
        fn lock(self) -> Door<Locked> {
            println!("  🔒 Door locked");
            Door { _state: PhantomData }
        }

        fn open(&self) {
            println!("  🚪 Door opened");
        }
    }

    let door = Door::<Locked>::new();
    // door.open();  // ❌ COMPILE ERROR: Can't open a locked door!
    let door = door.unlock();
    door.open();     // ✅ Can open an unlocked door
    let _door = door.lock();
    // _door.open(); // ❌ COMPILE ERROR: Locked again!

    // ========================================================================
    // SECTION 6: Newtype Pattern
    // ========================================================================
    println!("\n═══ Section 6: Newtype Pattern ═══\n");

    // Wrap a type to give it different meaning and prevent mixing

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct UserId(u64);

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct OrderId(u64);

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Email(u64);

    fn get_user(id: UserId) -> String {
        format!("User #{}", id.0)
    }

    fn get_order(id: OrderId) -> String {
        format!("Order #{}", id.0)
    }

    let user_id = UserId(42);
    let order_id = OrderId(42);

    println!("{}", get_user(user_id));
    println!("{}", get_order(order_id));
    // get_user(order_id);  // ❌ COMPILE ERROR: Can't pass OrderId as UserId!

    // Even though both are u64 internally, the type system prevents mistakes

    // ========================================================================
    // SECTION 7: impl Trait in Function Signatures
    // ========================================================================
    println!("\n═══ Section 7: impl Trait ═══\n");

    // impl Trait in arguments (sugar for generic bound)
    fn print_displayable(item: &impl fmt::Display) {
        println!("  Displaying: {}", item);
    }

    print_displayable(&42);
    print_displayable(&"hello");
    print_displayable(&3.14);

    // impl Trait in return position — existential type
    fn make_greeting(name: &str) -> impl fmt::Display {
        format!("Hello, {}! Welcome to Rust.", name)
    }

    let greeting = make_greeting("Rohit");
    println!("  {}", greeting);

    // Returning closures with impl
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }

    let add5 = make_adder(5);
    println!("  add5(3) = {}", add5(3));

    // ========================================================================
    // SECTION 8: Type Aliases
    // ========================================================================
    println!("\n═══ Section 8: Type Aliases ═══\n");

    // Type alias for complex types
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    fn parse_number(s: &str) -> Result<i32> {
        let n = s.parse::<i32>()?;
        Ok(n * 2)
    }

    match parse_number("21") {
        Ok(n) => println!("  Parsed: {}", n),
        Err(e) => println!("  Error: {}", e),
    }

    // Type alias for function pointers
    type MathOp = fn(f64, f64) -> f64;

    fn apply(op: MathOp, a: f64, b: f64) -> f64 {
        op(a, b)
    }

    let add: MathOp = |a, b| a + b;
    let mul: MathOp = |a, b| a * b;
    println!("  add(3, 4) = {}", apply(add, 3.0, 4.0));
    println!("  mul(3, 4) = {}", apply(mul, 3.0, 4.0));

    println!("\n✅ Lesson 7 Complete!");
    println!("   Next: Lesson 8 — Collections, Iterators & Functional Programming");
}
