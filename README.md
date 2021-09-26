# Summer2021-No.78 Rust-usimd项目建设

## 使用usimd计算与不使用usimd计算的效率对比

```Rust
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
```
```
vpt@vpt-PC:~/usimd/example/contrast$ cargo run --release
   Compiling cfg-if v0.1.10
   Compiling len-trait v0.6.1
   Compiling usimd v0.1.0 (/home/vpt/usimd)
   Compiling contrast v0.1.0 (/home/vpt/usimd/example/contrast)
   Finished release [optimized] target(s) in 0.28s
     Running `target/release/contrast`
不使用usimd:69µs, 使用usimd:23.306µs, 效率提升: 296.0611001458852%

```
