//! The Matrix type overloads all basic arithmetic operations: addition, subtraction,
//! multiplication, division - along with the possibility of getting negative and index
//! in/of given matrix. Matrix also provides fancy methods for e.g. getting and setting elements.
//!
//! # Example creation
//! ```
//! use lars::matrix;
//! use lars::matrix::Matrix;
//!
//! fn matrix_creation() {
//!     // Create 3 by 5 matrix of 3.0s
//!     let m1 = Matrix::new(3, 5, 3.0);
//!
//!     // Create 3 by 5 matrix of given elements
//!     let elements = [1.0, 3.0, 3.0, 7.0];
//!     let m2 = matrix::from(3, 5, &elements);
//!
//!     // Create identity matrix of size 5
//!     let m3 = matrix::identity(5);
//!
//!     // Create 3 by 5 random matrix
//!     let m4 = matrix::random(3, 5);
//!
//!     // Create 3 by 5 matrix of 0s
//!     let m5 = matrix::zeros(3, 5);
//!
//!     // Create matrix of zeros of dimension based on other matrix
//!     let m6 = matrix::zeros_like(m1.clone()); // Based on m1, thus 3 by 5
//! }
//! ```
//!
//! # Example operations
//! ```
//! use lars::matrix;
//! use lars::matrix::Matrix;
//!
//! fn matrix_operation() {
//!     // Create matrices
//!     let mut foo = matrix::new(5, 5, 2.0);
//!     let mut bar = matrix::new(5, 5, 1.5);
//!
//!     // Do operations
//!     let product = foo.clone() * bar.clone();
//!     let sum     = foo.clone() + bar.clone();
//!     let delta   = foo.clone() - bar.clone();
//!     let frac    = foo.clone() / bar.clone();
//!
//!     // Operations with non-matrices
//!     let doubled = foo.clone() * 2;
//!     let offset  = foo.clone() + 42;
//!     let subbed  = foo.clone() - 100;
//!     let halfed  = foo.clone() / 2;
//!
//!     // Transposing a matrix 'foo'
//!     foo.transpose();
//!
//!     // Getting transposed matrix
//!     let trans_foo = foo.transposed();
//!
//!     // Get trace of matrix
//!     let trace_bar = bar.trace();
//!
//!     // Set element of a matrix
//!     foo.set(2, 2, 4.2);
//!
//!     // Get element of a matrix
//!     let element = bar.get(2, 3);
//!
//!     let mut foobar = matrix::new(4, 4, 0.0);
//!
//!     // Reshape a matrix
//!     foobar.reshape(8, 2); // 4 by 4 >>> 8 by 2
//! }
//! ```
//!
//! # Example matrix * vector
//! ```
//! use lars::matrix;
//! use lars::matrix::Matrix;
//!
//! use lars::vector;
//! use lars::vector::Vector;
//!
//! fn matrix_vector_dot() {
//!     // Create 3 by 5 matrix of 3.0s
//!     let foo = Matrix::new(3, 5, 3.0);
//!
//!     // Create 3 dimensional vector of 6.0s
//!     let bar = Vector::new(3, 6.0);
//!
//!     // Multiply vector by matrix
//!     let foobar = foo * bar;
//! }
//! ```

extern crate rand;

use std::ops::{Index, Add, Sub, Mul, Div, Neg};
use std::fmt;

use common::Number;
use vector;
use vector::Vector;

pub struct Matrix<T: Number> {
    rows: usize,
    cols: usize,
    content: Vector<T>,
}

impl<T: Number + fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        for n in 0 .. self.rows {
            try!(write!(f, "["));
            for m in 0 .. self.cols {
                try!(write!(f, "{}", self.get(n, m)));
            }
            try!(write!(f, "],\n"));
        }
        try!(write!(f, "]"));
        Ok(())
    }
}

impl<T: Number> Index<usize> for Matrix<T> {
    type Output = [T];

    #[inline]
    fn index<'a>(&'a self, index: usize) -> &'a [T] {
        &self.content.content[self.rows * index .. self.rows * index + self.cols]
    }
}

impl<T: Number> Clone for Matrix<T> {
    fn clone(&self) -> Matrix<T> {
        Matrix::<T> {
            rows: self.rows,
            cols: self.cols,
            content: self.content.clone(),
        }
    }

    fn clone_from(&mut self, source: &Matrix<T>) {
        self.rows = source.rows;
        self.cols = source.cols;
        self.content = source.content.clone();
    }
}

impl<T: Number> Add<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: Matrix<T>) -> Matrix<T> {
        if self.rows == rhs.rows
                && self.cols == rhs.cols {
            Matrix::<T> {
                rows: self.rows,
                cols: self.cols,
                content: self.content + rhs.content,
            }
        } else {
            panic!("Can't add matrices of different dimensions!");
        }
    }
}

impl<T: Number> Sub<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: Matrix<T>) -> Matrix<T> {
        if self.rows == rhs.rows
                && self.cols == rhs.cols {
            Matrix::<T> {
                rows: self.rows,
                cols: self.cols,
                content: self.content - rhs.content,
            }
        } else {
            panic!("Can't subtract matrices of different dimensions!");
        }
    }
}

impl<T: Number> Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Matrix<T> {
        if self.cols == rhs.cols {
            let mut pass = Matrix::<T>::new(self.rows, self.cols, T::zero());
            for n in 0 .. self.rows {
                for m in 0 .. rhs.cols {
                    let mut product: T = T::zero();
                    for k in 0 .. self.cols {
                        product = product + self.get(n, k) * rhs.get(k, m);
                    }
                    pass.set(n, m, product);
                }
            }
            pass
        } else {
            panic!("Can't multiply matrices of different dimensions!")
        }
    }
}

impl<T: Number> Div<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn div(self, rhs: Matrix<T>) -> Matrix<T> {
        if self.cols == rhs.cols {
            let mut pass = Matrix::<T>::new(self.rows, self.cols, T::zero());
            for n in 0 .. self.rows {
                for m in 0 .. rhs.cols {
                    let mut product: T = T::zero();
                    for k in 0 .. self.cols {
                        product = product + self.get(n, k) / rhs.get(k, m);
                    }
                    pass.set(n, m, product);
                }
            }
            pass
        } else {
            panic!("Can't divide matrices of different dimensions!")
        }
    }
}

impl<T: Number + Neg<Output = T>> Neg for Matrix<T> {
    type Output = Matrix<T>;

    fn neg(self) -> Matrix<T> {
        Matrix::<T> {
            rows: self.rows,
            cols: self.cols,
            content: -self.content,
        }
    }
}

impl<T: Number> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Matrix<T> {
        Matrix::<T> {
            rows: self.rows,
            cols: self.cols,
            content: self.content * rhs,
        }
    }
}

impl<T: Number> Mul<Vector<T>> for Matrix<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Vector<T> {
        if self.cols == rhs.content.len() {
            let mut pass = Vector::<T>::new(0, T::zero());
            let mut i = 0;
            while i < self.content.len() / self.cols {
                let mut p = T::zero();
                for n in 0 .. self.cols {
                    p = p + self.content[(i * self.cols) + n] * rhs.content[n];
                }
                pass.content.push(p);
                i += 1
            }
            pass
        } else {
            panic!("Can't multiply matrix with given vector!")
        }
    }
}

impl<T: Number> Add<Vector<T>> for Matrix<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Vector<T> {
        if self.cols == rhs.content.len() {
            let mut pass = Vector::<T>::new(0, T::zero());
            let mut i = 0;
            while i < self.content.len() / self.cols {
                let mut p = T::zero();
                for n in 0 .. self.cols {
                    p = p + self.content[(i * self.cols) + n] + rhs.content[n];
                }
                pass.content.push(p);
                i += 1
            }
            pass
        } else {
            panic!("Can't add matrix with given vector!")
        }
    }
}

impl<T: Number> Sub<Vector<T>> for Matrix<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Vector<T> {
        if self.cols == rhs.content.len() {
            let mut pass = Vector::<T>::new(0, T::zero());
            let mut i = 0;
            while i < self.content.len() / self.cols {
                let mut p = T::zero();
                for n in 0 .. self.cols {
                    p = p + self.content[(i * self.cols) + n] - rhs.content[n];
                }
                pass.content.push(p);
                i += 1
            }
            pass
        } else {
            panic!("Can't subtract matrix with given vector!")
        }
    }
}

impl<T: Number> Div<Vector<T>> for Matrix<T> {
    type Output = Vector<T>;

    fn div(self, rhs: Vector<T>) -> Vector<T> {
        if self.cols == rhs.content.len() {
            let mut pass = Vector::<T>::new(0, T::zero());
            let mut i = 0;
            while i < self.content.len() / self.cols {
                let mut p = T::zero();
                for n in 0 .. self.cols {
                    p = p + self.content[(i * self.cols) + n] / rhs.content[n];
                }
                pass.content.push(p);
                i += 1
            }
            pass
        } else {
            panic!("Can't divide matrix with given vector!")
        }
    }
}

impl<T: Number> Div<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn div(self, rhs: T) -> Matrix<T> {
        Matrix::<T> {
            rows: self.rows,
            cols: self.cols,
            content: self.content / rhs,
        }
    }
}

impl<T: Number> Add<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: T) -> Matrix<T> {
        Matrix::<T> {
            rows: self.rows,
            cols: self.cols,
            content: self.content + rhs,
        }
    }
}

impl<T: Number> Sub<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, rhs: T) -> Matrix<T> {
        Matrix::<T> {
            rows: self.rows,
            cols: self.cols,
            content: self.content - rhs,
        }
    }
}

impl<T: Number> PartialEq for Matrix<T> {
    fn eq(&self, other: &Matrix<T>) -> bool {
        if self.rows != other.rows
                || self.cols != other.cols {
            return false;
        }
        self.content == other.content
    }
}

impl<T: Number> Eq for Matrix<T> {}

impl<T: Number> Matrix<T> {
    #[inline]
    pub fn get(&self, r: usize, c: usize) -> T {
        if r < self.rows && c < self.cols {
            self.content[r * self.cols + c]
        } else {
            panic!(format!("Matrix index ({}, {}) out of bounds!", r, c))
        }
    }

    #[inline]
    pub fn set(&mut self, r: usize, c: usize, a: T) {
        if r < self.rows && c < self.cols {
            self.content[r * self.cols + c] = a
        } else {
            panic!(format!("Matrix index ({}, {}) out of bounds!", r, c))
        }
    }

    pub fn new(rows: usize, cols: usize, default: T) -> Matrix<T>  {
        Matrix::<T> {
            rows: rows,
            cols: cols,
            content: Vector::new(rows * cols, default),
        }
    }

    pub fn reshape(&mut self, rows: usize, cols: usize) {
        if self.rows * self.cols == rows * cols {
            self.rows = rows;
            self.cols = cols;
        } else {
            panic!("Ammount of elements in matrix should be the same!")
        }
    }

    pub fn get_vector(&self) -> Vec<T> {
        self.content.content.clone()
    }

    pub fn get_cols(&self) -> usize {
        self.cols.clone()
    }

    pub fn get_rows(&self) -> usize {
        self.rows.clone()
    }

    pub fn transpose(&mut self) {
        let mut pass = self.content.clone();
        for n in 0 .. self.rows {
            for m in 0 .. self.cols {
                pass[m * self.cols + n] = self.get(n, m);
            }
        }
        self.content = pass
    }

    pub fn transposed(&self) -> Matrix<T> {
        let mut pass = self.clone();
        for n in 0 .. self.rows {
            for m in 0 .. self.cols {
                pass.content[m * self.cols + n] = self.get(n, m);
            }
        }
        pass
    }

    pub fn trace(&self) -> T {
        if self.rows == self.cols {
            let mut sum: T = T::zero();
            for n in 0 .. self.rows {
                sum = sum + self.get(n, n);
            }
            sum
        } else {
            panic!("Matrix must be a square!")
        }
    }

    pub fn powf(&self, pow: f64) {
        self.content.powf(pow);
    }
}

pub fn from<T: Number>(rows: usize, cols: usize, elements: &[T]) -> Matrix<T> {
    Matrix::<T> {
        rows: rows,
        cols: cols,
        content: vector::from(elements),
    }
}

pub fn identity<T: Number>(size: usize) -> Matrix<T> {
    let mut i = Matrix::<T>::new(size, size, T::zero());
    for n in 0 .. size {
        i.set(n, n, T::one());
    }
    i
}

pub fn zeros_like<T: Number>(other: Matrix<T>) -> Matrix<T> {
    Matrix::<T> {
        rows: other.rows,
        cols: other.cols,
        content: Vector::new(other.rows * other.cols, T::zero()),
    }
}

pub fn zeros<T: Number>(rows: usize, cols: usize) -> Matrix<T> {
    Matrix::<T> {
        rows: rows,
        cols: cols,
        content: Vector::new(rows * cols, T::zero()),
    }
}

pub fn random<T: Number + rand::Rand>(rows: usize, cols: usize) -> Matrix<T> {
    Matrix::<T> {
        rows: rows,
        cols: cols,
        content: vector::random(rows * cols),
    }
}
