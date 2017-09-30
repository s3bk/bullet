pub mod array;
pub use self::array::*;
use std::marker::PhantomData;

pub struct BorrowMut<'a>{_m: PhantomData<&'a ()>}
pub struct Borrow<'a>{_m: PhantomData<&'a ()>}

pub trait Ref<'a> {}
pub trait Mut<'a> {}
impl<'a> Ref<'a> for Borrow<'a> {}
impl<'a> Ref<'a> for BorrowMut<'a> {}
impl<'a> Mut<'a> for BorrowMut<'a> {}
