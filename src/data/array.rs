/// runtime Array 
struct Array<T> {
    data: *mut T,
    size: usize, // number of Ts
    shape: Vec<(usize, usize)> // (size, stride)
}
struct Slice<'a, T> {
    base: *mut T,
    stride: usize, // in elements
    start: usize, // base.offset(start) == first element
    end: usize // base.offset(end-1) == last element
}
impl<'a, T> Slice<'a, T> {
    /// get the element at offset i without bounds checking
    #[inline]
    unsafe fn idx(&self, i: usize) -> T {
        debug_assert!(pos >= self.start);
        debug_assert!(pos < self.end);
        debug_assert!(pos < std::isize::MAX);
        unsafe { *base.offset(i as isize) }
    }
}
impl<'a, T: Copy> Index<usize> for Slice<'a, T> {
    type Output = T;
    #[inline]
    fn index(&self, idx: usize) -> T {
        // indices outside of (start, end] are not valid
        assert!(idx >= self.start && idx < self.end, "out of bounds");
        
        // we are in bounds
        unsafe { self.idx(idx) }
    }
}
impl<'a, T: Copy> Iterator for Slice<'a, T> {
    type Item = T;
    #[inline]
    fn next(&mut self) {
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
    fn last(&self) -> Option<T> {
        let p = self.end - self.stride;
        if p >= self.start {
            Some(unsafe { self.idx(p) })
        } else {
            None
        }
    }
    #[inline]
    fn step_by(mut self, step: usize) -> Slice<'a, T> {
        self.stride *= step;
        self
    }
    #[inline]
    fn count(self) -> usize {
        (self.end - self.start) / self.stride
    }
    #[inline]
    fn for_each<F>(self, f: F) where F: FnMut(T) {
        while self.start < self.end {
            f(unsafe { self.idx(self.start) });
            self.start += self.stride;
        }
    }
}

impl<'a, T: Copy> DoubleEndedIterator for Slice<'a, T> {
    #[inline]
    fn next_back(&self) {
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
        idx.iter().zip(self.shape.iter()).map(|(i, (_, &stride))| i * stride).sum()
    }
    #[inline]
    unsafe fn idx(&self, pos: usize) -> T {
        debug_assert!(pos < self.size);
        debug_assert!(pos < std::isize::MAX);
        *self.base.offset(pos as isize)
    }
}
impl<'a, T> Index<&'a [usize]> for Array<T> {
    type Output = T;

    #[inline]
    fn index(&self, idx: &'a [usize]) -> &T {
        let off = self.offset(idx);
        assert!(off < self.size);
        unsafe { self.idx(off) }
    }
}
