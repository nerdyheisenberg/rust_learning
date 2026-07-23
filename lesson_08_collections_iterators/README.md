# Lesson 8: Collections, Iterators & Functional Programming

## 📚 Theory

### Iterator Protocol
The `Iterator` trait has one required method: `fn next(&mut self) -> Option<Self::Item>`.
Returns `Some(item)` for each element, `None` when exhausted. Lazy evaluation — no
work happens until you consume the iterator.

### Three Iteration Methods
- `iter()` → `&T` references (borrows)
- `iter_mut()` → `&mut T` (mutable borrows)
- `into_iter()` → `T` (takes ownership, consumes collection)

### Zero-Cost Abstraction Proof
Iterator chains like `.filter().map().collect()` compile to the SAME assembly as
hand-written loops. The LLVM optimizer fuses all iterator adapters into one pass.

### Key Collections
- `Vec<T>` — contiguous growable array (like C++ vector)
- `HashMap<K,V>` — hash table (like C++ unordered_map)
- `BTreeMap<K,V>` — sorted tree (like C++ map)
- `HashSet<T>` — unique values (like C++ unordered_set)
- `VecDeque<T>` — double-ended queue (like C++ deque)
