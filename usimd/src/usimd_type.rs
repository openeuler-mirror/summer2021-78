use std::arch::x86_64::*;
use std::ops::Add;

pub struct USimdI32{
    data:Vec<i32>,
}

impl From<Vec<i32>> for USimdI32 {
    fn from(item: Vec<i32>) -> Self {
        USimdI32 { data: item }
    }
}

impl Add for USimdI32 {
    type Output = USimdI32;

    fn add(self, rhs: Self) -> Self::Output {
        return if is_x86_feature_detected!("avx2") {
            unsafe { add_avx2(self, rhs) }
        } else {
            unsafe { add_avx2(self, rhs) }
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn add_avx2(mut a: USimdI32, b: USimdI32) -> USimdI32 {
    let len_a = a.data.len();
    let len_b = b.data.len();
    if len_a != len_b {
        panic!("Vectors can't be added because the lengths are unequal");
    }
    let group = len_a / 8;
    let mut tmp =
    for i in 0..len_a {
        _mm256_add_epi32(
            _mm256_set_epi32(a.data[i],a.data[i + 1],a.data[i + 2],a.data[i + 3],a.data[i + 4],a.data[i + 5],a.data[i + 6],a.data[i + 7]),
            _mm256_set_epi32(b.data[i],b.data[i + 1],b.data[i + 2],b.data[i + 3],b.data[i + 4],b.data[i + 5],b.data[i + 6],b.data[i + 7])
        )
    }
    USimdI32
}
