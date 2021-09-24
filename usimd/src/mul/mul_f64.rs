#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "arm")]
use std::arch::arm::*;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
use std::ops::IndexMut;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) fn mul_f64<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    return if is_x86_feature_detected!("avx512f") {
        unsafe { mul_f64_avx512(container_a, container_b) }
    } else if is_x86_feature_detected!("avx") {
        unsafe { mul_f64_avx(container_a, container_b) }
    } else if is_x86_feature_detected!("sse") {
        unsafe { mul_f64_sse(container_a, container_b) }
    } else {
        mul_f64_without_simd(container_a, container_b)
    };
}
#[cfg(target_arch = "arm")]
fn mul<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    return if is_arm_feature_detected!("neon") {
        unsafe { mul_neon(container_a, container_b) }
    } else {
        mul_without_simd(container_a, container_b)
    };
}
#[cfg(target_arch = "aarch64")]
fn mul<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    return if is_aarch64_feature_detected!("neon") {
        unsafe { mul_neon(container_a, container_b) }
    } else {
        mul_without_simd(container_a, container_b)
    };
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn mul_f64_avx512<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be muled because the lengths are unequal");
    }
    let group_len = 8;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm512_loadu_pd(&container_a[i * group_len] as *const f64);
        let b_vector = _mm512_loadu_pd(&container_b[i * group_len] as *const f64);
        _mm512_store_pd(
            &mut container_a[i * group_len] as *mut f64,
            _mm512_mul_pd(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] * container_b[group * group_len + i];
    }
    container_a
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn mul_f64_avx<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be muled because the lengths are unequal");
    }
    let group_len = 4;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm256_loadu_pd(&container_a[i * group_len] as *const f64);
        let b_vector = _mm256_loadu_pd(&container_b[i * group_len] as *const f64);
        _mm256_storeu_pd(
            &mut container_a[i * group_len] as *mut f64,
            _mm256_mul_pd(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] * container_b[group * group_len + i];
    }
    container_a
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn mul_f64_sse<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be muled because the lengths are unequal");
    }
    let group_len = 2;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm_loadu_pd(&container_a[i * group_len] as *const f64);
        let b_vector = _mm_loadu_pd(&container_b[i * group_len] as *const f64);
        _mm_store_pd(
            &mut container_a[i * 4] as *mut f64,
            _mm_mul_pd(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] * container_b[group * group_len + i];
    }
    container_a
}

fn mul_f64_without_simd<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be muled because the lengths are unequal");
    }
    for i in 0..len_a {
        container_a[i] = container_a[i] - container_b[i];
    }
    container_a

}
#[cfg(any(target_arch = "arm", target_arch = "aarch"))]
unsafe fn mul_neon<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be muled because the lengths are unequal");
    }
    let group_len = 2;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = vld1q_s32(&container_a[i * group_len] as *const f64);
        let b_vector = vld1q_s32(&container_b[i * group_len] as *const f64);
        vst1q_s32(
            &mut container_a[i * group_len] as *mut f64,
            vmul_s32(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] * container_b[group * group_len + i];
    }
    container_a
}

#[cfg(test)]
mod tests {
    use crate::mul::mul_f64::mul_f64;

    #[test]
    fn mul_f64_works() {
        let mut a:Vec<f64> = vec![1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0];
        let b:Vec<f64> = vec![512.0, 256.0, 128.0, 64.0, 32.0, 16.0, 8.0, 4.0, 2.0, 1.0];
        mul_f64(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 512.0);
        }
    }
}