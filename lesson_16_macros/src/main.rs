// ============================================================================
// LESSON 16: Macros — Metaprogramming in Rust
// ============================================================================
// Declarative macros (macro_rules!) — pattern matching on syntax trees
// Procedural macros — code that generates code (not covered hands-on here
//   because they require separate crate, but theory is explained)
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 16: Macros — Metaprogramming in Rust           ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Why Macros?
    // ========================================================================
    println!("═══ Section 1: Why Macros? ═══\n");

    // Macros generate code at COMPILE TIME
    // They can do things functions can't:
    // 1. Accept variable number of arguments
    // 2. Generate boilerplate code
    // 3. Create DSLs (domain-specific languages)
    // 4. Implement traits automatically
    //
    // Rust macros are HYGIENIC — they don't accidentally capture variables
    // from the surrounding scope (unlike C preprocessor macros)

    println!("  Macros expand at compile time");
    println!("  They are hygienic (no accidental variable capture)");
    println!("  They can accept variable arguments\n");

    // ========================================================================
    // SECTION 2: Basic Declarative Macros
    // ========================================================================
    println!("═══ Section 2: Basic macro_rules! ═══\n");

    // Simple macro — like a function but with pattern matching on syntax
    macro_rules! say_hello {
        () => {
            println!("  Hello from macro!");
        };
        ($name:expr) => {
            println!("  Hello, {}!", $name);
        };
    }

    say_hello!();
    say_hello!("Rohit");

    // Macro with different syntax forms
    macro_rules! create_function {
        ($name:ident) => {
            fn $name() {
                println!("  Function `{}` was created by a macro!", stringify!($name));
            }
        };
    }

    create_function!(foo);
    create_function!(bar);
    foo();
    bar();

    // ========================================================================
    // SECTION 3: Fragment Specifiers
    // ========================================================================
    println!("\n═══ Section 3: Fragment Specifiers ═══\n");

    // Available fragment specifiers:
    // $x:expr   — expression (5 + 3, "hello", function_call())
    // $x:ident  — identifier (variable_name, function_name)
    // $x:ty     — type (i32, String, Vec<T>)
    // $x:pat    — pattern (Some(x), (a, b), _)
    // $x:stmt   — statement (let x = 5;)
    // $x:block  — block ({ ... })
    // $x:item   — item (fn, struct, impl)
    // $x:path   — path (std::io::Result)
    // $x:tt     — token tree (any single token or {...})
    // $x:literal — literal value (42, "hello", true)
    // $x:meta   — meta item (cfg(test), derive(Debug))

    macro_rules! print_type {
        ($val:expr, $type:ty) => {
            println!("  {} as {} = {}", stringify!($val), stringify!($type), $val as $type);
        };
    }

    print_type!(42, f64);
    print_type!(3.14, i32);

    // ========================================================================
    // SECTION 4: Repetitions — Variable Arguments
    // ========================================================================
    println!("\n═══ Section 4: Repetitions ═══\n");

    // $(...),* — zero or more, comma separated
    // $(...),+ — one or more, comma separated
    // $(...);* — zero or more, semicolon separated

    // vec!-like macro
    macro_rules! my_vec {
        () => {
            Vec::new()
        };
        ($($element:expr),+ $(,)?) => {
            {
                let mut v = Vec::new();
                $(v.push($element);)+
                v
            }
        };
    }

    let v = my_vec![1, 2, 3, 4, 5];
    println!("my_vec: {:?}", v);

    let empty: Vec<i32> = my_vec![];
    println!("my_vec empty: {:?}", empty);

    // HashMap creation macro
    macro_rules! hashmap {
        ($($key:expr => $value:expr),* $(,)?) => {
            {
                let mut map = std::collections::HashMap::new();
                $(map.insert($key, $value);)*
                map
            }
        };
    }

    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
        "Charlie" => 92,
    };
    println!("hashmap: {:?}", scores);

    // ========================================================================
    // SECTION 5: Recursive Macros
    // ========================================================================
    println!("\n═══ Section 5: Recursive Macros ═══\n");

    // Macros can call themselves recursively
    macro_rules! count {
        () => { 0usize };
        ($head:tt $($tail:tt)*) => { 1usize + count!($($tail)*) };
    }

    println!("count!(a b c d e) = {}", count!(a b c d e));

    // Compile-time min/max
    macro_rules! max {
        ($x:expr) => ($x);
        ($x:expr, $($rest:expr),+) => {
            {
                let x = $x;
                let rest = max!($($rest),+);
                if x > rest { x } else { rest }
            }
        };
    }

    println!("max(3, 7, 2, 9, 1) = {}", max!(3, 7, 2, 9, 1));

    // ========================================================================
    // SECTION 6: Debug and Utility Macros
    // ========================================================================
    println!("\n═══ Section 6: Utility Macros ═══\n");

    // debug macro — prints expression and its value
    macro_rules! dbg_custom {
        ($val:expr) => {
            {
                let result = $val;
                println!("  [{}:{}] {} = {:?}", file!(), line!(), stringify!($val), result);
                result
            }
        };
    }

    let x = dbg_custom!(5 + 3);
    let y = dbg_custom!(x * 2);
    let _ = dbg_custom!(vec![1, 2, 3]);
    let _ = y;

    // Timer macro
    macro_rules! time_it {
        ($name:expr, $block:block) => {
            {
                let start = std::time::Instant::now();
                let result = $block;
                let elapsed = start.elapsed();
                println!("  {} took {:?}", $name, elapsed);
                result
            }
        };
    }

    let sum = time_it!("sum computation", {
        (0..1_000_000).sum::<i64>()
    });
    println!("  Sum: {}", sum);

    // ========================================================================
    // SECTION 7: Macro Export and Use
    // ========================================================================
    println!("\n═══ Section 7: Macro Export ═══\n");

    // #[macro_export] makes macro available outside the crate
    // Macros are always at the crate root when exported

    // Macro use in match arms — pattern generating macro
    macro_rules! match_day {
        ($day:expr) => {
            match $day {
                1 => "Monday",
                2 => "Tuesday",
                3 => "Wednesday",
                4 => "Thursday",
                5 => "Friday",
                6 => "Saturday",
                7 => "Sunday",
                _ => "Invalid day",
            }
        };
    }

    for day in 1..=8 {
        println!("  Day {}: {}", day, match_day!(day));
    }

    // ========================================================================
    // SECTION 8: Procedural Macros (Theory)
    // ========================================================================
    println!("\n═══ Section 8: Procedural Macros (Theory) ═══\n");

    // Procedural macros are Rust functions that take TokenStream as input
    // and produce TokenStream as output. They require a separate crate.
    //
    // Three types:
    // 1. Derive macros: #[derive(MyTrait)]
    //    - Auto-implement traits based on struct/enum structure
    //    - Examples: serde's #[derive(Serialize, Deserialize)]
    //
    // 2. Attribute macros: #[my_attribute]
    //    - Transform items (functions, structs, etc.)
    //    - Examples: #[tokio::main], #[test]
    //
    // 3. Function-like macros: my_macro!(...)
    //    - Like declarative macros but with full Rust power
    //    - Can parse arbitrary syntax

    println!("  Procedural macros require separate crate (proc-macro = true)");
    println!("  Tools: syn (parsing), quote (code generation)");
    println!("  ");
    println!("  // Example derive macro structure:");
    println!("  #[proc_macro_derive(MyTrait)]");
    println!("  pub fn my_trait_derive(input: TokenStream) -> TokenStream {{");
    println!("      let ast = syn::parse(input).unwrap();");
    println!("      // Generate implementation...");
    println!("      quote! {{ impl MyTrait for #name {{ }} }}.into()");
    println!("  }}");

    println!("\n✅ Lesson 16 Complete!");
    println!("   Next: Lesson 17 — Advanced Type System & Design Patterns");
}
