use std::ops::*;
use prelude::*;

pub struct Integration<V, F, N> {
    f:  F,
pub t:  N,
pub y:  V,
    h:  N,
    h_half: N,
    h_third: N,
    h_sixth: N
}
impl<V, F, N> Integration<V, F, N> where N: Real {
    pub fn new(f: F, s0: V, t0: N, dt: N) -> Integration<V, F, N> {
        Integration {
            f:  f,
            t:  t0,
            y:  s0,
            h:  dt,
            h_half: dt * N::frac(1, 2),
            h_third: dt * N::frac(1, 3),
            h_sixth: dt * N::frac(1, 6)
        }
    }
}
impl<V, F, N> Iterator for Integration<V, F, N> where
    V: Real + Splat<N>,
    F: Fn(N, V) -> V,
    N: Real
{
    type Item = V;
    fn next(&mut self) -> Option<V> {
        let ref f = self.f;
        let t = self.t;
        let h = self.h;
        let h_half = self.h_half;
        let h_third = self.h_third;
        let h_sixth = self.h_sixth;
    
        let y = self.y;
        let k1 = f(t, y);
        let k2 = f(t + h_half, y + k1 * h_half.broadcast());
        let k3 = f(t + h_half, y + k2 * h_half.broadcast());
        let k4 = f(t + h, y + k3 * h.broadcast());
        
        self.y = y + (k1 + k4) * h_sixth.broadcast() + (k2 + k3) * h_third.broadcast();
        self.t = (t+h).wrap(N::PI, N::int(2) * N::PI);
        
        Some(self.y)
    }
}

