#![feature(non_ascii_idents)]
#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]
#![feature(box_syntax)]

extern crate tuple;
extern crate math;
extern crate hound;

use tuple::{T2, TupleElements};
use math::integrate::Integration;
use math::real::Real;
use math::cast::Cast;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[allow(non_snake_case)]
#[inline]
fn duffing(ɛ: f32, λ: f32, Ω: f32, α: f32, β: f32)
 -> impl Fn(f32, T2<f32, f32>) -> T2<f32, f32>
{
    use std::intrinsics::{fmul_fast, cosf32};
    move |t, s| {
        unsafe {
            T2(
                s.1,
                fmul_fast(ɛ, cosf32(fmul_fast(Ω, t)))
                - fmul_fast(λ, s.1)
                - fmul_fast(s.0, α + fmul_fast(fmul_fast(s.0, s.0), β))
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
    
    let mut data_source = Integration::new(
        duffing(7.52, 0.2, 1.0, 0.0, 1.0), // the function to integrate
        T2(1.0, 1.0), // initial value
        0.0, // inital time
        440. / spec.sample_rate as f32 // step size
    ).map(|v| v * T2(0.25, 0.16666));
    
    for value in data_source.take(spec.sample_rate as usize * 60) {
        let value: T2<i16, i16> = value.map(|f| 
            f * (std::i16::MAX as f32)
        ).cast().unwrap();
        writer.write_sample(value.0);
        writer.write_sample(value.1);
    }
    writer.finalize().unwrap();
}
