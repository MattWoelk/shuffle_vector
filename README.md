# Shuffle Vector

A rust implementation of a Shuffle Vector, as described by <https://beta.observablehq.com/@jobleonard/shuffle-vectors>.

```rust
let mut v = ShuffleVector::new (vec!());

v.push(1); // [1]
v.push(2); // [2, 1]
v.push(3); // [2, 3, 1]
v.push(4); // [4, 3, 1, 2]
v.push(5); // [4, 3, 5, 2, 1]

dbg!(v.clone()); // [4, 3, 5, 2, 1]

dbg!(v.pop()); // 1
dbg!(v.pop()); // 2
dbg!(v.pop()); // 5

dbg!(v.clone()); // [4, 3]
```