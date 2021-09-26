use std::time;
fn main() {
    let len = 33090;
    let mut a = vec![1;len];
    let b = vec![1;len];
    let mut c = vec![0;len];
    let start = time::SystemTime::now();
    for i in 0..len {
        c[i] = a[i] + b[i];
    }
    let end = time::SystemTime::now();
    let time1 = end.duration_since(start).unwrap();
    let start = time::SystemTime::now();
    usimd::add_i32(&mut a,&b);
    let end = time::SystemTime::now();
    let time2 = end.duration_since(start).unwrap();
    assert_eq!(a,c);
    let up = time1.as_nanos() as f64 / time2.as_nanos() as f64 * 100. ;
    println!("不使用usimd:{:?}, 使用usimd:{:?}, 效率提升: {}%", time1, time2, up)
}
