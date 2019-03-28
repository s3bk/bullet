/*
#![feature(proc_macro)]

extern crate tuple;
extern crate jack;
extern crate termion;
extern crate math_traits;
extern crate bullet_macros;

use bullet_macros::math;
use tuple::*;
use math_traits::Real;
use std::sync::mpsc::channel;
use termion::event::Key;
use termion::input::TermRead;
use jack::{Client, ClientOptions, ClosureProcessHandler, ProcessScope, AsyncClient, AudioIn, AudioOut, Control};

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
fn main() {
    let (tx, rx) = channel();

    let (client, _) = Client::new("Duffing", ClientOptions::empty()).unwrap();
    let port_in = client.register_port("duffing_in", AudioIn).unwrap();
    let mut port_out_x = client.register_port("duffing_out_x", AudioOut).unwrap();
    let mut port_out_y = client.register_port("duffing_out_y", AudioOut).unwrap();
    
    let dt = 880.0 / (client.sample_rate() as f32);
    let mut x = T2(0.2, 0.2);
    let mut p = DuffingParams::default();
    let scale = 20.;
    let scale_inv = 1.0 / scale;
    let process = ClosureProcessHandler::new(move |_: &Client, ps: &ProcessScope| {
        if let Ok(params) = rx.try_recv() {
            p = params;
        }
        
        let iter = T3(port_in.as_slice(ps).iter(), port_out_x.as_mut_slice(ps).iter_mut(), port_out_y.as_mut_slice(ps).iter_mut());
        for T3(&sample_in, sample_out_x, sample_out_y) in iter {
            let drive = sample_in * scale;
            let dx_dt = T2(
                x.1,
                p.epsilon * drive - p.lambda * x.1 - x.0 * (p.alpha + (x.0 * x.0 * p.beta))
            );
            x += dx_dt * dt;
            *sample_out_x = (x.0 * scale_inv).clamp(-1.0, 1.0);
            *sample_out_y = (x.1 * scale_inv).clamp(-1.0, 1.0);
        }
        Control::Continue
    });
    let _active_client = AsyncClient::new(client, (), process).unwrap();
    
    let stdin = std::io::stdin();
    
    let mut params = DuffingParams::default();
    
    let update = |p: &mut DuffingParams, idx, fac| {
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
*/
fn main(){}
