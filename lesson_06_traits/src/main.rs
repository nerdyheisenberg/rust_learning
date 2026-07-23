// ============================================================================
// LESSON 6: Traits — Rust's Polymorphism
// ============================================================================

use std::fmt;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 6: Traits — Rust's Polymorphism                ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Defining and Implementing Traits
    // ========================================================================
    println!("═══ Section 1: Basic Traits ═══\n");

    // Define a trait — like an interface/abstract class
    trait Drawable {
        fn draw(&self);  // Required method — must be implemented

        fn description(&self) -> String {  // Default method — optional override
            String::from("A drawable shape")
        }
    }

    // Implement Drawable for Circle
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Drawable for Circle {
        fn draw(&self) {
            println!("  Drawing circle at ({}, {}) with radius {}", self.x, self.y, self.radius);
        }

        fn description(&self) -> String {
            format!("Circle(r={})", self.radius)
        }
    }

    // Implement Drawable for Square
    struct Square {
        x: f64,
        y: f64,
        side: f64,
    }

    impl Drawable for Square {
        fn draw(&self) {
            println!("  Drawing square at ({}, {}) with side {}", self.x, self.y, self.side);
        }
        // Uses default description() implementation
    }

    let circle = Circle { x: 0.0, y: 0.0, radius: 5.0 };
    let square = Square { x: 1.0, y: 1.0, side: 3.0 };

    circle.draw();
    println!("  Description: {}", circle.description());
    square.draw();
    println!("  Description: {}", square.description());

    // ========================================================================
    // SECTION 2: Trait Bounds — Static Dispatch
    // ========================================================================
    println!("\n═══ Section 2: Trait Bounds (Static Dispatch) ═══\n");

    // Generic function with trait bound — monomorphized at compile time
    fn print_description<T: Drawable>(item: &T) {
        println!("  → {}", item.description());
        item.draw();
    }

    print_description(&circle);
    print_description(&square);

    // Multiple trait bounds with + syntax:
    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    // where clause — more readable for complex bounds
    fn print_shape_info<T>(shape: &T)
    where
        T: Drawable + HasArea + fmt::Debug,
    {
        println!("  Shape: {:?}", shape);
        println!("  Area: {:.2}", shape.area());
        shape.draw();
    }

    // Need Debug for the constraint
    impl fmt::Debug for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle(c=({},{}), r={})", self.x, self.y, self.radius)
        }
    }

    impl fmt::Debug for Square {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Square(c=({},{}), s={})", self.x, self.y, self.side)
        }
    }

    print_shape_info(&circle);
    print_shape_info(&square);

    // ========================================================================
    // SECTION 3: Trait Objects — Dynamic Dispatch (dyn Trait)
    // ========================================================================
    println!("\n═══ Section 3: Dynamic Dispatch (dyn Trait) ═══\n");

    // Trait objects allow heterogeneous collections (like C++ virtual)
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { x: 0.0, y: 0.0, radius: 3.0 }),
        Box::new(Square { x: 1.0, y: 1.0, side: 4.0 }),
        Box::new(Circle { x: 5.0, y: 5.0, radius: 1.0 }),
    ];

    println!("Drawing all shapes:");
    for shape in &shapes {
        shape.draw();
    }

    // Size of trait objects (fat pointers):
    println!("\nSizes:");
    println!("  Box<Circle>: {} bytes", std::mem::size_of::<Box<Circle>>());
    println!("  Box<dyn Drawable>: {} bytes (fat pointer: data + vtable)",
             std::mem::size_of::<Box<dyn Drawable>>());

    // Function accepting trait object
    fn draw_shape(shape: &dyn Drawable) {
        shape.draw();
    }

    draw_shape(&circle);
    draw_shape(&square);

    // ========================================================================
    // SECTION 4: Standard Library Traits
    // ========================================================================
    println!("\n═══ Section 4: Standard Library Traits ═══\n");

    // Display — for user-facing output (like operator<< in C++)
    #[derive(Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
        }
    }

    let p = Point { x: 1.0, y: 2.0 };
    println!("Display: {}", p);     // Uses Display
    println!("Debug: {:?}", p);     // Uses Debug

    // Default trait — provides default values (like default constructor)
    impl Default for Point {
        fn default() -> Self {
            Point { x: 0.0, y: 0.0 }
        }
    }

    let origin = Point::default();
    println!("Default point: {}", origin);

    // From/Into — type conversions
    impl From<(f64, f64)> for Point {
        fn from(tuple: (f64, f64)) -> Self {
            Point { x: tuple.0, y: tuple.1 }
        }
    }

    let p: Point = (3.0, 4.0).into();  // Uses Into (auto-derived from From)
    let p2 = Point::from((5.0, 6.0));   // Uses From directly
    println!("From tuple: {}, {}", p, p2);

    // PartialEq and Eq — equality comparison
    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            (self.x - other.x).abs() < f64::EPSILON
                && (self.y - other.y).abs() < f64::EPSILON
        }
    }

    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 1.0, y: 2.0 };
    let c = Point { x: 3.0, y: 4.0 };
    println!("a == b: {}", a == b);
    println!("a == c: {}", a == c);

    // ========================================================================
    // SECTION 5: Operator Overloading via Traits
    // ========================================================================
    println!("\n═══ Section 5: Operator Overloading ═══\n");

    use std::ops::{Add, Mul, Neg};

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Neg for Point {
        type Output = Point;

        fn neg(self) -> Point {
            Point { x: -self.x, y: -self.y }
        }
    }

    impl Mul<f64> for Point {
        type Output = Point;

        fn mul(self, scalar: f64) -> Point {
            Point {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }
    }

    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let sum = p1.clone() + p2;
    println!("p1 + p2 = {}", sum);
    println!("-p1 = {}", -p1.clone());
    println!("p1 * 3 = {}", p1 * 3.0);

    // ========================================================================
    // SECTION 6: Supertraits (Trait Inheritance)
    // ========================================================================
    println!("\n═══ Section 6: Supertraits ═══\n");

    // A trait can require another trait be implemented:
    trait Animal: fmt::Display {  // Animal requires Display
        fn name(&self) -> &str;
        fn sound(&self) -> &str;

        fn info(&self) -> String {
            format!("{} says {}", self.name(), self.sound())
        }
    }

    struct Dog {
        name: String,
    }

    impl fmt::Display for Dog {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "🐕 {}", self.name)
        }
    }

    impl Animal for Dog {
        fn name(&self) -> &str { &self.name }
        fn sound(&self) -> &str { "Woof!" }
    }

    let dog = Dog { name: String::from("Rex") };
    println!("{}", dog);           // Display
    println!("{}", dog.info());    // Animal method

    // ========================================================================
    // SECTION 7: Associated Types vs Generic Parameters
    // ========================================================================
    println!("\n═══ Section 7: Associated Types ═══\n");

    // Associated type — ONE implementation per type
    trait Container {
        type Item;  // Associated type — determined by implementor
        fn first(&self) -> Option<&Self::Item>;
        fn last(&self) -> Option<&Self::Item>;
        fn len(&self) -> usize;
    }

    struct NumberList {
        items: Vec<i32>,
    }

    impl Container for NumberList {
        type Item = i32;  // NumberList's Container uses i32

        fn first(&self) -> Option<&i32> { self.items.first() }
        fn last(&self) -> Option<&i32> { self.items.last() }
        fn len(&self) -> usize { self.items.len() }
    }

    let list = NumberList { items: vec![10, 20, 30] };
    println!("First: {:?}, Last: {:?}, Len: {}",
             list.first(), list.last(), list.len());

    // Compare: Generic parameter — MULTIPLE implementations per type
    trait Convertible<T> {
        fn convert(&self) -> T;
    }

    struct Value(f64);

    impl Convertible<String> for Value {
        fn convert(&self) -> String { format!("{:.2}", self.0) }
    }

    impl Convertible<i32> for Value {
        fn convert(&self) -> i32 { self.0 as i32 }
    }

    let v = Value(42.7);
    let s: String = v.convert();
    let i: i32 = v.convert();
    println!("As String: {}, As i32: {}", s, i);

    // ========================================================================
    // SECTION 8: Derive Macros for Traits
    // ========================================================================
    println!("\n═══ Section 8: Derive Macros ═══\n");

    // #[derive(...)] automatically implements common traits:
    #[derive(Debug, Clone, PartialEq, Default)]
    struct Config {
        width: u32,
        height: u32,
        fullscreen: bool,
    }

    let c1 = Config { width: 1920, height: 1080, fullscreen: true };
    let c2 = c1.clone();
    let c3 = Config::default();

    println!("c1: {:?}", c1);
    println!("c1 == c2: {}", c1 == c2);
    println!("Default: {:?}", c3);

    // Derivable traits: Debug, Clone, Copy, PartialEq, Eq,
    //                   PartialOrd, Ord, Hash, Default

    println!("\n✅ Lesson 6 Complete!");
    println!("   Next: Lesson 7 — Generics & Type System Mastery");
}
