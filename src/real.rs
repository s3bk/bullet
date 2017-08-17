use std::ops::{Add, Sub, Mul, Div};

use simd::{f32x4, bool32fx4, u32x4, i32x4};
//impl_simd!(f32x4 : T4(a,b,c,d,));

#[cfg(target_feature = "sse2")]
use simd::x86::sse2::{f64x2, u64x2, i64x2};
//impl_simd!(f64x2 : T2(a,b,));

#[cfg(target_feature = "avx")]
use simd::x86::avx::{f32x8, f64x4, bool32fx8, AvxF32x8};
//impl_simd!(f32x8 : T8(a,b,c,d, e,f,g,h,));
//impl_simd!(f64x4 : T4(a,b,c,d,));

#[cfg(target_feature = "sse2")]
use simd::x86::sse2::Sse2F64x2;

use rand::{Rand, Rng};
use rand::distributions::{Sample, IndependentSample, Range as Uniform};
use std::fmt::Debug;
use fmath;

use tuple::*;
//Float + NumCast + SampleRange + PartialOrd + Clone + Add + Debug
pub trait Real:
    Sized + Copy + Debug
  + Mul<Output=Self> + Add<Output=Self> + Sub<Output=Self> + Div<Output=Self>
{
    const PI: Self;
    type Bool;
    type Scalar;
    type Iterator: Iterator<Item=Self::Scalar>;

    fn values(self) -> Self::Iterator;
    
    fn int(v: i16) -> Self;
    fn frac(nom: i16, denom: u16) -> Self;
    fn inv(self) -> Self {
        <Self as Real>::int(1) / self
    }
    
    fn uniform01<R: Rng>(rng: &mut R) -> Self;

    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    
    /// if self exeeds at, subtract span
    fn wrap(self, at: Self, span: Self) -> Self;
    
    fn splat(s: Self::Scalar) -> Self;
    
    fn clamp(self, min: Self, max: Self) -> Self {
        let clamped_low = min.select(self, self.lt(min));
        max.select(clamped_low, self.gt(max))
    }
    
    fn lt(self, rhs: Self) -> Self::Bool;
    fn le(self, rhs: Self) -> Self::Bool;
    fn gt(self, rhs: Self) -> Self::Bool;
    fn ge(self, rhs: Self) -> Self::Bool;
    fn eq(self, rhs: Self) -> Self::Bool;

    // if cont true. then select self, otherwhise other
    fn select(self, other: Self, cond: Self::Bool) -> Self;
    fn max(self, other: Self) -> Self {
        self.select(other, self.gt(other))
    }
    fn min(self, other: Self) -> Self {
        self.select(other, self.lt(other))
    }
}

macro_rules! impl_real {
    ($($t:ident: $s:ident),*) => ( $(
        impl Real for $t {
            const PI: Self = ::std::$t::consts::PI;
            type Bool = bool;
            type Scalar = $t;
            type Iterator = ::std::iter::Once<$t>;

            fn splat(s: Self::Scalar) -> Self {
                s
            }
            
            fn values(self) -> Self::Iterator {
                ::std::iter::once(self)
            }
            
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

            fn abs(self) -> Self { self.abs() }
            fn sqrt(self) -> Self { self.sqrt() }
            
            fn lt(self, rhs: Self) -> Self::Bool { self < rhs }
            fn le(self, rhs: Self) -> Self::Bool { self <= rhs }
            fn gt(self, rhs: Self) -> Self::Bool { self > rhs }
            fn ge(self, rhs: Self) -> Self::Bool { self >= rhs }
            fn eq(self, rhs: Self) -> Self::Bool { self == rhs }

            fn select(self, other: Self, cond: Self::Bool) -> Self {
                if cond { self } else { other }
            }
        }
    )* )
}

#[cfg(target_feature = "sse2")]
impl_real!(f32: f32x4, f64: f64x2);

#[cfg(target_feature = "avx")]
impl Real for f32x8 {
    const PI: Self = f32x8::splat(::std::f32::consts::PI);
    type Bool = bool32fx8;
    type Scalar = f32;
    type Iterator = IntoElements<T8<f32, f32, f32, f32, f32, f32, f32, f32>>;

    fn splat(s: Self::Scalar) -> Self {
        f32x8::splat(s)
    }
    
    fn values(self) -> Self::Iterator {
        T8::from(self).into_elements()
    }
    
    fn int(v: i16) -> Self { Self::splat(f32::from(v)) }
    fn frac(nom: i16, denom: u16) -> Self {
        Self::splat(f32::from(nom) / f32::from(denom))
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
        let e = uniform01.ind_sample(rng);
        let f = uniform01.ind_sample(rng);
        let g = uniform01.ind_sample(rng);
        let h = uniform01.ind_sample(rng);
        f32x8::new(a, b, c, d, e, f, g, h)
    }

    fn abs(self) -> Self {
        self.le(Self::splat(0.0f32)).select(-self, self)
    }
    fn sqrt(self) -> Self {
        AvxF32x8::sqrt(self)
    }
    
    fn min(self, other: Self) -> Self {
        AvxF32x8::min(self, other)
    }
    fn max(self, other: Self) -> Self {
        AvxF32x8::max(self, other)
    }
    
    fn lt(self, rhs: Self) -> Self::Bool { f32x8::lt(self, rhs) }
    fn le(self, rhs: Self) -> Self::Bool { f32x8::le(self, rhs) }
    fn gt(self, rhs: Self) -> Self::Bool { f32x8::gt(self, rhs) }
    fn ge(self, rhs: Self) -> Self::Bool { f32x8::ge(self, rhs) }
    fn eq(self, rhs: Self) -> Self::Bool { f32x8::eq(self, rhs) }
    
    fn select(self, other: Self, cond: Self::Bool) -> Self {
        cond.select(self, other)
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
 */


use tuple::*;
macro_rules! tuple_init {
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
    
        impl<T: Real> Real for $Tuple<$(first_i!(T, $T),)*>
        {
            const PI: Self = $Tuple( $(first_e!(T::PI, $T),)* );
            type Bool = $Tuple<$(first_t!(T::Bool, $T)),*>;
            type Scalar = T;
            type Iterator = IntoElements<Self>;

            fn splat(s: Self::Scalar) -> Self {
                $Tuple( $(first_e!(s, $idx),)* )
            }
            fn values(self) -> Self::Iterator {
                self.into_elements()
            }
            
            fn int(v: i16) -> Self {
                $Tuple( $(first_e!(T::int(v), $idx),)* )
            }
            fn frac(nom: i16, denom: u16) -> Self {
                $Tuple( $(first_e!(T::frac(nom, denom), $idx),)* )
            }
    
            fn uniform01<R: Rng>(rng: &mut R) -> Self {
                $Tuple( $(first_e!(T::uniform01(rng), $idx),)* )
            }

            fn abs(self) -> Self {
                $Tuple( $(T::abs(self.$idx)),* )
            }

            fn sqrt(self) -> Self {
                $Tuple( $(T::sqrt(self.$idx)),* )
            }
            
            fn wrap(self, at: Self, span: Self) -> Self {
                $Tuple( $(T::wrap(self.$idx, at.$idx, span.$idx),)* )
            }
    
            fn clamp(self, min: Self, max: Self) -> Self {
                $Tuple( $(T::clamp(self.$idx, min.$idx, max.$idx),)* )
            }
            
            fn lt(self, rhs: Self) -> Self::Bool {
                $Tuple( $(T::lt(self.$idx, rhs.$idx),)* )
            }
            fn le(self, rhs: Self) -> Self::Bool {
                $Tuple( $(T::le(self.$idx, rhs.$idx),)* )
            }
            fn gt(self, rhs: Self) -> Self::Bool {
                $Tuple( $(T::gt(self.$idx, rhs.$idx),)* )
            }
            fn ge(self, rhs: Self) -> Self::Bool {
                $Tuple( $(T::ge(self.$idx, rhs.$idx),)* )
            }
            fn eq(self, rhs: Self) -> Self::Bool {
                $Tuple( $(T::eq(self.$idx, rhs.$idx),)* )
            }

            fn select(self, other: Self, cond: Self::Bool) -> Self {
                $Tuple( $(T::select(self.$idx, other.$idx, cond.$idx),)* )
            }
        }
    )*)
}
impl_tuple!(tuple_init);
