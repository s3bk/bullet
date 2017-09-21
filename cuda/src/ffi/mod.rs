#[allow(warnings)]
pub mod cuda;

#[link(name="cuda")]
extern {}

//#[allow(warnings)]
//pub mod cuda_runtime;
#[link(name="cudart", kind="dylib")]
extern {}

use std::ops;
use self::cuda::cudaError_enum;

impl ops::Try for cudaError_enum {
    type Ok = ();
    type Error = cudaError_enum;

    #[inline(always)]
    fn into_result(self) -> Result<(), cudaError_enum> {
        match self {
            cudaError_enum::CUDA_SUCCESS => Ok(()),
            e => Err(e)
        }
    }
    #[inline(always)]
    fn from_error(e: cudaError_enum) -> Self {
        e
    }
    #[inline(always)]
    fn from_ok(_: ()) -> Self {
        cudaError_enum::CUDA_SUCCESS
    }
}
