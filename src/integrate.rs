use std::ops::*;
use prelude::*;

pub struct Integration<V, F, N> {
    f:  F,
pub t:  N,
pub y:  V,
    h:  N,
    h_half: N,
    h_third: N,
    h_sixth: N,
    wrap: N
}
impl<V, F, N> Integration<V, F, N> where N: Real {
    pub fn new(f: F, s0: V, t0: N, dt: N, wrap: N) -> Integration<V, F, N> {
        Integration {
            f:  f,
            t:  t0,
            y:  s0,
            h:  dt,
            h_half: dt * N::frac(1, 2),
            h_third: dt * N::frac(1, 3),
            h_sixth: dt * N::frac(1, 6),
            wrap:   wrap
        }
    }
}
impl<V, F, N> Iterator for Integration<V, F, N> where
    V: Real<Scalar=N>,
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
        let k2 = f(t + h_half, y + k1 * V::splat(h_half));
        let k3 = f(t + h_half, y + k2 * V::splat(h_half));
        let k4 = f(t + h, y + k3 * V::splat(h));
        
        self.y = y + (k1 + k4) * V::splat(h_sixth) + (k2 + k3) * V::splat(h_third);
        self.t = (t + h).wrap(self.wrap, self.wrap + self.wrap);
        
        Some(self.y)
    }
}

