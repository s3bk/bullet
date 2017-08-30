#![feature(non_ascii_idents)]
#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]
#![feature(box_syntax)]

extern crate tuple;
extern crate math;
extern crate hound;

use tuple::{T2, TupleElements, Map};
use math::integrate::Integration;
use math::real::Real;
use math::cast::Cast;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

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
    use std::intrinsics::{fmul_fast, cosf32};
    move |t, s| {
        unsafe {
            T2(
                s.1,
                fmul_fast(p.epsilon, cosf32(fmul_fast(p.omega, t)))
                - fmul_fast(p.lambda, s.1)
                - fmul_fast(s.0, p.alpha + fmul_fast(fmul_fast(s.0, s.0), p.beta))
            )
        }
    }
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
