
extern crate bullet;


fn main() {
    #[cfg(feature="ptx")]
    {
    use std::env;
    use bullet::builder::Builder;
    use bullet::rt::ptx::bench_ptx;
        let pow: u32 = env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(10u32);
        let b = Builder::new();
        let n = b.parse("sin(x^4)^2 + cos(3*x-5)").unwrap();
        let num_points = 1usize << pow;
        println!("{}: {}ms", num_points, 1000. * bench_ptx(&n, num_points));
    }
}
