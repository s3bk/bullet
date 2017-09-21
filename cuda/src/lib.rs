#![feature(unique, try_trait)]

pub mod ffi;
use self::ffi::cuda::*;
use std::{mem, ptr, slice};
use std::os::raw::c_void;
use std::ops::{Deref, DerefMut};
use std::ptr::Unique;

#[derive(Debug)]
pub enum CudaError {
    Other(cudaError_enum),
    Prohibited
}
impl From<cudaError_enum> for CudaError {
    fn from(v: cudaError_enum) -> CudaError {
        CudaError::Other(v)
    }
}



pub struct Device {
    handle: CUdevice
}
impl Device {
    pub fn get(num: u32) -> Result<Device, CudaError> {        
        unsafe {
            let mut handle = mem::zeroed();
            cuInit(0)?;
            cuDeviceGet(&mut handle, num as i32)?;
        
            Ok(Device {
                handle: handle
            })
        }
    }
    pub fn test(&self) -> Result<(), CudaError> {
        if self.compute_mode()? == CUcomputemode_enum::CU_COMPUTEMODE_PROHIBITED as i32 {
            return Err(CudaError::Prohibited);
        }
        Ok(())
    }
    #[inline]
    fn get_attr(&self, attr: CUdevice_attribute_enum) -> Result<i32, CudaError> {
        let mut val = 0;
        unsafe {
            cuDeviceGetAttribute(&mut val, attr, self.handle)?;
        }
        Ok(val)
    }
    
    pub fn get_name(&self) -> Result<String, CudaError> {
        let mut buf = vec![0u8; 256];
        unsafe {
            cuDeviceGetName(buf.as_mut_ptr() as *mut i8, buf.len() as i32, self.handle)?;
            let len = buf.iter().position(|&b| b == 0).unwrap();
            buf.truncate(len);
        }
        
        Ok(String::from_utf8(buf).expect("driver returned non-ascii"))
    }
    pub fn total_memory(&self) -> Result<usize, CudaError> {
        let mut size = 0;
        unsafe {
            cuDeviceTotalMem_v2(&mut size, self.handle)?;
        }
        Ok(size)
    }
    pub fn create_context(&self) -> Result<Context, CudaError> {
        unsafe {
            let mut handle = mem::zeroed();
            cuCtxCreate_v2(&mut handle, 2, self.handle)?;
            Ok(Context { handle: handle })
        }
    }
}

macro_rules! impl_device_attr {
    ($($(#[$meta:meta])* $fn:ident: $attr:ident,)*) => (
        impl Device { $(
            $( #[$meta] )*
            pub fn $fn(&self) -> Result<i32, CudaError> {
                self.get_attr(CUdevice_attribute_enum::$attr)
            }
        )* }
    )
}
impl_device_attr!(
    constant_memory: CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY,
    /// clock rate in kHz
    clock_rate: CU_DEVICE_ATTRIBUTE_CLOCK_RATE,
    compute_mode: CU_DEVICE_ATTRIBUTE_COMPUTE_MODE,
);

struct ZeroString<'a> {
    string: &'a mut String
}
impl<'a> ZeroString<'a> {
    pub fn new(s: &'a mut String) -> ZeroString<'a> {
        if s.capacity() <= s.len() + 1 {
            s.reserve(1);
        }
        s.push('\0');
        ZeroString { string: s }
    }
    pub fn ptr(&self) -> *const i8 {
        self.string.as_ptr() as *const i8
    }
}
impl<'a> Drop for ZeroString<'a> {
    fn drop(&mut self) {
        self.string.pop();
    }
}

pub struct Context {
    handle: CUcontext
}
impl Context {
    pub fn create_module(&self, data: &mut String) -> Result<Module, CudaError> {
        unsafe {
            let mut module = mem::zeroed();
            
            let s = ZeroString::new(data);
            cuModuleLoadData(&mut module, s.ptr() as *const c_void)?;
            
            Ok(Module {
                module,
                context: self
            })
        }
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            cuCtxDestroy_v2(self.handle);
        }
    }
}

pub struct Module<'a> {
    module: CUmodule,
    context: &'a Context
}
impl<'a> Module<'a> {
    pub fn get(&self, name: &str) -> Result<Function, CudaError> {
        let mut name = String::from(name);
        
        let kernel = unsafe {
            let s = ZeroString::new(&mut name);
            let mut kernel = mem::zeroed();
            cuModuleGetFunction(&mut kernel, self.module, s.ptr())?;
            kernel
        };

        Ok(Function {
            func: kernel,
            name: name,
            module: self
        })
    }
}

pub struct Function<'a> {
    func: CUfunction,
    name: String,
    module: &'a Module<'a>
}
impl<'a> Function<'a> {
    /// this copies the given data into GPU memory
    /// and executes the kernel.
    /// The number and types of the parameters have to match those of the the function!
    #[inline]
    pub unsafe fn launch(&self, grid: [u32; 3], block: [u32; 3], shared_mem: u32, args: &mut [*mut c_void]) -> Result<(), CudaError>
    {
        println!("grid: {:?}, block: {:?}", grid, block);
        cuLaunchKernel(
            self.func,
            grid[0], grid[1], grid[2],
            block[0], block[1], block[2],
            shared_mem,
            ptr::null_mut(), // stream
            args.as_mut_ptr(),
            ptr::null_mut(), // parameters
        )?;
        Ok(())
    }
    #[inline]
    pub unsafe fn launch_simple<T: Copy>(&self, data_in: &Buffer<T>, data_out: &mut Buffer<T>) -> Result<(), CudaError> {
        let batch = 512;
        let mut src = data_in.dev_ptr()?;
        let mut dst = data_out.dev_ptr()?;
        let mut args = [
            &mut src as *mut u64 as *mut c_void,
            &mut dst as *mut u64 as *mut c_void
        ];
        self.launch(
            [(data_in.len() / batch as usize) as u32, 1, 1],
            [batch, 1, 1],
            0,
            &mut args
        )?;
        cuCtxSynchronize()?;
        data_out.set_len(data_in.len());
        Ok(())
    }
}

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
    fn dev_ptr(&self) -> Result<u64, CudaError> {
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
#[test]
fn test_device() {
    let dev = Device::get(0).expect("failed to init");
    println!("name: {}", dev.get_name().unwrap());
    println!("clock: {} kHz", dev.clock_rate().unwrap());
    println!("memory: {} kB", dev.total_memory().unwrap() / 1024);

    dev.test().expect("self test failed");
    println!("self test succeeded");
    let ctx = dev.create_context().unwrap();
    println!("got a context");

    let test_prog = include_str!("../tests/main.ptx");
    let m = ctx.create_module(&mut String::from(test_prog)).expect("failed to create module");
    let mut data = vec![0f32; 1024];
    let f = m.get("main").expect("could not get kernel adress");
    unsafe {
        f.launch_simple(&mut data).expect("failed to launch kernel");
    }
    println!("{:?}", data);
}

