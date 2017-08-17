use std::ops::RangeInclusive;

pub trait Cast<O>: Sized {
    /// Try to represent self as O.
    /// If possibe, return Some(Self as O) otherwhise None.
    fn cast(self) -> Option<O>;
    
    /// Try to represent self within the range r of O.
    /// If possibe, return Some(self as O) otherwhise None.
    fn cast_clipped(self, r: RangeInclusive<O>) -> Option<O>;
    
    /// Represent self in the range r
    /// If Self is not in r, choose the nearest end of r.
    /// (returns r.start <= self as O <= r.end)
    fn cast_clamped(self, r: RangeInclusive<O>) -> O;
}

macro_rules! impl_cast_unchecked {
    ($($src:ty as [$($dst:ty),*],)*) => (
        $( $(
            impl Cast<$dst> for $src {
                #[inline(always)]
                fn cast(self) -> Option<$dst> {
                    Some(self as $dst)
                }
                #[inline(always)]
                fn cast_clipped(self, r: RangeInclusive<$dst>) -> Option<$dst> {
                    let v = self as $dst;
                    if v >= r.start && v <= r.end {
                        Some(v)
                    } else {
                        None
                    }
                }
                #[inline(always)]
                fn cast_clamped(self, r: RangeInclusive<$dst>) -> $dst {
                    let v = self as $dst;
                    if v < r.start {
                        r.start
                    } else if  v > r.end {
                        r.end
                    } else {
                        v
                    }
                }
            }
        )* )*
    )
}

macro_rules! impl_cast_checked {
    ($($src:ident as [$($dst:ident),*],)*) => (
        $( $(
            impl Cast<$dst> for $src {
                #[inline(always)]
                fn cast(self) -> Option<$dst> {
        		    const MIN: $src = ::std::$dst::MIN as $src;
                    const MAX: $src = ::std::$dst::MAX as $src;
                    if self >= MIN && self <= MAX {
                        Some(self as $dst)
                    } else {
                        None
                    }
                }
                #[inline(always)]
                fn cast_clipped(self, r: RangeInclusive<$dst>) -> Option<$dst> {
                    // if r.start < 0 (-> big nr), r.end >= 0, the check fails.
                    // if both < 0, the check fails too
                    // if 0 > start > end, then it passes.
                    if r.start < r.end && self >= (r.start as $src) && self <= (r.end as $src) {
                        Some(self as $dst)
                    } else {
                        None
                    }
                }
                #[inline(always)]
                fn cast_clamped(self, r: RangeInclusive<$dst>) -> $dst {
                    if self < r.start as $src {
                        r.start
                    } else if self > r.end as $src {
                        r.end
                    } else {
                        self as $dst
                    }
                }
            }
        )* )*
    )
}
#[test]
fn test_clip_checked() {
    assert_eq!((-3f32).cast_clipped(0u16...5), None);
    assert_eq!(8f32.cast_clipped(0u16...5), None);
    assert_eq!(3f32.cast_clipped(0u16...5), Some(3));
    assert_eq!(100f32.cast_clipped(0usize...1000), Some(100));
}

macro_rules! impl_cast_signed {
    ($( $unsigned:ty, $signed:ty; )*) => (
        $(
            impl Cast<$unsigned> for $signed {
                #[inline(always)]
                fn cast(self) -> Option<$unsigned> {
                    if self >= 0 {
                        Some(self as $unsigned)
                    } else {
                        None
                    }
                }
                #[inline(always)]
                fn cast_clipped(self, r: RangeInclusive<$unsigned>) -> Option<$unsigned> {
                    let start = r.start as $signed;
                    let u = self as $unsigned;
                    if start >= 0 && self >= start && u <= r.end {
                        Some(u)
                    } else {
                        None
                    }
                }
                #[inline(always)]
                fn cast_clamped(self, r: RangeInclusive<$unsigned>) -> $unsigned {                             
                    let start = r.start as $signed;
                    if start < 0 || self < start {
                        r.start
                    } else if self > r.end as $signed {
                        r.end
                    } else {
                        self as $unsigned
                    }
                }
            }
            impl Cast<$signed> for $unsigned {
                #[inline(always)]
                fn cast(self) -> Option<$signed> {
                    let s = self as $signed;
                    if s >= 0 {
                        Some(s)
                    } else {
                        None
                    }
                }
                #[inline(always)]
                fn cast_clipped(self, r: RangeInclusive<$signed>) -> Option<$signed> {
                    let s = self as $signed;
                    if s >= 0 && s >= r.start && s <= r.end {
                        Some(s)
                    } else {
                        None
                    }
                }
                #[inline(always)]
                fn cast_clamped(self, r: RangeInclusive<$signed>) -> $signed {
                    let s = self as $signed;
                    if s < 0 || s > r.end {
                        r.end
                    } else if s < r.start {
                        r.start
                    } else {
                        s
                    }
                }
            }
        )*
    )
}
macro_rules! impl_cast_id {
    ($( $a:ty, $b:ty; )*) => (
        $(
            impl Cast<$b> for $a {
                #[inline(always)]
                fn cast(self) -> Option<$b> {
                    Some(self as $b)
                }
                #[inline(always)]
                fn cast_clipped(self, r: RangeInclusive<$b>) -> Option<$b> {
                    let b = self as $b;
                    if b >= r.start && b <= r.end {
                        Some(b)
                    } else {
                        None
                    }
                }
                #[inline(always)]
                fn cast_clamped(self, r: RangeInclusive<$b>) -> $b {
                    let b = self as $b;
                    if b < r.start {
                        r.start
                    } else if b > r.end {
                        r.end
                    } else {
                        b
                    }
                }
            }
            impl Cast<$a> for $b {
                #[inline(always)]
                fn cast(self) -> Option<$a> {
                    Some(self as $a)
                }
                #[inline(always)]
                fn cast_clipped(self, r: RangeInclusive<$a>) -> Option<$a> {
                    let a = self as $a;
                    if a >= r.start && a <= r.end {
                        Some(a)
                    } else {
                        None
                    }
                }
                #[inline(always)]
                fn cast_clamped(self, r: RangeInclusive<$a>) -> $a {
                    let a = self as $a;
                    if a < r.start {
                        r.start
                    } else if a > r.end {
                        r.end
                    } else {
                        a
                    }
                }
            }
        )*
    )
}
impl_cast_unchecked!(
     u8 as [    u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64, usize, isize],
    u16 as [             u16, i32, u32, i64, u64, i128, u128, f32, f64, usize, isize],
    u32 as [                       u32,      u64, i128, u128, f32, f64              ],
    u64 as [                                 u64,       u128, f32, f64              ],
     i8 as [i8,     i16,      i32,      i64,      i128,       f32, f64,        isize],
    i16 as [        i16,      i32,      i64,      i128, u128, f32, f64,        isize],
    i32 as [                  i32,      i64,      i128, u128, f32, f64              ],
    i64 as [                            i64,      i128, u128, f32, f64              ],
    f32 as [                                                  f32, f64              ],
    f64 as [                                                       f64              ],
);
#[cfg(target_pointer_width = "32")]
impl_cast_unchecked!(
  usize as [                         u64, i128, u128, f32, f64],
  isize as [                    i64,      i128, u128, f32, f64],
);
#[cfg(target_pointer_width = "64")]
impl_cast_unchecked!(
    u32 as [                                                    usize],
  usize as [                                    u128, f32, f64],
  isize as [                              i128, u128, f32, f64],
);
impl_cast_checked!(
    u16 as [u8, i8                                   ],
    i16 as [u8, i8                                   ],
    u32 as [u8, i8, u16, i16                         ],
    i32 as [u8, i8, u16, i16                         ],
    u64 as [u8, i8, u16, i16, u32, i32               ],
    i64 as [u8, i8, u16, i16, u32, i32               ],
    f32 as [u8, i8, u16, i16, u32, i32, u64, i64     , usize, isize],
    f64 as [u8, i8, u16, i16, u32, i32, u64, i64, f32, usize, isize],
);
#[cfg(target_pointer_width = "32")]
impl_cast_checked!(
    u64 as [                                           usize],
  usize as [u8, i8, u16, i16                         ],
  isize as [u8, i8, u16, i16                         ],
);
#[cfg(target_pointer_width = "64")]
impl_cast_checked!(
  usize as [u8, i8, u16, i16, u32, i32               ],
  isize as [u8, i8, u16, i16, u32, i32               ],
);

impl_cast_signed!(u8, i8; u16, i16; u32, i32; u64, i64; usize, isize;);

#[cfg(target_pointer_width = "32")]
impl_cast_id!(usize, u32; isize, i32;);

#[cfg(target_pointer_width = "64")]
impl_cast_id!(usize, u64; isize, i64;);


use tuple::*;
macro_rules! impl_cast {
    ($($Tuple:ident { $($T:ident . $t:ident . $idx:tt),* } )*) => ($(
        #[allow(non_camel_case_types)]
        impl<$($T, $t),*> Cast<$Tuple<$($t),*>> for $Tuple<$($T),*>
        where $( $T: Cast<$t> ),*
        {
            #[inline(always)]
            fn cast(self) -> Option<$Tuple<$($t),*>> {
                match ( $(self.$idx.cast(), )* ) {
                    ( $( Some($t), )* ) => Some($Tuple($($t),*)),
                    _ => None
                }
            }
            #[inline(always)]
            fn cast_clipped(self, r: RangeInclusive<$Tuple<$($t),*>>) -> Option<$Tuple<$($t),*>> {
                match ( $(self.$idx.cast_clipped(r.start.$idx ... r.end.$idx), )* ) {
                    ( $( Some($t), )* ) => Some($Tuple($($t),*)),
                    _ => None
                }
            }
            #[inline(always)]
            fn cast_clamped(self, r: RangeInclusive<$Tuple<$($t),*>>) -> $Tuple<$($t),*> {
                $Tuple( $(self.$idx.cast_clamped(r.start.$idx ... r.end.$idx)),* )
            }
        }
    )*)
}

impl_tuple!(impl_cast);

