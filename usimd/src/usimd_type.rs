use std::arch::x86_64::*;
use std::ops::Add;

pub struct USimdI32<'a> {
    pub data: &'a Vec<i32>,
}

impl<'a> From<&'a Vec<i32>> for USimdI32<'a> {
    fn from(item: &'a Vec<i32>) -> Self {
        USimdI32 { data: item }
    }
}

impl<'a> Add for USimdI32<'a> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return if is_x86_feature_detected!("avx2") {
            unsafe { add_avx2(self, rhs) }
        } else {
            unsafe { add_avx2(self, rhs) }
        };
    }
}

#[target_feature(enable = "avx2")]
unsafe fn add_avx2<'a>(a: USimdI32<'a>, b: USimdI32<'a>) -> USimdI32<'a> {
    let len_a = a.data.len();
    let len_b = b.data.len();
    if len_a != len_b {
        panic!("Vectors can't be added because the lengths are unequal");
    }

    let mut result_data:Vec<i32> = Vec::with_capacity(len_a);
    let group = len_a / 8;
    for i in 0..group {
        let a_vector = _mm256_loadu_epi32(&a.data[i * 8] as *const i32);
        let b_vector = _mm256_loadu_epi32(&b.data[i * 8] as *const i32);
        _mm256_store_epi32(&mut result_data[0] as *mut i32,_mm256_add_epi32(a_vector,b_vector));
    }
    for i in 0..len_a - group * 8 {
        result_data[group * 8 + i] = a.data[group * 8 + i] + b.data[group * 8 + i];
    }
    USimdI32::from(&result_data)
}
