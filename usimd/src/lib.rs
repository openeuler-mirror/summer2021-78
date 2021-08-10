#![feature(stdsimd)]
mod usimd_type;

#[cfg(test)]
mod tests {
    use crate::usimd_type::USimdI32;

    #[test]
    fn add_works() {
        let a = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        let b = vec![16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1];
        let a_usimd = USimdI32::from(&a);
        let b_usimd = USimdI32::from(&b);
        let c = a_usimd + b_usimd;
        for i in 0..16 {
            assert_eq!(c.data[i],17);
        }
    }
}
