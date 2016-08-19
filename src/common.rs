extern crate num;

use self::num::traits::Num;

pub trait Number: Num + Clone + Copy {
    fn powf(&self, pow: f64) -> f64;
}

impl Number for f64 {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}

impl Number for f32 {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}

impl Number for i64 {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}

impl Number for i32 {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}

impl Number for i16 {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}

impl Number for i8 {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}

impl Number for u64 {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}

impl Number for u32 {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}

impl Number for u16 {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}

impl Number for u8 {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}

impl Number for usize {
    fn powf(&self, pow: f64) -> f64 {
        (*self as f64).powf(pow)
    }
}
