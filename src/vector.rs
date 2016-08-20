//! The Vector type overloads all basic arithmetic operations: addition, subtraction,
//! multiplication, division - along with the possibility of getting negative and index
//! in/of given vector.
//!
//! # Example creation
//!
//! ```
//! use lars::vector;
//! use lars::vector::Vector;
//!
//! fn vector_creation() {
//!     // Create 12 dimensional vector of 6.0s
//!     let v1 = Vector::new(12, 6.0);
//!
//!     // Create vector from elements
//!     let elements = [1.0, 3.0, 3.0, 7.0];
//!     let v2 = vector::from(&elements);
//!
//!     // Create random vector of length 12
//!     let v3 = vector::random(12);
//! }
//! ```
//!
//! # Example operations
//! ```
//! use lars::vector;
//! use lars::vector::Vector;
//!
//! fn vector_operation() {
//!     // Create vectors
//!     let mut foo = vector::new(5, 2.0);
//!     let mut bar = vector::new(5, 1.5);
//!
//!     // Do operations
//!     let product = foo.clone() * bar.clone();
//!     let sum     = foo.clone() + bar.clone();
//!     let delta   = foo.clone() - bar.clone();
//!     let frac    = foo.clone() / bar.clone();
//!
//!     // Operations with non-vectors
//!     let doubled = foo.clone() * 2;
//!     let offset  = foo.clone() + 42;
//!     let subbed  = foo.clone() - 100;
//!     let halfed  = foo.clone() / 2;
//! }
//! ```
//!
//! # Example vector * matrix
//! ```
//! use lars::vector;
//! use lars::vector::Vector;
//!
//! use lars::matrix;
//! use lars::matrix::Matrix;
//!
//! fn vector_matrix_dot() {
//!     // Create 3 dimensional vector of 6.0s
//!     let foo = Vector::new(3, 6.0);
//!
//!     // Create 3 by 5 matrix of 3.0s
//!     let bar = Matrix::new(3, 5, 3.0);
//!
//!     // Multiply matrix by vector
//!     let foobar = foo * bar;
//! }
//! ```

extern crate rand;

use std::fmt;

use std::ops::{Index, IndexMut, Add, Sub, Mul, Div, Neg};
use common::Number;

use matrix::Matrix;

pub struct Vector<T: Number> {
    pub content: Vec<T>,
}

impl<T: Number + fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        for n in 0 .. self.content.len() {
            try!(write!(f, "{}", self.content[n]));
            if n != self.content.len() - 1 {
                try!(write!(f, ","))
            }
        }
        try!(write!(f, "]"));
        Ok(())
    }
}

impl<T: Number> Index<usize> for Vector<T> {
    type Output = T;
    #[inline]
    fn index<'a>(&'a self, index: usize) -> &'a T {
        &self.content[index]
    }
}

impl<T: Number> IndexMut<usize> for Vector<T> {
    #[inline]
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        &mut self.content[index]
    }
}

impl<T: Number> Clone for Vector<T> {
    fn clone(&self) -> Vector<T> {
        Vector::<T> {
            content: self.content.clone()
        }
    }

    fn clone_from(&mut self, source: &Vector<T>) {
        self.content = source.content.clone()
    }
}

impl<T: Number> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Vector<T> {
        if self.len() == rhs.len() {
            let mut pass = Vector::<T>::new(self.len(), T::zero());
            for n in 0 .. self.len() {
                pass.content[n] = self.content[n] + rhs.content[n];
            }
            pass
        } else {
            panic!("Trying to add vectors of different dimensions!")
        }
    }
}

impl<T: Number> Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: Vector<T>) -> Vector<T> {
        if self.len() == rhs.len() {
            let mut pass = Vector::<T>::new(self.len(), T::zero());
            for n in 0 .. self.len() {
                pass.content[n] = self.content[n] - rhs.content[n];
            }
            pass
        } else {
            panic!("Trying to subtract vectors of different dimensions!")
        }
    }
}

impl<T: Number> Mul<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Vector<T> {
        if self.len() == rhs.len() {
            let mut pass = Vector::<T>::new(self.len(), T::zero());
            for n in 0 .. self.len() {
                pass.content[n] = self.content[n] * rhs.content[n];
            }
            pass
        } else {
            panic!("Trying to multiply vectors of different dimensions!")
        }
    }
}

impl<T: Number> Mul<Matrix<T>> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: Matrix<T>) -> Vector<T> {
        if rhs.get_cols() == rhs.get_vector().len() {
            let mut pass = Vector::<T>::new(0, T::zero());
            let mut i = 0;
            while i < rhs.get_vector().len() / rhs.get_cols() {
                let mut p = T::zero();
                for n in 0 .. rhs.get_cols() {
                    p = p + rhs.get_vector()[(i * rhs.get_cols()) + n] * self.content[n];
                }
                pass.content.push(p);
                i += 1
            }
            pass
        } else {
            panic!("Can't multiply vector by given matrix!")
        }
    }
}

impl<T: Number> Div<Matrix<T>> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, rhs: Matrix<T>) -> Vector<T> {
        if rhs.get_cols() == rhs.get_vector().len() {
            let mut pass = Vector::<T>::new(0, T::zero());
            let mut i = 0;
            while i < rhs.get_vector().len() / rhs.get_cols() {
                let mut p = T::zero();
                for n in 0 .. rhs.get_cols() {
                    p = p + rhs.get_vector()[(i * rhs.get_cols()) + n] / self.content[n];
                }
                pass.content.push(p);
                i += 1
            }
            pass
        } else {
            panic!("Can't divide vector by given matrix!")
        }
    }
}

impl<T: Number> Add<Matrix<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Matrix<T>) -> Vector<T> {
        if rhs.get_cols() == rhs.get_vector().len() {
            let mut pass = Vector::<T>::new(0, T::zero());
            let mut i = 0;
            while i < rhs.get_vector().len() / rhs.get_cols() {
                let mut p = T::zero();
                for n in 0 .. rhs.get_cols() {
                    p = p + rhs.get_vector()[(i * rhs.get_cols()) + n] + self.content[n];
                }
                pass.content.push(p);
                i += 1
            }
            pass
        } else {
            panic!("Can't add vector by given matrix!")
        }
    }
}

impl<T: Number> Sub<Matrix<T>> for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Matrix<T>) -> Vector<T> {
        if rhs.get_cols() == rhs.get_vector().len() {
            let mut pass = Vector::<T>::new(0, T::zero());
            let mut i = 0;
            while i < rhs.get_vector().len() / rhs.get_cols() {
                let mut p = T::zero();
                for n in 0 .. rhs.get_cols() {
                    p = p + rhs.get_vector()[(i * rhs.get_cols()) + n] - self.content[n];
                }
                pass.content.push(p);
                i += 1
            }
            pass
        } else {
            panic!("Can't subtract vector by given matrix!")
        }
    }
}

impl<T: Number> Div<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn div(self, rhs: Vector<T>) -> Vector<T> {
        if self.len() == rhs.len() {
            let mut pass = Vector::<T>::new(self.len(), T::zero());
            for n in 0 .. self.len() {
                pass.content[n] = self.content[n] / rhs.content[n];
            }
            pass
        } else {
            panic!("Trying to divide vectors of different dimensions!")
        }
    }
}

impl<T: Number + Neg<Output = T>> Neg for Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Vector<T> {
        let mut v = self.clone();
        for n in 0 .. self.len() {
            v.content[n] = -self.content[n];
        }
        v
    }
}

impl<T: Number> Mul<T> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Vector<T> {
        let mut v = self.clone();
        for n in 0 .. self.len() {
            v.content[n] = self.content[n] * rhs;
        }
        v
    }
}

impl<T: Number> Div<T> for Vector<T> {
    type Output = Vector<T>;
    fn div(self, rhs: T) -> Vector<T> {
        let mut v = self.clone();
        for n in 0 .. self.len() {
            v.content[n] = self.content[n] / rhs;
        }
        v
    }
}

impl<T: Number> Add<T> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: T) -> Vector<T> {
        let mut v = self.clone();
        for n in 0 .. self.len() {
            v.content[n] = self.content[n] + rhs;
        }
        v
    }
}

impl<T: Number> Sub<T> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: T) -> Vector<T> {
        let mut v = self.clone();
        for n in 0 .. self.len() {
            v.content[n] = self.content[n] - rhs;
        }
        v
    }
}

impl<T: Number> PartialEq for Vector<T> {
    fn eq(&self, other: &Vector<T>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for n in 0 .. self.len() {
            if self.content[n] != other.content[n] {
                return false;
            }
        }
        true
    }
}

impl<T: Number> Eq for Vector<T> {}

impl<T: Number> Vector<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn new(length: usize, default: T) -> Vector<T> {
        let d = vec![default; length];
        Vector::<T> {
            content: d,
        }
    }

    pub fn powf(&self, pow: f64) {
        for n in 0 .. self.len() {
            self.content[n].powf(pow);
        }
    }
}

pub fn from<T: Number>(elements: &[T]) -> Vector<T> {
    let mut v = Vector::<T> {
        content: Vec::with_capacity(elements.len())
    };
    v.content.extend_from_slice(elements);
    v
}

pub fn random<T: Number + rand::Rand>(length: usize) -> Vector<T> {
    let mut d = vec![T::zero(); length];
    for x in d.iter_mut() {
        *x = rand::random::<T>()
    }
    Vector::<T> {
        content: d,
    }
}
