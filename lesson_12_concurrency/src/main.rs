// ============================================================================
// LESSON 12: Concurrency — Fearless and Correct
// ============================================================================
// Rust prevents data races at compile time through ownership + Send/Sync traits.
// ============================================================================

use std::sync::{Arc, Mutex, RwLock, mpsc, Barrier};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  LESSON 12: Concurrency — Fearless and Correct         ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================================================
    // SECTION 1: Spawning Threads
    // ========================================================================
    println!("═══ Section 1: Spawning Threads ═══\n");

    // std::thread::spawn creates OS threads
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  Spawned thread: count {}", i);
            thread::sleep(Duration::from_millis(10));
        }
        42  // Return value from thread
    });

    for i in 1..=3 {
        println!("  Main thread: count {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    // join() waits for thread to finish and gets its return value
    let result = handle.join().unwrap();
    println!("Thread returned: {}\n", result);

    // move closure — transfer ownership to thread
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // `data` was moved into this thread — safe to use!
        println!("Thread owns data: {:?}", data);
        data.iter().sum::<i32>()
    });
    // println!("{:?}", data);  // ❌ data was moved
    println!("Sum from thread: {}", handle.join().unwrap());

    // ========================================================================
    // SECTION 2: Scoped Threads (Rust 1.63+)
    // ========================================================================
    println!("\n═══ Section 2: Scoped Threads ═══\n");

    // Scoped threads can borrow from the parent scope — guaranteed to join!
    let data = vec![1, 2, 3, 4, 5];
    let mut results = vec![];

    thread::scope(|s| {
        // These threads can borrow `data` because scope guarantees they finish
        let t1 = s.spawn(|| {
            let sum: i32 = data.iter().sum();
            println!("  Thread 1: sum = {}", sum);
            sum
        });

        let t2 = s.spawn(|| {
            let product: i32 = data.iter().product();
            println!("  Thread 2: product = {}", product);
            product
        });

        results.push(t1.join().unwrap());
        results.push(t2.join().unwrap());
    });
    // `data` is still valid here — scoped threads only borrowed it
    println!("Results: {:?}, Data still here: {:?}\n", results, data);

    // ========================================================================
    // SECTION 3: Arc<Mutex<T>> — Shared Mutable State
    // ========================================================================
    println!("═══ Section 3: Arc<Mutex<T>> ═══\n");

    // Arc = Atomic Reference Counting (thread-safe Rc)
    // Mutex = Mutual Exclusion (only one thread can access at a time)
    // Arc<Mutex<T>> = thread-safe shared mutable state (like C++ shared_ptr<mutex>+data)

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("  Thread {} incremented to {}", i, *num);
            // Mutex automatically unlocked when `num` goes out of scope (RAII!)
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}\n", *counter.lock().unwrap());

    // ========================================================================
    // SECTION 4: RwLock<T> — Reader-Writer Lock
    // ========================================================================
    println!("═══ Section 4: RwLock<T> ═══\n");

    // RwLock allows multiple readers OR one writer (like C++ shared_mutex)
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    thread::scope(|s| {
        // Multiple readers
        for i in 0..3 {
            let data = Arc::clone(&data);
            s.spawn(move || {
                let read = data.read().unwrap();
                println!("  Reader {}: {:?}", i, *read);
            });
        }

        // Writer (waits for all readers)
        let data = Arc::clone(&data);
        s.spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let mut write = data.write().unwrap();
            write.push(4);
            println!("  Writer added 4");
        });
    });

    println!("Final data: {:?}\n", *data.read().unwrap());

    // ========================================================================
    // SECTION 5: Channels — Message Passing
    // ========================================================================
    println!("═══ Section 5: Channels (mpsc) ═══\n");

    // mpsc = Multiple Producer, Single Consumer
    let (tx, rx) = mpsc::channel();

    // Multiple producers
    for i in 0..5 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let message = format!("msg {} from thread {}", i, thread::current().name().unwrap_or("unnamed"));
            tx_clone.send(message).unwrap();
        });
    }
    drop(tx);  // Drop original sender — only clones remain, then they'll be dropped too

    // Single consumer — receives until all senders are dropped
    println!("Received messages:");
    for received in rx {
        println!("  ← {}", received);
    }

    // Bounded channel with sync_channel
    println!("\nBounded channel:");
    let (tx, rx) = mpsc::sync_channel(2);  // Buffer size 2

    thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
            println!("  Sent: {}", i);
        }
    });

    for val in rx {
        println!("  Received: {}", val);
        thread::sleep(Duration::from_millis(50));
    }

    // ========================================================================
    // SECTION 6: Atomic Types — Lock-Free Operations
    // ========================================================================
    println!("\n═══ Section 6: Atomic Types ═══\n");

    // AtomicU64 — lock-free counter (like C++ std::atomic)
    let atomic_counter = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&atomic_counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Atomic counter: {} (expected: 10000)",
             atomic_counter.load(Ordering::SeqCst));

    // Memory ordering:
    // SeqCst  — strictest, easiest to reason about
    // AcqRel  — for read-modify-write operations
    // Release — for stores (writes)
    // Acquire — for loads (reads)
    // Relaxed — no ordering guarantees (fastest but dangerous)

    // ========================================================================
    // SECTION 7: Barrier — Thread Synchronization Point
    // ========================================================================
    println!("\n═══ Section 7: Barrier ═══\n");

    let barrier = Arc::new(Barrier::new(5));
    let mut handles = vec![];

    for i in 0..5 {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("  Thread {} started", i);
            thread::sleep(Duration::from_millis(i as u64 * 100));
            println!("  Thread {} waiting at barrier", i);
            barrier.wait();  // All threads wait here until all 5 arrive
            println!("  Thread {} passed barrier!", i);
        }));
    }

    for h in handles { h.join().unwrap(); }

    // ========================================================================
    // SECTION 8: Thread Pool Pattern
    // ========================================================================
    println!("\n═══ Section 8: Simple Thread Pool ═══\n");

    // Simple thread pool using channels
    let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
    let rx = Arc::new(Mutex::new(rx));

    // Create worker threads
    let mut workers = vec![];
    for id in 0..4 {
        let rx = Arc::clone(&rx);
        workers.push(thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(task) => {
                        println!("  Worker {} executing task", id);
                        task();
                    }
                    Err(_) => {
                        println!("  Worker {} shutting down", id);
                        break;
                    }
                }
            }
        }));
    }

    // Submit jobs
    for i in 0..8 {
        let tx = tx.clone();
        tx.send(Box::new(move || {
            thread::sleep(Duration::from_millis(50));
            println!("    Task {} completed", i);
        })).unwrap();
    }

    drop(tx);  // Signal workers to shut down
    for w in workers { w.join().unwrap(); }

    println!("\n✅ Lesson 12 Complete!");
    println!("   Next: Lesson 13 — Async/Await");
}
