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
pub(crate) fn sub_f64<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    return if is_x86_feature_detected!("avx512f") {
        unsafe { sub_f64_avx512(container_a, container_b) }
    } else if is_x86_feature_detected!("avx") {
        unsafe { sub_f64_avx(container_a, container_b) }
    } else if is_x86_feature_detected!("sse") {
        unsafe { sub_f64_sse(container_a, container_b) }
    } else {
        sub_f64_without_simd(container_a, container_b)
    };
}
#[cfg(target_arch = "arm")]
fn sub<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
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
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    return if is_aarch64_feature_detected!("neon") {
        unsafe { sub_neon(container_a, container_b) }
    } else {
        sub_without_simd(container_a, container_b)
    };
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn sub_f64_avx512<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be subed because the lengths are unequal");
    }
    let group_len = 8;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm512_loadu_pd(&container_a[i * group_len] as *const f64);
        let b_vector = _mm512_loadu_pd(&container_b[i * group_len] as *const f64);
        _mm512_store_pd(
            &mut container_a[i * group_len] as *mut f64,
            _mm512_sub_pd(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] + container_b[group * group_len + i];
    }
    container_a
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn sub_f64_avx<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be subed because the lengths are unequal");
    }
    let group_len = 4;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm256_loadu_pd(&container_a[i * group_len] as *const f64);
        let b_vector = _mm256_loadu_pd(&container_b[i * group_len] as *const f64);
        _mm256_storeu_pd(
            &mut container_a[i * group_len] as *mut f64,
            _mm256_sub_pd(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] - container_b[group * group_len + i];
    }
    container_a
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn sub_f64_sse<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be subed because the lengths are unequal");
    }
    let group_len = 2;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = _mm_loadu_pd(&container_a[i * group_len] as *const f64);
        let b_vector = _mm_loadu_pd(&container_b[i * group_len] as *const f64);
        _mm_store_pd(
            &mut container_a[i * 4] as *mut f64,
            _mm_sub_pd(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] - container_b[group * group_len + i];
    }
    container_a
}

fn sub_f64_without_simd<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
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
        T: IndexMut<usize, Output = f64> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be subed because the lengths are unequal");
    }
    let group_len = 2;
    let group = len_a / group_len;
    for i in 0..group {
        let a_vector = vld1q_s32(&container_a[i * group_len] as *const f64);
        let b_vector = vld1q_s32(&container_b[i * group_len] as *const f64);
        vst1q_s32(
            &mut container_a[i * group_len] as *mut f64,
            vsub_s32(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * group_len {
        container_a[group * group_len + i] = container_a[group * group_len + i] - container_b[group * group_len + i];
    }
    container_a
}