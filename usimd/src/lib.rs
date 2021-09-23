#![feature(stdsimd)]
mod add_i32;
mod add_i64;
#[cfg(test)]
mod tests {
    use crate::add_i32;
    use crate::add_i64;
    #[test]
    fn add_works() {
        let mut a = vec![1,2, 3, 4, 5, 6, 7, 8,9,10];
        let b = vec![10,9,8,7,6,5,4,3,2,1];
        add_i32::add_i32(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 11);
        }
        let mut a:Vec<i64> = vec![1,2, 3, 4, 5, 6, 7, 8,9,10];
        let b = vec![10,9,8,7,6,5,4,3,2,1];
        add_i64::add_i64(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 11);
        }
    }
}
