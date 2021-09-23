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
pub(crate) fn add_i32<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
where
    T: IndexMut<usize, Output = i32> + len_trait::Len + ?Sized,
{
    return if is_x86_feature_detected!("avx512f") {
        unsafe { add_i32_avx512(container_a, container_b) }
    } else if is_x86_feature_detected!("avx") {
        unsafe { add_i32_avx(container_a, container_b) }
    } else if is_x86_feature_detected!("sse") {
        unsafe { add_i32_sse(container_a, container_b) }
    } else {
        add_i32_without_simd(container_a, container_b)
    };
}
#[cfg(target_arch = "arm")]
fn add<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
where
    T: IndexMut<usize, Output = i32> + len_trait::Len + ?Sized,
{
    return if is_arm_feature_detected!("neon") {
        unsafe { add_neon(container_a, container_b) }
    } else {
        add_without_simd(container_a, container_b)
    };
}
#[cfg(target_arch = "aarch64")]
fn add<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
where
    T: IndexMut<usize, Output = i32> + len_trait::Len + ?Sized,
{
    return if is_aarch64_feature_detected!("neon") {
        unsafe { add_neon(container_a, container_b) }
    } else {
        add_without_simd(container_a, container_b)
    };
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn add_i32_avx512<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
where
    T: IndexMut<usize, Output = i32> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be added because the lengths are unequal");
    }

    let group = len_a / 16;
    for i in 0..group {
        let a_vector = _mm512_loadu_epi32(&container_a[i * 16] as *const i32);
        let b_vector = _mm512_loadu_epi32(&container_b[i * 16] as *const i32);
        _mm512_store_epi32(
            &mut container_a[i * 16] as *mut i32,
            _mm512_add_epi32(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * 16 {
        container_a[group * 16 + i] = container_a[group * 16 + i] + container_b[group * 16 + i];
    }
    container_a
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn add_i32_avx<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
where
    T: IndexMut<usize, Output = i32> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be added because the lengths are unequal");
    }

    let group = len_a / 8;
    for i in 0..group {
        let a_vector = _mm256_loadu_epi32(&container_a[i * 8] as *const i32);
        let b_vector = _mm256_loadu_epi32(&container_b[i * 8] as *const i32);
        _mm256_store_epi32(
            &mut container_a[i * 8] as *mut i32,
            _mm256_add_epi32(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * 8 {
        container_a[group * 8 + i] = container_a[group * 8 + i] + container_b[group * 8 + i];
    }
    container_a
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn add_i32_sse<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
where
    T: IndexMut<usize, Output = i32> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be added because the lengths are unequal");
    }

    let group = len_a / 4;
    for i in 0..group {
        let a_vector = _mm_loadu_epi32(&container_a[i * 4] as *const i32);
        let b_vector = _mm_loadu_epi32(&container_b[i * 4] as *const i32);
        _mm_store_epi32(
            &mut container_a[i * 4] as *mut i32,
            _mm_add_epi32(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * 4 {
        container_a[group * 4 + i] = container_a[group * 4 + i] + container_b[group * 4 + i];
    }
    container_a
}

fn add_i32_without_simd<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
where
    T: IndexMut<usize, Output = i32> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be added because the lengths are unequal");
    }
    for i in 0..len_a {
        container_a[i] = container_a[i] + container_b[i];
    }
    container_a
}
#[cfg(any(target_arch = "arm", target_arch = "aarch"))]
unsafe fn add_neon<'a, T>(container_a: &'a mut T, container_b: &T) -> &'a T
    where
        T: IndexMut<usize, Output = i32> + len_trait::Len + ?Sized,
{
    let len_a = container_a.len();
    let len_b = container_b.len();
    if len_a != len_b {
        panic!("Vectors can't be added because the lengths are unequal");
    }

    let group = len_a / 4;
    for i in 0..group {
        let a_vector = vld1q_s32(&container_a[i * 4] as *const i32);
        let b_vector = vld1q_s32(&container_b[i * 4] as *const i32);
        vst1q_s32(
            &mut container_a[i * 4] as *mut i32,
            vadd_s32(a_vector, b_vector),
        );
    }
    for i in 0..len_a - group * 4 {
        container_a[group * 4 + i] = container_a[group * 4 + i] + container_b[group * 4 + i];
    }
    container_a
}
