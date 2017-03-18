use std::ops::{Add, Sub, Mul, Div};
use std::iter::Iterator;
use std::fmt;

/*
pub trait Elements {
    type E;
    const N: usize;
    
    fn elements(self) -> [Self::E; Self::N]
}
*/
macro_rules! tuple_init {
    ($($Tuple:ident { $($idx:tt -> $T:ident,)* } )*) => ($(
        pub struct $Tuple<$($T,)*>($(pub $T,)*);
        impl<$($T,)*> Clone for $Tuple<$($T,)*> where $( $T: Clone + 'static, )* {
            fn clone(&self) -> Self {
                $Tuple( $( self.$idx.clone(), )* )
            }
        }
        impl<$($T,)*> Copy for $Tuple<$($T,)*> where $( $T: Copy + 'static, )* {}
        impl<$($T,)*> fmt::Debug for $Tuple<$($T,)*> where $( $T: fmt::Debug + 'static, )* {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                ( $(&self.$idx, )* ).fmt(f)
            }
        }
        impl<$($T,)*> Add for $Tuple<$($T,)*> where $( $T: Add + 'static, )* {
            type Output = $Tuple<$($T::Output,)*>;
            fn add(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx + rhs.$idx, )* )
            }
        }
        impl<$($T,)*> Sub for $Tuple<$($T,)*> where $( $T: Sub + 'static, )* {
            type Output = $Tuple<$($T::Output,)*>;
            fn sub(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx - rhs.$idx, )* )
            }
        }
        impl<$($T,)*> Mul for $Tuple<$($T,)*> where $( $T: Mul + 'static, )* {
            type Output = $Tuple<$($T::Output,)*>;
            fn mul(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx * rhs.$idx, )* )
            }
        }
        impl<$($T,)*> Div for $Tuple<$($T,)*> where $( $T: Div + 'static, )* {
            type Output = $Tuple<$($T::Output,)*>;
            fn div(self, rhs: Self) -> Self::Output {
                $Tuple( $(self.$idx / rhs.$idx, )* )
            }
        }
        impl<$($T,)*> From<u16> for $Tuple<$($T,)*> where $( $T: From<u16> + 'static, )* {
            fn from(x: u16) -> Self {
                $Tuple( $( $T::from(x), )* )
            }
        }
        impl<$($T,)*> Iterator for $Tuple<$($T,)*> where $( $T: Iterator + 'static, )* {
            type Item = $Tuple<$($T::Item,)*>;
            #[allow(non_snake_case)] 
            fn next(&mut self) -> Option<Self::Item> {
                match ( $(self.$idx.next(), )* ) {
                    ( $( Some($T), )* ) => Some($Tuple( $($T,)* )),
                    _ => None
                }
            }
        }
    )*)
}

#[macro_export]
macro_rules! impl_tuple {
    ($def:ident) => ($def!(
    T1 { 0 -> A, }
    T2 { 0 -> A, 1 -> B, }
    T3 { 0 -> A, 1 -> B, 2 -> C, }
    T4 { 0 -> A, 1 -> B, 2 -> C, 3 -> D, }
    T5 { 0 -> A, 1 -> B, 2 -> C, 3 -> D, 4 -> E, }
    T6 { 0 -> A, 1 -> B, 2 -> C, 3 -> D, 4 -> E, 5 -> F, }
    T7 { 0 -> A, 1 -> B, 2 -> C, 3 -> D, 4 -> E, 5 -> F, 6 -> G, }
    T8 { 0 -> A, 1 -> B, 2 -> C, 3 -> D, 4 -> E, 5 -> F, 6 -> G, 7 -> H, }
    );)
}

impl_tuple!(tuple_init);
