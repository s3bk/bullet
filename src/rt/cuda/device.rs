use std::mem;
use super::ffi::cuda::*;
use super::*;

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
/*
    let test_prog = include_str!("../tests/main.ptx");
    let m = ctx.create_module(&mut String::from(test_prog)).expect("failed to create module");
    let mut data = vec![0f32; 1024];
    let f = m.get("main").expect("could not get kernel adress");
    unsafe {
        f.launch_simple(&mut data).expect("failed to launch kernel");
    }
    println!("{:?}", data);
*/
}

