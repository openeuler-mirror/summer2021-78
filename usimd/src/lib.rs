#![feature(stdsimd)]
mod add_i32;
mod add_i64;
mod add_i8;
mod add_i16;
mod add_f32;
mod add_f64;
mod sub_i32;
mod sub_i64;
mod sub_i8;
mod sub_i16;
mod sub_f32;
mod sub_f64;
mod mul_i32_result_i32;
mod i32_to_i64_then_mul;
mod mul_f32;
mod div_f32;
mod mul_f64;
mod div_f64;

#[cfg(test)]
mod tests {
    use crate::add_i32;
    use crate::add_i64;
    use crate::add_i8;
    use crate::add_i16;
    use crate::add_f32;
    use crate::add_f64;
    use crate::sub_i32;
    use crate::sub_i64;
    use crate::sub_i8;
    use crate::sub_i16;
    use crate::sub_f32;
    use crate::sub_f64;
    use crate::mul_i32_result_i32;
    use crate::i32_to_i64_then_mul;
    use crate::mul_f32;
    use crate::div_f32;
    use crate::mul_f64;
    use crate::div_f64;
    #[test]
    fn add_works() {
        let mut a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        add_i32::add_i32(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 11);
        }
        let mut a: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        add_i64::add_i64(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 11);
        }
        let mut a: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        add_i8::add_i8(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 11);
        }
        let mut a: Vec<i16> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        add_i16::add_i16(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 11);
        }
        let mut a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let b = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        add_f32::add_f32(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 11.0);
        }
        let mut a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let b = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        add_f64::add_f64(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 11.0);
        }
        let mut a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        sub_i32::sub_i32(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 0);
        }
        let mut a: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        sub_i64::sub_i64(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 0);
        }
        let mut a: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        sub_i8::sub_i8(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 0);
        }
        let mut a: Vec<i16> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        sub_i16::sub_i16(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 0);
        }
        let mut a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let b = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        sub_f32::sub_f32(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 0.0);
        }
        let mut a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let b = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        sub_f64::sub_f64(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 0.0);
        }
        let mut a = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
        let b = vec![512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
        mul_i32_result_i32::mul_i32(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 512);
        }
        let mut a = vec![1, 0, 4, 0, 16, 0, 64, 0, 256, 512];
        let b = vec![512,0, 128, 0, 32, 0, 8, 0, 2, 0];
        i32_to_i64_then_mul::i32_to_i644_then_mul(&mut a, &b);
        for i in 0..5 {
            assert_eq!(a[i * 2],512);
        }
        let mut a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let b = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        div_f32::div_f32(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 1.0);
        }
        let mut a:Vec<f32> = vec![1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0];
        let b = vec![512.0, 256.0, 128.0, 64.0, 32.0, 16.0, 8.0, 4.0, 2.0, 1.0];
        mul_f32::mul_f32(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 512.0);
        }
        let mut a:Vec<f64> = vec![1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0];
        let b = vec![512.0, 256.0, 128.0, 64.0, 32.0, 16.0, 8.0, 4.0, 2.0, 1.0];
        mul_f64::mul_f64(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 512.0);
        }
        let mut a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let b = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        div_f64::div_f64(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 1.0);
        }
    }
}
