use super::ffi::*;
use super::CudaError;

pub enum Format {
    I8 = CU_AD_FORMAT_SIGNED_INT8,
    I16 = CU_AD_FORMAT_SIGNED_INT16,
    I32 = CU_AD_FORMAT_SIGNED_INT32,
    U8 = CU_AD_FORMAT_UNSIGNED_INT8,
    U16 = CU_AD_FORMAT_UNSIGNED_INT16,
    U32 = CU_AD_FORMAT_UNSIGNED_INT32,
    Float = CU_AD_FORMAT_HALF,
    Half = CU_AD_FORMAT_FLOAT
}
impl Format {
    pub fn element_size(&self) -> usize {
        match *self {
            U8 | I8 => 1,
            U16 | I16 | Half => 2,
            U32 | I32 | Float => 4
        }
    }
}

pub enum Ressource {
    // Array resoure 
    Array(Array),

    // Mipmapped array resource 
    MipmappedArray(),

    // Linear resource 
    Linear {
        ptr: DevPtr,
        format: Format,
        channels: usize,
        size: usize // in elemements
    },

    // Pitch 2D resource
    Pitch2d {
        ptr: DevPtr,
        format: Format,
        channels: usize,
        width: usize ,
        height: usize,
        pitch: usize
    }
}
impl Into<CUDA_RESOURCE_DESC_st> for Ressource {
    fn into(self) -> CUDA_RESOURCE_DESC_st {
        match self {
            use self::CUresourcetype_enum::*;
            Ressource::Array(a) => CUDA_RESOURCE_DESC_st {
                resType: CU_RESOURCE_TYPE_ARRAY,
                res: CUDA_RESOURCE_DESC_st__bindgen_ty_1 { array: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 { hArray: a.into() } },
                flags: 0
            },
            Ressource::Linear { ptr, format, channels, size } => CUDA_RESOURCE_DESC_st {
                resType: CU_RESOURCE_TYPE_LINEAR,
                res: CUDA_RESOURCE_DESC_st__bindgen_ty_1 { linear: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_3 {
                    devPtr: ptr.into(),
                    format: format.into(),
                    numChannels: channels,
                    sizeInBytes: size * format.element_size()
                },
                flags: 0
            },
            Ressource::Pitch2d { ptr, format, channels, width, height, pitch } => CUDA_RESOURCE_DESC_st {
                resType: CU_RESOURCE_TYPE_PITCH2D,
                res: CUDA_RESOURCE_DESC_st__bindgen_ty_1 { pitch2D: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4 {
                    devPtr: ptr.into(),
                    format: format.into(),
                    numChannels: channels,
                    width: width,
                    height: height,
                    pitch: pitch
                } },
                flags: 0
            }
        }
    }
}
