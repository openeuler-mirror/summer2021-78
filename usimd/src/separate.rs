#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::time;
pub fn simd_feature_detected() -> i32 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        return if is_x86_feature_detected!("avx512f") {
        4
    } else if is_x86_feature_detected!("avx") {
        2
    } else if is_x86_feature_detected!("sse") {
        1
    } else {
        0
    };
}

pub enum SimdF64 {
    Avx512(__m512d),
    Avx(__m256d),
    Sse(__m128d),
}

pub unsafe fn set1_pd(a: f64, simd_feature: i32) -> SimdF64 {
    match simd_feature {
        0 => panic!("error,your computer doesn't support simd"),
        1 => return SimdF64::Sse(_mm_set1_pd(a)),
        2 => return SimdF64::Avx(_mm256_set1_pd(a)),
        4 => return SimdF64::Avx512(_mm512_set1_pd(a)),
        _ => panic!("please detect the simd feature first"),
    }
}

pub unsafe fn set_pd(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    g: f64,
    h: f64,
    simd_feature: i32,
) -> SimdF64 {
    match simd_feature {
        0 => panic!("error,your computer doesn't support simd"),
        1 => {
            println!("Only the first two parameters can be set!!!");
            return SimdF64::Sse(_mm_set_pd(a, b));
        }
        2 => {
            println!("Only the first four parameters can be set");
            return SimdF64::Avx(_mm256_set_pd(a, b, c, d));
        }
        4 => return SimdF64::Avx512(_mm512_set_pd(a, b, c, d, e, f, g, h)),
        _ => panic!("please detect the simd feature first"),
    }
}

pub unsafe fn setzero_pd(simd_feature: i32) -> SimdF64 {
    match simd_feature {
        0 => panic!("error,your computer doesn't support simd"),
        1 => return SimdF64::Sse(_mm_setzero_pd()),
        2 => return SimdF64::Avx(_mm256_setzero_pd()),
        4 => return SimdF64::Avx512(_mm512_setzero_pd()),
        _ => panic!("please detect the simd feature first"),
    }
}

pub unsafe fn add_pd(a: &SimdF64, b: &SimdF64) -> SimdF64 {
    return match a {
        SimdF64::Avx512(data1) => {
            if let SimdF64::Avx512(data2) = b {
                SimdF64::Avx512(_mm512_add_pd(*data1, *data2))
            } else {
                panic!();
            }
        }
        SimdF64::Avx(data1) => {
            if let SimdF64::Avx(data2) = b {
                SimdF64::Avx(_mm256_add_pd(*data1, *data2))
            } else {
                panic!();
            }
        }
        SimdF64::Sse(data1) => {
            if let SimdF64::Sse(data2) = b {
                SimdF64::Sse(_mm_add_pd(*data1, *data2))
            } else {
                panic!();
            }
        }
    };
}

pub unsafe fn mul_pd(a: &SimdF64, b: &SimdF64) -> SimdF64 {
    return match a {
        SimdF64::Avx512(data1) => {
            if let SimdF64::Avx512(data2) = b {
                SimdF64::Avx512(_mm512_mul_pd(*data1, *data2))
            } else {
                panic!();
            }
        }
        SimdF64::Avx(data1) => {
            if let SimdF64::Avx(data2) = b {
                SimdF64::Avx(_mm256_mul_pd(*data1, *data2))
            } else {
                panic!();
            }
        }
        SimdF64::Sse(data1) => {
            if let SimdF64::Sse(data2) = b {
                SimdF64::Sse(_mm_mul_pd(*data1, *data2))
            } else {
                panic!();
            }
        }
    };
}

pub unsafe fn div_pd(a: &SimdF64, b: &SimdF64) -> SimdF64 {
    return match a {
        SimdF64::Avx512(data1) => {
            if let SimdF64::Avx512(data2) = b {
                SimdF64::Avx512(_mm512_div_pd(*data1, *data2))
            } else {
                panic!();
            }
        }
        SimdF64::Avx(data1) => {
            if let SimdF64::Avx(data2) = b {
                SimdF64::Avx(_mm256_div_pd(*data1, *data2))
            } else {
                panic!();
            }
        }
        SimdF64::Sse(data1) => {
            if let SimdF64::Sse(data2) = b {
                SimdF64::Sse(_mm_div_pd(*data1, *data2))
            } else {
                panic!();
            }
        }
    };
}

pub unsafe fn store_pd(ptr: *mut f64, a: SimdF64) {
    match a {
        SimdF64::Avx512(data1) => _mm512_store_pd(ptr, data1),
        SimdF64::Avx(data2) => _mm256_store_pd(ptr, data2),
        SimdF64::Sse(data3) => _mm_store_pd(ptr, data3),
    };
}
