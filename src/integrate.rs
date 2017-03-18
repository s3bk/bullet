use std::ops::*;
use prelude::*;

pub struct Integration<V, F, N> {
    f:  F,
    t:  N,
    y:  V,
    h:  N
}
impl<V, F, N> Integration<V, F, N> {
    pub fn new(f: F, s0: V, t0: N, dt: N) -> Integration<V, F, N> {
        Integration {
            f:  f,
            t:  t0,
            y:  s0,
            h:  dt
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
        use std::f64;
        
        let ref f = self.f;
        let t = self.t;
        let h = self.h;
        let h_half = h / 2.into();
        let h_third = h / 3.into();
        let h_sixth = h / 6.into();
    
        let y = self.y;
        let k1 = f(t, y);
        let k2 = f(t + h_half, y + k1 * h_half.broadcast());
        let k3 = f(t + h_half, y + k2 * h_half.broadcast());
        let k4 = f(t + h, y + k3 * h.broadcast());
        
        self.y = y + (k1 + k4) * h_sixth.broadcast() + (k2 + k3) * h_third.broadcast();
        self.t = (t+h).wrap(N::PI, N::from(2) * N::PI);
        
        Some(self.y)
    }
}

