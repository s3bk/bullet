use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialOrd;
use std::fmt;

use rand::{Rand, Rng};
use rand::distributions::{Sample, IndependentSample};

pub trait Real:
    Sized + Copy
  + Mul<Output=Self> + Add<Output=Self> + Sub<Output=Self> + Div<Output=Self>
  + From<u16>
{
    const PI: Self;
    
    fn wrap(self, at: Self, span: Self) -> Self;
    
    fn broadcast<O>(self) -> O where O: Splat<Self> {
        O::splat(self)
    }
}

pub trait Splat<E> {
    fn splat(e: E) -> Self;
}

impl Real for f32 {
    const PI: f32 = ::std::f32::consts::PI;
    
    fn wrap(self, at: Self, span: Self) -> Self {
        if self > at { self - span } else { self }
    }
}

impl Real for f64 {
    const PI: f64 = ::std::f64::consts::PI;
    
    fn wrap(self, at: Self, span: Self) -> Self {
        if self > at { self - span } else { self }
    }
}


macro_rules! first_t {
    ($A:ty, $B:ty) => ($A)
}
macro_rules! first_i {
    ($A:ident, $B:ident) => ($A)
}
macro_rules! first_e {
    ($a:expr, $b:tt) => ($a)
}
/*
macro_rules! impl_simd {
    ($name:ident : $Tuple:ident( $($n:ident,)* ) ) => (
        impl Real for $name {
            const PI: $name = $name::csplat(<<$name as Simd>::Elem as Real>::PI);
            
            #[inline(always)]
            fn wrap(self, at: Self, span: Self) -> Self {
                self.le(at).select(self, self-span)
            }
        }
        impl Splat<<$name as Simd>::Elem> for $name {
            #[inline(always)]
            fn splat(x: <$name as Simd>::Elem) -> $name {
                $name::csplat(x)
            }
        }
        
        impl From<$name> for $Tuple< $(first_t!(<$name as Simd>::Elem, $n),)* >
        {
            #[inline(always)]
            fn from(s: $name) -> Self {
                let mut i = 0;
                $( let $n = s.extract(i); i += 1; )*
                $Tuple($( $n, )*)
            }
        }
        
    )
}

use simd::{f32x4};
impl_simd!(f32x4 : T4(a,b,c,d,));

use simd::x86::sse2::{f64x2};
impl_simd!(f64x2 : T2(a,b,));

use simd::x86::avx::{f32x8, f64x4};
impl_simd!(f32x8 : T8(a,b,c,d, e,f,g,h,));
impl_simd!(f64x4 : T4(a,b,c,d,));
*/

use tuple::*;
macro_rules! tuple_init {
    ($($Tuple:ident { $($T:ident . $idx:tt),* } )*) => ($(
    
        impl<$($T,)*> Real for $Tuple<$($T,)*>
        where $( $T: Real + 'static, )*
        {
            const PI: Self = $Tuple( $($T::PI,)* );
        
            fn wrap(self, at: Self, span: Self) -> Self {
                $Tuple( $( self.$idx.wrap(at.$idx, span.$idx), )* )
            }
        }
        impl<E> Splat<E> for $Tuple<$(first_i!(E, $T),)*>
        where E: Clone
        {
            fn splat(e: E) -> Self {
                $Tuple( $( first_e!(e.clone(), $idx), )* )
            }
        }
        
        /*
        impl<$($T,)*> Rand for $Tuple<$($T,)*>
        where $( $T: Rand, )*
        {
            fn rand<R: Rng>(rng: &mut R) -> Self {
                $Tuple( $( $T::rand(rng), )* )
            }
        }
        impl<$($T,)* S> Sample<S> for $Tuple<$($T,)*>
        where $( $T: Sample<S>, )*
        {
            fn sample<R: Rng>(&mut self, rng: &mut R) -> S {
                $Tuple( $( self.$idx.sample(rng), )* )
            }
        }
        impl<$($T,)* S> IndependentSample<S> for $Tuple<$($T,)*>
        where $( $T: IndependentSample<S>, )*
        {
            fn ind_sample<R: Rng>(&self, rng: &mut R) -> S {
                $Tuple( $( self.$idx.ind_sample(rng), )* )
            }
        }*/
    )*)
}
impl_tuple!(tuple_init);
