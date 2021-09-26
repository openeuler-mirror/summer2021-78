// use std::time;
// fn main() {
//     let len = 33090;
//     let mut a = vec![1;len];
//     let b = vec![1;len];
//     let mut c = vec![0;len];
//     let start = time::SystemTime::now();
//     for i in 0..len {
//         c[i] = a[i] + b[i];
//     }
//     let end = time::SystemTime::now();
//     let time1 = end.duration_since(start).unwrap();
//     let start = time::SystemTime::now();
//     usimd::add_i32(&mut a,&b);
//     let end = time::SystemTime::now();
//     let time2 = end.duration_since(start).unwrap();
//     assert_eq!(a,c);
//     let up = time1.as_nanos() as f64 / time2.as_nanos() as f64 * 100. ;
//     println!("不使用usimd:{:?}, 使用usimd:{:?}, 效率提升: {}%", time1, time2, up)
// }

use std::time;
use usimd::*;
fn main() {
    let mut pi = 0.0;
    let dt = 1e8;
    let delta = 1.0 / dt;
    let start = time::SystemTime::now();
    for i in 0..dt as i64 {
        let x = i as f64 / dt;
        pi = pi + delta / (1.0 + x * x);
    }
    pi = pi * 4.0;
    let end = time::SystemTime::now();
    let time1 = end.duration_since(start).unwrap();
    println!("不使用usimd计算出的pi值：{}",pi);
    let start = time::SystemTime::now();
    let simd_feature = simd_feature_detected();
    unsafe {
        let ymm0 = set1_pd(1.0, simd_feature);
        let ymm1 = set1_pd(delta, simd_feature);
        let ymm2 = set_pd(
            delta * 3.0,
            delta * 2.0,
            delta,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            simd_feature,
        );
        let mut ymm4 = setzero_pd(simd_feature);
        let mut i = 0;
        while i <= dt as i64 - 4 {
            let mut ymm3 = set1_pd(i as f64 * delta, simd_feature);
            ymm3 = add_pd(&ymm3, &ymm2);
            ymm3 = mul_pd(&ymm3, &ymm3);
            ymm3 = add_pd(&ymm0, &ymm3);
            ymm3 = div_pd(&ymm1, &ymm3);
            ymm4 = add_pd(&ymm4, &ymm3);
            i = i + 4;
        }
        let mut result: [f64; 4] = [0.; 4];
        store_pd(result.as_mut_ptr(), ymm4);
        pi = result.iter().sum();
        pi = pi * 4.0;
        let end = time::SystemTime::now();
        let time2 = end.duration_since(start).unwrap();
        println!("使用usimd计算出的pi值：{}",pi);
        let up = time1.as_nanos() as f64 / time2.as_nanos() as f64 * 100. ;
        println!("不使用usimd:{:?}, 使用usimd:{:?}, 效率提升: {}%", time1, time2, up);
    }
}
