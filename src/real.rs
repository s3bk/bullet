use std::ops::{Add, Sub, Mul, Div};

use rand::{Rng};
use rand::distributions::{IndependentSample, Range as Uniform};
use std::fmt::Debug;

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

macro_rules! first_t {
    ($A:ty, $B:tt) => ($A)
}
macro_rules! first_i {
    ($A:ident, $B:tt) => ($A)
}
macro_rules! first_e {
    ($a:expr, $b:tt) => ($a)
}

macro_rules! impl_simd {
    ($($simd:ident: $scalar:ident, $bool:ty, $trait:ident, $Tuple:ident($($idx:tt)*));*) => ( $(
        impl Real for $simd {
            const PI: Self = $simd::splat(::std::$scalar::consts::PI);
            type Bool = $bool;
            type Scalar = $scalar;
            type Iterator = IntoElements<$Tuple<$(first_t!($scalar, $idx)),*>>;

            fn splat(s: Self::Scalar) -> Self {
                $simd::splat(s)
            }
            
            fn values(self) -> Self::Iterator {
                $Tuple::from(self).into_elements()
            }
            
            fn int(v: i16) -> Self { Self::splat($scalar::from(v)) }
            fn frac(nom: i16, denom: u16) -> Self {
                Self::splat($scalar::from(nom) / $scalar::from(denom))
            }
            
            fn wrap(self, at: Self, span: Self) -> Self {
                self.gt(at).select(self - span, self)
            }
            
            fn uniform01<R: Rng>(rng: &mut R) -> Self {
                let uniform01 = Uniform::new(0., 1.);
                $simd::new($(first_e!(uniform01.ind_sample(rng), $idx)),*)
            }

            fn abs(self) -> Self {
                self.le(Self::splat(0.0)).select(-self, self)
            }
            fn sqrt(self) -> Self {
                $trait::sqrt(self)
            }
            
            fn min(self, other: Self) -> Self {
                $trait::min(self, other)
            }
            fn max(self, other: Self) -> Self {
                $trait::max(self, other)
            }
            
            fn lt(self, rhs: Self) -> Self::Bool { $simd::lt(self, rhs) }
            fn le(self, rhs: Self) -> Self::Bool { $simd::le(self, rhs) }
            fn gt(self, rhs: Self) -> Self::Bool { $simd::gt(self, rhs) }
            fn ge(self, rhs: Self) -> Self::Bool { $simd::ge(self, rhs) }
            fn eq(self, rhs: Self) -> Self::Bool { $simd::eq(self, rhs) }
            
            fn select(self, other: Self, cond: Self::Bool) -> Self {
                cond.select(self, other)
            }
        }
    )* )
}
        
#[cfg(target_feature = "mmx")]
use simd::{f32x4, bool32fx4, u32x4, i32x4};
#[cfg(target_feature = "mmx")]
impl_simd!(f32x4: f32, bool32fx4, Sse2F32x4, T4(0 1 2 3));

#[cfg(target_feature = "sse2")]
use simd::x86::sse2::{f64x2, bool64fx2, Sse2F64x2};
#[cfg(target_feature = "sse2")]
impl_simd!(f64x2: f64, bool64fx2, Sse2F64x2, T2(0 1));

#[cfg(target_feature = "avx")]
use simd::x86::avx::{f32x8, f64x4, bool32fx8, bool64fx4, AvxF32x8, AvxF64x4};
#[cfg(target_feature = "avx")]
impl_simd!(
    f32x8: f32, bool32fx8, AvxF32x8, T8(0 1 2 3 4 5 6 7);
    f64x4: f64, bool64fx4, AvxF64x4, T4(0 1 2 3)
);
        
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
