//avx2 i32 加法
#![feature(stdsimd)]
use std::arch::x86_64::*;
use std::ops::IndexMut;

//T是某种连续储存数据的容器,参数为容器的引用，第一个为可变引用，储存运算结果
unsafe fn add<'a,T>(container_a: &'a mut T,container_b: &T) -> &'a T
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
        let a_vector = _mm256_loadu_epi32(container_a[i * 8] as *const i32);
        let b_vector = _mm256_loadu_epi32(container_b[i * 8] as *const i32);
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
fn main() {
     let mut a = vec![1,2,3];
     let b = vec![1,2,3];
     unsafe {
         add(&mut a,&b);
         println!("{:?}",a);
     }

    // let mut c = [1,2,3];
    // let mut d = [1,2,3];
    // unsafe {
    //     add(& mut c,&d);
    //     println!("{:?}",c);
    // }
    let mut c = [1,2,3];
    let d = [1,2,3];
    let slice_c = &mut c[0..3];
    let slice_d = &d[0..3];
    unsafe {
        add(slice_c,slice_d);
        println!("{:?}",slice_c);
    }
}
