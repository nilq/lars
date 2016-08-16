# lars
A library implementing basic linear algebra in Rust.

---
### Example use
```rust
extern crate lars;
use lars::vector;
use lars::vector:Vector;
fn vector_stuff() {
    // Create 5 dimensional vector of 3.0s
    let v0 = Vector(5, 3.0);
    let v1 = v0.clone()
    // Multiplying vectors
    let mut v2 = v0 * v1
    // Other operators
    v2 = v0 + v1
    v2 = v0 / v1
    v2 = v0 - v1
    // Createing vector from elements
    let elements = [1.0, 3.0, 3.0, 7.0];
    let foo = vector::from(&elements);
    // Random vector of length 42
    let bar = vector::random(42);
}
```
