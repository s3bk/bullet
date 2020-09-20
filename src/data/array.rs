use super::*;
use std::ops::*;

/// runtime Array 
pub struct Array<T> {
    base: *mut T,
    size: usize, // number of Ts
    shape: Vec<(usize, usize)> // (size, stride)
}
pub struct Slice<T, B> {
    base: *mut T,
    stride: usize, // in elements
    start: usize, // base.offset(start) == first element
    end: usize, // base.offset(end-1) == last element,
    _b: B
}
impl<T: Copy, B> Slice<T, B> {
    /// get the element at offset i without bounds checking
    #[inline]
    pub unsafe fn idx(&self, pos: usize) -> T {
        debug_assert!(pos >= self.start);
        debug_assert!(pos < self.end);
        debug_assert!(pos < ::std::isize::MAX as usize);
        *self.base.offset(pos as isize)
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.base as *const T
    }
    #[inline]
    pub fn len(&self) -> usize {
        (self.end - self.start) / self.stride
    }   
}
impl<'a, T, B: Mut<'a>> Slice<T, B> {
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.base
    }
}
impl<T: Copy, B> Index<usize> for Slice<T, B> {
    type Output = T;
    #[inline]
    fn index(&self, idx: usize) -> &T {
        let pos = idx * self.stride;
        // indices outside of (start, end] are not valid
        assert!(pos >= self.start && pos < self.end, "out of bounds");
        
        // we are in bounds
        unsafe { &*self.base.offset(pos as isize) }
    }
}
impl<'a, T: Copy, B: Mut<'a>> IndexMut<usize> for Slice<T, B> {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut T {
        let pos = idx * self.stride;
        // indices outside of (start, end] are not valid
        assert!(pos >= self.start && pos < self.end, "out of bounds");
        
        // we are in bounds
        unsafe { &mut *self.base.offset(pos as isize) }
    }
}
impl<T: Copy, B> Iterator for Slice<T, B> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.start < self.end {
            let v = unsafe { self.idx(self.start) };
            self.start += self.stride;
            Some(v)
        } else {
            None
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = (self.end - self.start) / self.stride;
        (n, Some(n))
    }
    #[inline]
    fn last(self) -> Option<T> {
        let p = self.end - self.stride;
        if p >= self.start {
            Some(unsafe { self.idx(p) })
        } else {
            None
        }
    }
    #[inline]
    fn count(self) -> usize {
        (self.end - self.start) / self.stride
    }
    #[inline]
    fn for_each<F>(mut self, mut f: F) where F: FnMut(T) {
        while self.start < self.end {
            f(unsafe { self.idx(self.start) });
            self.start += self.stride;
        }
    }
}

impl<T: Copy, B> DoubleEndedIterator for Slice<T, B> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if self.start < self.end {
            let v = unsafe { *self.base.offset(self.end as isize) };
            self.end -= self.stride;
            Some(v)
        } else {
            None
        }
    }
}

impl<T> Array<T> {
    #[inline]
    fn offset(&self, idx: &[usize]) -> usize {
        idx.iter().zip(self.shape.iter()).map(|(i, &(_, stride))| i * stride).sum()
    }
}
impl<'a, T> Index<&'a [usize]> for Array<T> {
    type Output = T;

    #[inline]
    fn index(&self, idx: &'a [usize]) -> &T {
        let off = self.offset(idx);
        assert!(off < self.size);
        unsafe { &*self.base.offset(off as isize) }
    }
}
