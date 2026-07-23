// ============================================================================
// LESSON 13: Async/Await — Asynchronous Programming
// ============================================================================
// NOTE: This lesson requires adding tokio to Cargo.toml:
//   [dependencies]
//   tokio = { version = "1", features = ["full"] }
// ============================================================================
// Since we aren't adding external deps for this lesson, we demonstrate the
// CONCEPTS using std::future and explain how tokio works underneath.
// ============================================================================

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};
use std::time::Duration;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 13: Async/Await — Asynchronous Programming     ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Understanding Futures
    // ========================================================================
    println!("═══ Section 1: The Future Trait ═══\n");

    // In Rust, async code is based on the Future trait:
    //
    // trait Future {
    //     type Output;
    //     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    // }
    //
    // Poll is: enum Poll<T> { Ready(T), Pending }
    //
    // KEY INSIGHT: Futures are LAZY — they do NOTHING until polled!
    // This is different from Go goroutines or JS promises which start immediately.

    println!("  The Future trait: Lazy evaluation");
    println!("  - Ready(T)  — the future completed with value T");
    println!("  - Pending   — the future is not done yet, will wake later");
    println!();

    // ========================================================================
    // SECTION 2: How async/await Compiles
    // ========================================================================
    println!("═══ Section 2: async/await State Machine ═══\n");

    // When you write:
    //   async fn example() -> i32 {
    //       let a = step_one().await;
    //       let b = step_two(a).await;
    //       a + b
    //   }
    //
    // The compiler generates a STATE MACHINE:
    //   enum ExampleFuture {
    //       State0,                        // Initial state
    //       State1 { step_one_future },     // Awaiting step_one
    //       State2 { a, step_two_future },  // Awaiting step_two
    //       Completed,                      // Done
    //   }

    println!("  async fn → compiler generates a state machine enum");
    println!("  Each .await point = a new state in the enum");
    println!("  The state machine implements Future trait");
    println!("  poll() advances the state machine one step\n");

    // ========================================================================
    // SECTION 3: Building a Simple Executor
    // ========================================================================
    println!("═══ Section 3: Simple Executor ═══\n");

    // A minimal future that counts down
    struct Countdown {
        count: u32,
    }

    impl Future for Countdown {
        type Output = String;

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<String> {
            if self.count == 0 {
                Poll::Ready(String::from("Countdown complete!"))
            } else {
                println!("  Countdown: {} remaining", self.count);
                self.count -= 1;
                // In a real async runtime, we'd register with a reactor
                // and call cx.waker().wake() when ready
                Poll::Pending
            }
        }
    }

    // Minimal executor — polls futures repeatedly
    fn block_on<F: Future>(mut future: F) -> F::Output {
        // Create a no-op waker (real executors use proper wakers)
        fn dummy_raw_waker() -> RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
            let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
            RawWaker::new(std::ptr::null(), vtable)
        }

        let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
        let mut cx = Context::from_waker(&waker);
        let mut future = unsafe { Pin::new_unchecked(&mut future) };

        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(result) => return result,
                Poll::Pending => {
                    std::thread::sleep(Duration::from_millis(100));
                    // In real executor: park thread until waker is called
                }
            }
        }
    }

    let result = block_on(Countdown { count: 3 });
    println!("  Result: {}\n", result);

    // ========================================================================
    // SECTION 4: async/await Patterns (Conceptual)
    // ========================================================================
    println!("═══ Section 4: Async Patterns (with tokio) ═══\n");

    // The following shows what you'd write with tokio:
    println!("  // Basic async function");
    println!("  async fn fetch_data(url: &str) -> Result<String, Error> {{");
    println!("      let response = reqwest::get(url).await?;");
    println!("      let body = response.text().await?;");
    println!("      Ok(body)");
    println!("  }}\n");

    println!("  // Concurrent execution with join!");
    println!("  async fn concurrent() {{");
    println!("      let (a, b, c) = tokio::join!(");
    println!("          fetch_data(\"url1\"),");
    println!("          fetch_data(\"url2\"),");
    println!("          fetch_data(\"url3\"),");
    println!("      );");
    println!("  }}\n");

    println!("  // Select — race multiple futures");
    println!("  tokio::select! {{");
    println!("      val = future1 => println!(\"future1 won: {{}}\", val),");
    println!("      val = future2 => println!(\"future2 won: {{}}\", val),");
    println!("  }}\n");

    println!("  // Spawning concurrent tasks");
    println!("  let handle = tokio::spawn(async {{");
    println!("      expensive_computation().await");
    println!("  }});\n");

    // ========================================================================
    // SECTION 5: Async vs Threads Comparison
    // ========================================================================
    println!("═══ Section 5: Async vs Threads ═══\n");

    println!("  ┌──────────────────────────────────────────────────┐");
    println!("  │  Threads vs Async Comparison                     │");
    println!("  ├──────────────┬───────────────────────────────────┤");
    println!("  │ Threads      │ Async/Await                      │");
    println!("  ├──────────────┼───────────────────────────────────┤");
    println!("  │ OS-managed   │ User-space (runtime: tokio)      │");
    println!("  │ ~8MB stack   │ ~few KB per task                  │");
    println!("  │ Preemptive   │ Cooperative                      │");
    println!("  │ Good for CPU │ Good for I/O                     │");
    println!("  │ ~1000s max   │ ~millions possible               │");
    println!("  │ Simple code  │ Colored functions                │");
    println!("  └──────────────┴───────────────────────────────────┘\n");

    // ========================================================================
    // SECTION 6: Pin and Unpin Explained
    // ========================================================================
    println!("═══ Section 6: Pin and Unpin ═══\n");

    // Pin<T> guarantees that the data it points to won't be moved in memory.
    // This is CRITICAL for async because compiled state machines may contain
    // self-references (a field pointing to another field in same struct).
    //
    // If the struct moved, self-references would point to old (invalid) memory!
    //
    // Most types are `Unpin` (can be moved freely even when pinned).
    // Only self-referential types (and futures) need to be `!Unpin`.

    println!("  Pin<T> prevents moving data after it's been pinned");
    println!("  Required for self-referential types (async state machines)");
    println!("  Most types are Unpin (Pin has no effect on them)");
    println!("  Futures generated by async fn are !Unpin\n");

    // Examples of pinning:
    let mut value = 42;
    let pinned = Pin::new(&mut value);  // Safe because i32 is Unpin
    println!("  Pinned value: {}", *pinned);

    // Box::pin for heap allocation + pinning
    let boxed_pinned: Pin<Box<i32>> = Box::pin(42);
    println!("  Box::pin value: {}", *boxed_pinned);

    // ========================================================================
    // SECTION 7: Tokio Cargo.toml Reference
    // ========================================================================
    println!("═══ Section 7: Tokio Setup Reference ═══\n");

    println!("  To use async Rust in practice, add to Cargo.toml:");
    println!("  [dependencies]");
    println!("  tokio = {{ version = \"1\", features = [\"full\"] }}");
    println!();
    println!("  Then use #[tokio::main] macro:");
    println!("  #[tokio::main]");
    println!("  async fn main() {{");
    println!("      let result = my_async_fn().await;");
    println!("  }}");

    println!("\n✅ Lesson 13 Complete!");
    println!("   Next: Lesson 14 — Unsafe Rust & FFI");
}
