#![feature(stdsimd)]
mod add;
mod div;
mod mul;
mod sub;

pub use crate::add::add_i32::add_i32;
pub use crate::add::add_i8::add_i8;
pub use crate::add::add_i16::add_i16;
pub use crate::add::add_i64::add_i64;
pub use crate::add::add_f32::add_f32;
pub use crate::add::add_f64::add_f64;
pub use crate::mul::mul_f64::mul_f64;
pub use crate::mul::mul_f32::mul_f32;
pub use crate::div::div_f64::div_f64;
pub use crate::div::div_f32::div_f32;
pub use crate::mul::i32_to_i64_then_mul::i32_to_i64_then_mul;
pub use crate::sub::sub_i8::sub_i8;
pub use crate::sub::sub_i16::sub_i16;
pub use crate::sub::sub_i32::sub_i32;
pub use crate::sub::sub_i64::sub_i64;
pub use crate::sub::sub_f32::sub_f32;
pub use crate::sub::sub_f64::sub_f64;