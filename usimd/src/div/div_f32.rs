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
pub fn div_f32<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f32> + len_trait::Len + ?Sized,
{
    return if is_x86_feature_detected!("avx512f") {
        unsafe { div_f32_avx512(container_a, container_b) }
    } else if is_x86_feature_detected!("avx") {
        unsafe { div_f32_avx(container_a, container_b) }
    } else if is_x86_feature_detected!("sse") {
        unsafe { div_f32_sse(container_a, container_b) }
    } else {
        div_f32_without_simd(container_a, container_b)
    };
}
#[cfg(target_arch = "arm")]
fn div<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f32> + len_trait::Len + ?Sized,
{
    return if is_arm_feature_detected!("neon") {
        unsafe { div_neon(container_a, container_b) }
    } else {
        div_without_simd(container_a, container_b)
    };
}
#[cfg(target_arch = "aarch64")]
fn div<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f32> + len_trait::Len + ?Sized,
{
    return if is_aarch64_feature_detected!("neon") {
        unsafe { div_neon(container_a, container_b) }
    } else {
        div_without_simd(container_a, container_b)
    };
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn div_f32_avx512<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f32> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be dived because the lengths are unequal");
    }
    let group_len = 16;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm512_loadu_ps(&container_a[i * group_len] as *const f32);
        let b_vector = _mm512_loadu_ps(&container_b[i * group_len] as *const f32);
        _mm512_store_ps(
            &mut container_a[i * group_len] as *mut f32,
            _mm512_div_ps(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] / container_b[group * group_len + i];
    }
    container_a
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn div_f32_avx<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f32> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be dived because the lengths are unequal");
    }
    let group_len = 8;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm256_loadu_ps(&container_a[i * group_len] as *const f32);
        let b_vector = _mm256_loadu_ps(&container_b[i * group_len] as *const f32);
        _mm256_storeu_ps(
            &mut container_a[i * group_len] as *mut f32,
            _mm256_div_ps(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] / container_b[group * group_len + i];
    }
    container_a
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn div_f32_sse<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f32> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be dived because the lengths are unequal");
    }
    let group_len = 4;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm_loadu_ps(&container_a[i * group_len] as *const f32);
        let b_vector = _mm_loadu_ps(&container_b[i * group_len] as *const f32);
        _mm_store_ps(
            &mut container_a[i * 4] as *mut f32,
            _mm_div_ps(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] / container_b[group * group_len + i];
    }
    container_a
}

fn div_f32_without_simd<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f32> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be dived because the lengths are unequal");
    }
    for i in 0..len_a {
        container_a[i] = container_a[i] / container_b[i];
    }
    container_a

}
#[cfg(any(target_arch = "arm", target_arch = "aarch"))]
unsafe fn div_neon<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f32> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be dived because the lengths are unequal");
    }
    let group_len = 2;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = vld1q_s32(&container_a[i * group_len] as *const f32);
        let b_vector = vld1q_s32(&container_b[i * group_len] as *const f32);
        vst1q_s32(
            &mut container_a[i * group_len] as *mut f32,
            vdiv_s32(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] / container_b[group * group_len + i];
    }
    container_a
}

#[cfg(test)]
mod tests {
    use crate::div::div_f32::div_f32;

    #[test]
    fn div_f32_works() {
        let mut a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let b = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        div_f32(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 1.0);
        }
    }
}