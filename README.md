# lars
A library implementing basic linear algebra in Rust.

---
### Example features of `lars::vector`
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
    // Other operations
    v2 = v0 + v1
    v2 = v0 / v1
    v2 = v0 - v1
    // Creating vector from elements
    let elements = [1.0, 3.0, 3.0, 7.0];
    let foo = vector::from(&elements);
    // Random vector of length 42
    let bar = vector::random(42);
}
```

### Example features of `lars::matrix`
```rust
extern crate lars;
use lars::matrix;
use lars::matrix::Matrix;
fn matrix_stuff() {
    // Create a 5 by 5 matrix of 3.0s
    let m0 = Matrix(5, 5, 3.0);
    let m1 = m0.clone();
    // Multiplying matrices
    let mut m2 = m0 * m1;
    // Other operations
    m2 = m0 + m1;
    m2 = m0 / m1;
    m2 = m0 - m1;
    // Creating 2 by 2 matrix from elements
    let elements = [1.0, 3.0, 3.0, 7.0];
    let foo = matrix::from(2, 2, &elements);
    // Random 5 by 5 matrix
    let bar = matrix::random(5, 5);
    // Create 5 by 5 identity matrix
    let m01 = matrix::identity(5);
    // Create matrix of zeros of dimensions of m0
    let m02 = matrix::zeros_like(m0);
    // Create 5 by 7 matrix of zeros
    let mut m03 = matrix::zeros(5, 8);
    // Get transposed matrix from other matrix
    let barT = matrix::transposed(bar);
    // Transpose m2
    m2.transpose();
    // Reshape m03 (Must have same ammount of elements)
    m03.reshape(2, 20);
    // Get and set elements of m03
    m03.set(1, 7, 69);
    assert_eq!(m03.get(1, 7), 69); // true
}
```

---

# TODO

- Elementwise multiplication between matrix and vector.
