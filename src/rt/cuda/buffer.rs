use std::ptr::Unique;
use std::ops::{Deref, DerefMut};
use super::*;

pub struct Buffer<T: Copy> {
    ptr: Unique<T>,
    len: usize,
    cap: usize
}
impl<T: Copy> Buffer<T> {
    #[inline]
    pub fn with_capacity(count: usize) -> Result<Buffer<T>, CudaError> {
        let mut ptr = ptr::null_mut();
        unsafe {
            cuMemHostAlloc(&mut ptr as *mut _ as *mut *mut c_void, count * mem::size_of::<T>(), CU_MEMHOSTALLOC_DEVICEMAP)?;
        }
        Ok(Buffer {
            ptr: Unique::new(ptr).unwrap(),
            len: 0,
            cap: count
        })
    }
    #[inline]
    pub fn push(&mut self, t: T) {
        assert!(self.len < self.cap);
        unsafe {
            ptr::write(self.ptr.as_ptr().offset(self.len as isize), t);
        }
        self.len += 1;
    }
    #[inline]
    pub fn dev_ptr(&self) -> Result<u64, CudaError> {
        let mut d_ptr = 0u64;
        unsafe {
            cuMemHostGetDevicePointer_v2(&mut d_ptr, self.ptr.as_ptr() as *mut c_void, 0)?;
        }
        Ok(d_ptr)
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
    #[inline]
    pub unsafe fn set_len(&mut self, len: usize) {
        assert!(len <= self.cap);
        self.len = len;
    }
}
impl<T: Copy> Deref for Buffer<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}
impl<T: Copy> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        }
    }
}
impl<T: Copy> Drop for Buffer<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            cuMemFreeHost(self.ptr.as_ptr() as *mut c_void);
        }
    }
}
