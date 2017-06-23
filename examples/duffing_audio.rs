#![feature(non_ascii_idents)]
#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]
#![feature(box_syntax)]

extern crate tuple;
extern crate math;
extern crate cpal;
extern crate futures;
extern crate termion;


use tuple::{T2, TupleElements};
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
use std::sync::mpsc::{channel, Sender, Receiver};
use std::io::Read;

struct MyExecutor;

impl Executor for MyExecutor {
    fn execute(&self, r: Run) {
        r.run();
    }
}

#[derive(Copy, Clone)]
struct DuffingParams {
    epsilon:    f32,
    lambda:     f32,
    omega:      f32,
    alpha:      f32,
    beta:       f32
}
impl Default for DuffingParams {
    fn default() -> DuffingParams {
        DuffingParams {
            epsilon: 7.72,
            lambda:  0.2,
            omega:   1.0,
            alpha:   0.1,
            beta:    1.0
        }
    }
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

fn control(tx: Sender<DuffingParams>) {
    use termion::event::Key;
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;
    
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    
    let mut params = DuffingParams::default();
    
    let mut update = |p: &mut DuffingParams, idx, fac| {
        {
            let f = match idx {
                0 => &mut p.epsilon,
                1 => &mut p.lambda,
                2 => &mut p.omega,
                3 => &mut p.alpha,
                4 => &mut p.beta,
                _ => panic!()
            };
            *f *= fac;
        }
        println!(
            "epsilon: {:6.4}  lambda: {:6.4}  omega: {:6.4}  alpha: {:6.4}  beta: {:6.4}\r",
            p.epsilon, p.lambda, p.omega, p.alpha, p.beta
        );
        tx.send(*p).unwrap();
    };
    
    let mut idx = 0;
    
    for evt in stdin.keys() {
        match evt.unwrap() {
            Key::Char('1') => idx = 0,
            Key::Char('2') => idx = 1,
            Key::Char('3') => idx = 2,
            Key::Char('4') => idx = 3,
            Key::Char('5') => idx = 4,
            Key::Up => update(&mut params, idx, 1.1),
            Key::Down => update(&mut params, idx, 1.0 / 1.1),
            Key::Esc => break,
            Key::Char('q') => break,
            _ => ()
        }
        
        let name = match idx {
            0 => "epsilon",
            1 => "lambda",
            2 => "omega",
            3 => "alpha",
            4 => "beta",
            _ => panic!()
        };
        
        println!("selected: {}\r", name);
    }
}

fn main() {
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list().unwrap()
    .find(|fmt| fmt.samples_rate.0 == 48000 && fmt.channels.len() == 2)
    .expect("Failed to get endpoint format");

    let event_loop = cpal::EventLoop::new();
    let executor = Arc::new(MyExecutor);

    let (mut voice, stream) = cpal::Voice::new(&endpoint, &format, &event_loop).expect("Failed to create a voice");

    println!("format: {:?}", format);
    
    let (tx, rx) = channel();
    thread::spawn(move || control(tx));
    
    // Produce a sinusoid of maximum amplitude.
    let samples_rate = format.samples_rate.0 as f32;
    let mut integrator = Integration::new(
        duffing(DuffingParams::default()), // the function to integrate
        T2(1.0, 1.0), // initial value
        0.0, // inital time
        440. / samples_rate // step size
    );
    
    voice.play();
    task::spawn(stream.for_each(move |buffer| -> Result<_, ()> {
        if let Ok(params) = rx.try_recv() {
            let t = integrator.t;
            let y = integrator.y;
            
            integrator = Integration::new(
                duffing(params), // the function to integrate
                y, // initial value
                t, // inital time
                440. / samples_rate // step size
            );
        }
        let mut data_source = integrator.by_ref()
        .map(|v| v * T2(0.1, 0.1));
        
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
