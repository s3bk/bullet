#![feature(non_ascii_idents)]
#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]
#![feature(box_syntax)]
#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

extern crate tuple;
extern crate math_traits;
extern crate hound;
extern crate bullet;
extern crate bullet_macros;

use tuple::{T2, Map};
use bullet::integrate::Integration;
use math_traits::{Cast, Real};
use bullet_macros::math;

#[derive(Copy, Clone, Debug)]
struct DuffingParams {
    epsilon:    f32,
    lambda:     f32,
    omega:      f32,
    alpha:      f32,
    beta:       f32
}

#[allow(non_snake_case)]
#[inline]
fn duffing(p: DuffingParams)
 -> impl Fn(f32, T2<f32, f32>) -> T2<f32, f32>
{
    let DuffingParams { epsilon: ɛ, lambda: λ, omega: ω, alpha: α, beta: β } = p; 
    move |t, T2(x, y)| T2(x, math!(ɛ cos(ω t) - λ y - α x - β x^3))
}

fn main() {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("duffing.wav", spec).unwrap();
    
    let params = DuffingParams {
        epsilon: 10.446971,
        lambda: 0.00013858214,
        omega: 3.2886522,
        alpha: 0.000000030056544,
        beta: 64.18658
    };
    let mut data_source = Integration::new(
        duffing(params), // the function to integrate
        T2(1.0, 1.0), // initial value
        0.0, // inital time
        440. / spec.sample_rate as f32, // step size,
        f32::PI / params.omega
    ).map(|v| v * T2(0.2, 0.05));
    
    for value in data_source.take(spec.sample_rate as usize * 600) {
        let value: T2<i16, i16> = value.map(|f| 
            f * (std::i16::MAX as f32)
        ).cast().unwrap();
        writer.write_sample(value.0);
        writer.write_sample(value.1);
    }
    writer.finalize().unwrap();
}
