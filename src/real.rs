use std::ops::{Add, Sub, Mul, Div};

use simd::{f32x4, bool32fx4, u32x4, i32x4};
//impl_simd!(f32x4 : T4(a,b,c,d,));

use simd::x86::sse2::{f64x2, u64x2, i64x2};
//impl_simd!(f64x2 : T2(a,b,));

use simd::x86::avx::{f32x8, f64x4};
//impl_simd!(f32x8 : T8(a,b,c,d, e,f,g,h,));
//impl_simd!(f64x4 : T4(a,b,c,d,));

use simd::x86::sse2::Sse2F64x2;

use rand::{Rand, Rng};
use rand::distributions::{Sample, IndependentSample, Range as Uniform};
use std::fmt::Debug;


use tuple::*;
//Float + NumCast + SampleRange + PartialOrd + Clone + Add + Debug
pub trait Real:
    Sized + Copy + Debug
  + Mul<Output=Self> + Add<Output=Self> + Sub<Output=Self> + Div<Output=Self>
{
    const PI: Self;
    type Bool;
    
    fn int(v: i16) -> Self;
    fn frac(nom: i16, denom: u16) -> Self;
    
    fn uniform01<R: Rng>(rng: &mut R) -> Self;
    
    fn wrap(self, at: Self, span: Self) -> Self;
    
    fn broadcast<O>(self) -> O where O: Splat<Self> {
        O::splat(self)
    }
    
    fn clamp(self, min: Self, max: Self) -> Self;
    
    fn lt(self, rhs: Self) -> Self::Bool;
    fn le(self, rhs: Self) -> Self::Bool;
    fn gt(self, rhs: Self) -> Self::Bool;
    fn ge(self, rhs: Self) -> Self::Bool;
    fn eq(self, rhs: Self) -> Self::Bool;
}

pub trait Splat<E> {
    fn splat(e: E) -> Self;
}

macro_rules! impl_real {
    ($($t:ident: $s:ident),*) => ( $(
        impl Real for $t {
            const PI: Self = ::std::$t::consts::PI;
            type Bool = bool;
            
            fn int(v: i16) -> Self { v.into() }
            fn frac(nom: i16, denom: u16) -> Self {
                $t::from(nom) / $t::from(denom)
            }
            
            fn wrap(self, at: Self, span: Self) -> Self {
                if self > at { self - span } else { self }
            }
            
            fn uniform01<R: Rng>(rng: &mut R) -> Self {
                let uniform01 = Uniform::new(0., 1.);
                uniform01.ind_sample(rng)
            }
            #[inline(always)]
            fn clamp(self, min: Self, max: Self) -> Self {
                let max_ = $s::splat(max);
                let min_ = $s::splat(min);
                $s::splat(self).max(min_).min(max_).extract(0) as Self
            }
            fn lt(self, rhs: Self) -> Self::Bool { self < rhs }
            fn le(self, rhs: Self) -> Self::Bool { self <= rhs }
            fn gt(self, rhs: Self) -> Self::Bool { self > rhs }
            fn ge(self, rhs: Self) -> Self::Bool { self >= rhs }
            fn eq(self, rhs: Self) -> Self::Bool { self == rhs }
        }
    )* )
}

impl_real!(f32: f32x4, f64: f64x2);

impl Splat<f32> for f32x4 {
    fn splat(e: f32) -> f32x4 {
        f32x4::splat(e)
    }
}
impl Real for f32x4 {
    const PI: Self = f32x4::splat(::std::f32::consts::PI);
    type Bool = bool32fx4;
    
    fn int(v: i16) -> Self { f32::from(v).broadcast() }
    fn frac(nom: i16, denom: u16) -> Self {
        (f32::from(nom) / f32::from(denom)).broadcast()
    }
    
    fn wrap(self, at: Self, span: Self) -> Self {
        self.gt(at).select(self - span, self)
    }
    
    fn uniform01<R: Rng>(rng: &mut R) -> Self {
        let uniform01 = Uniform::new(0., 1.);
        let a = uniform01.ind_sample(rng);
        let b = uniform01.ind_sample(rng);
        let c = uniform01.ind_sample(rng);
        let d = uniform01.ind_sample(rng);
        f32x4::new(a, b, c, d)
    }
    
    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }
    
    fn lt(self, rhs: Self) -> Self::Bool { f32x4::lt(self, rhs) }
    fn le(self, rhs: Self) -> Self::Bool { f32x4::le(self, rhs) }
    fn gt(self, rhs: Self) -> Self::Bool { f32x4::gt(self, rhs) }
    fn ge(self, rhs: Self) -> Self::Bool { f32x4::ge(self, rhs) }
    fn eq(self, rhs: Self) -> Self::Bool { f32x4::eq(self, rhs) }
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
*/
use tuple::*;
macro_rules! tuple_init {
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
    
        impl<$($T,)*> Real for $Tuple<$($T,)*>
        where $( $T: Real ),*
        {
            const PI: Self = $Tuple( $($T::PI,)* );
            type Bool = $Tuple<$($T::Bool),*>;
    
            fn int(v: i16) -> Self {
                $Tuple( $($T::int(v),)* )
            }
            fn frac(nom: i16, denom: u16) -> Self {
                $Tuple( $($T::frac(nom, denom),)* )
            }
    
            fn uniform01<R: Rng>(rng: &mut R) -> Self {
                $Tuple( $($T::uniform01(rng),)* )
            }
    
            fn wrap(self, at: Self, span: Self) -> Self {
                $Tuple( $($T::wrap(self.$idx, at.$idx, span.$idx),)* )
            }
    
            fn clamp(self, min: Self, max: Self) -> Self {
                $Tuple( $($T::clamp(self.$idx, min.$idx, max.$idx),)* )
            }
            
            fn lt(self, rhs: Self) -> Self::Bool {
                $Tuple( $($T::lt(self.$idx, rhs.$idx),)* )
            }
            fn le(self, rhs: Self) -> Self::Bool {
                $Tuple( $($T::le(self.$idx, rhs.$idx),)* )
            }
            fn gt(self, rhs: Self) -> Self::Bool {
                $Tuple( $($T::gt(self.$idx, rhs.$idx),)* )
            }
            fn ge(self, rhs: Self) -> Self::Bool {
                $Tuple( $($T::ge(self.$idx, rhs.$idx),)* )
            }
            fn eq(self, rhs: Self) -> Self::Bool {
                $Tuple( $($T::eq(self.$idx, rhs.$idx),)* )
            }
        }
        impl<E> Splat<E> for $Tuple<$(first_i!(E, $T),)*>
        where E: Clone
        {
            fn splat(e: E) -> Self {
                $Tuple( $( first_e!(e.clone(), $idx), )* )
            }
        }
    )*)
}
impl_tuple!(tuple_init);
