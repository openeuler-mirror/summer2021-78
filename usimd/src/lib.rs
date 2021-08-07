mod usimd_type;

#[cfg(test)]
mod tests {
    use crate::usimd_type::USimdI32;

    #[test]
    fn add_works() {
        let a = USimdI32::from(vec![1,2,3,4,5,6,7,8,9,10]);
        let b = USimdI32::from(vec![1;10]);
        let c = a + b;
        assert_eq!(c,USimdI32::from(vec![2,3,4,5,,6,7,8,9,10,11]));
    }
}
