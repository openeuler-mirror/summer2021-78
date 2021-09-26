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
pub fn sub_i16<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = i16> + len_trait::Len + ?Sized,
{
    return if is_x86_feature_detected!("avx512f") {
        unsafe { sub_i16_avx512(container_a, container_b) }
    } else if is_x86_feature_detected!("avx") {
        unsafe { sub_i16_avx(container_a, container_b) }
    } else if is_x86_feature_detected!("sse") {
        unsafe { sub_i16_sse(container_a, container_b) }
    } else {
        sub_i16_without_simd(container_a, container_b)
    };
}
#[cfg(target_arch = "arm")]
fn sub<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = i16> + len_trait::Len + ?Sized,
{
    return if is_arm_feature_detected!("neon") {
        unsafe { sub_neon(container_a, container_b) }
    } else {
        sub_without_simd(container_a, container_b)
    };
}
#[cfg(target_arch = "aarch64")]
fn sub<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = i16> + len_trait::Len + ?Sized,
{
    return if is_aarch64_feature_detected!("neon") {
        unsafe { sub_neon(container_a, container_b) }
    } else {
        sub_without_simd(container_a, container_b)
    };
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn sub_i16_avx512<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = i16> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be subed because the lengths are unequal");
    }
    let group_len = 32;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm512_loadu_epi16(&container_a[i * group_len] as *const i16);
        let b_vector = _mm512_loadu_epi16(&container_b[i * group_len] as *const i16);
        _mm512_storeu_epi16(
            &mut container_a[i * group_len] as *mut i16,
            _mm512_sub_epi16(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] + container_b[group * group_len + i];
    }
    container_a
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn sub_i16_avx<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = i16> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be subed because the lengths are unequal");
    }
    let group_len = 16;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm256_loadu_epi16(&container_a[i * group_len] as *const i16);
        let b_vector = _mm256_loadu_epi16(&container_b[i * group_len] as *const i16);
        _mm256_storeu_epi16(
            &mut container_a[i * group_len] as *mut i16,
            _mm256_sub_epi16(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] - container_b[group * group_len + i];
    }
    container_a
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn sub_i16_sse<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = i16> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be subed because the lengths are unequal");
    }
    let group_len = 8;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm_loadu_epi16(&container_a[i * group_len] as *const i16);
        let b_vector = _mm_loadu_epi16(&container_b[i * group_len] as *const i16);
        _mm_storeu_epi16(
            &mut container_a[i * 4] as *mut i16,
            _mm_sub_epi16(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] - container_b[group * group_len + i];
    }
    container_a
}

fn sub_i16_without_simd<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = i16> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be subed because the lengths are unequal");
    }
    for i in 0..len_a {
        container_a[i] = container_a[i] - container_b[i];
    }
    container_a

}
#[cfg(any(target_arch = "arm", target_arch = "aarch"))]
unsafe fn sub_neon<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = i16> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be subed because the lengths are unequal");
    }
    let group_len = 2;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = vld1q_s32(&container_a[i * group_len] as *const i16);
        let b_vector = vld1q_s32(&container_b[i * group_len] as *const i16);
        vst1q_s32(
            &mut container_a[i * group_len] as *mut i16,
            vsub_s32(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] - container_b[group * group_len + i];
    }
    container_a
}

mod tests {
    use crate::sub::sub_i16::sub_i16;

    #[test]
    fn sub_i16_works() {
        let mut a: Vec<i16> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        sub_i16(&mut a, &b);
        for i in 0..10 {
            assert_eq!(a[i], 0);
        }
    }
}