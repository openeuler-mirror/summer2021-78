#![feature(iter_zip)]

fn main() {
    let len = 10000000;
    let mut a = vec![1;len];
    let b = vec![1;len];
    let mut c = vec![0;len];
    for i in 0 ..len {
        c[i] = a[i] + b[i];
    }
    // usimd::add_i32(&mut a,&b);
}
