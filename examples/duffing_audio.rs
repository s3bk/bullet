#![feature(non_ascii_idents)]
#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]
#![feature(box_syntax)]

extern crate tuple;
extern crate num;
extern crate math;
extern crate cpal;
extern crate futures;

use tuple::{T2, TupleElements};
use num::Num;
use math::integrate::Integration;
use math::real::Real;
use math::cast::Cast;
use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct MyExecutor;

impl Executor for MyExecutor {
    fn execute(&self, r: Run) {
        r.run();
    }
}

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
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list().unwrap()
    .find(|fmt| fmt.samples_rate.0 == 48000)
    .expect("Failed to get endpoint format");

    let event_loop = cpal::EventLoop::new();
    let executor = Arc::new(MyExecutor);

    let (mut voice, stream) = cpal::Voice::new(&endpoint, &format, &event_loop).expect("Failed to create a voice");

    println!("format: {:?}", format);
    
    // Produce a sinusoid of maximum amplitude.
    let samples_rate = format.samples_rate.0 as f32;
    let mut data_source = Integration::new(
        duffing(7.72, 0.2, 1.0, 0.0, 1.0), // the function to integrate
        T2(1.0, 1.0), // initial value
        0.0, // inital time
        440. / samples_rate // step size
    ).map(|v| v * T2(0.25, 0.16666));
    
    voice.play();
    task::spawn(stream.for_each(move |buffer| -> Result<_, ()> {
        match buffer {
            cpal::UnknownTypeBuffer::U16(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
                    let value: T2<u16, u16> = value.map(|f|
                        (0.5 * f + 0.5) * (std::u16::MAX as f32)
                    ).cast().unwrap();
                    
                    for (ch, out) in sample.iter_mut().enumerate() {
                        *out = value.get(ch).cloned().unwrap_or(0);
                    }
                }
            },

            cpal::UnknownTypeBuffer::I16(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
                    let value: T2<i16, i16> = value.map(|f| 
                        f * (std::i16::MAX as f32)
                    ).cast().unwrap();
                    
                    for (ch, out) in sample.iter_mut().enumerate() {
                        *out = value.get(ch).cloned().unwrap_or(0);
                    }
                }
            },

            cpal::UnknownTypeBuffer::F32(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
                    for (ch, out) in sample.iter_mut().enumerate() {
                        *out = value.get(ch).cloned().unwrap_or(0.0);
                    }
                }
            },
        };

        Ok(())
    })).execute(executor);

    event_loop.run();
}
