use std::{
    borrow::BorrowMut,
    fmt::Display,
    marker::PhantomData,
    mem::{replace, take},
    path::Path,
    ptr::null_mut,
};

use num_traits::Bounded;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    config::{CompressionConfig, KTXCompressionConfig},
    ffi::{
        self, ktxTexture2, ktxTexture2_CreateWrapped, ktxTextureCreateInfo,
        ktxTextureCreateStorageEnum, ktxTexture_DestroyWrapped,
        ktxTexture_SetImageFromMemoryWrapped, ktxTexture_WriteToNamedFileWrapped, ktx_error_code_e,
        VkFormat,
    },
};

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZLibDeflationValue(u8);

impl Display for ZLibDeflationValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Deserialize<'a> for ZLibDeflationValue {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let u8_value = u8::deserialize(deserializer)?;

        Ok(u8_value.into())
    }
}

impl Default for ZLibDeflationValue {
    fn default() -> Self {
        Self(7)
    }
}

impl Bounded for ZLibDeflationValue {
    fn min_value() -> Self {
        Self(1)
    }

    fn max_value() -> Self {
        Self(9)
    }
}

impl From<u8> for ZLibDeflationValue {
    fn from(value: u8) -> Self {
        Self(value).clamp(Self::min_value(), Self::max_value())
    }
}

impl From<ZLibDeflationValue> for u32 {
    fn from(value: ZLibDeflationValue) -> Self {
        value.0 as u32
    }
}
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZstdDeflationValue(u8);

impl Display for ZstdDeflationValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Deserialize<'a> for ZstdDeflationValue {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let u8_value = u8::deserialize(deserializer)?;

        Ok(u8_value.into())
    }
}

impl Default for ZstdDeflationValue {
    fn default() -> Self {
        Self(17)
    }
}

impl Bounded for ZstdDeflationValue {
    fn min_value() -> Self {
        Self(1)
    }

    fn max_value() -> Self {
        Self(22)
    }
}

impl From<u8> for ZstdDeflationValue {
    fn from(value: u8) -> Self {
        Self(value).clamp(Self::min_value(), Self::max_value())
    }
}

impl From<ZstdDeflationValue> for u32 {
    fn from(value: ZstdDeflationValue) -> Self {
        value.0 as u32
    }
}

pub struct Uninitialized;
pub struct BeforeImageSetInMemory;
pub struct ImageSetInMemory;

pub struct KtxTexture<Stage> {
    ktx_texture_2_ptr: *mut ktxTexture2,
    texture_create_info: ktxTextureCreateInfo,
    phantom_marker: PhantomData<Stage>,
}

impl KtxTexture<Uninitialized> {
    pub fn new(
        width: u32,
        height: u32,
        format: VkFormat,
    ) -> Result<KtxTexture<BeforeImageSetInMemory>, ktx_error_code_e> {
        let mut result = ktx_error_code_e::KTX_FILE_DATA_ERROR;

        let mut texture_create_info = ktxTextureCreateInfo::new(width, height, format);

        let ktx_texture_2_ptr = unsafe {
            ktxTexture2_CreateWrapped(
                texture_create_info.borrow_mut(),
                ktxTextureCreateStorageEnum::KTX_TEXTURE_CREATE_ALLOC_STORAGE,
                &mut result,
            )
        };

        if result == ktx_error_code_e::KTX_SUCCESS {
            Ok(KtxTexture {
                ktx_texture_2_ptr,
                texture_create_info,
                phantom_marker: PhantomData,
            })
        } else {
            Err(result)
        }
    }
}

impl KtxTexture<BeforeImageSetInMemory> {
    /// # Safety
    /// `image_data` is a raw pointer being passed into an outside library, and could have undefined behavior
    pub unsafe fn set_image_in_memory(
        mut self,
        image_data: *mut u8,
    ) -> Result<KtxTexture<ImageSetInMemory>, ktx_error_code_e> {
        let result = unsafe {
            ktxTexture_SetImageFromMemoryWrapped(
                self.ktx_texture_2_ptr,
                0,
                0,
                0,
                image_data,
                self.texture_create_info.image_size(),
            )
        };
        if result == ktx_error_code_e::KTX_SUCCESS {
            Ok(KtxTexture {
                ktx_texture_2_ptr: replace(&mut self.ktx_texture_2_ptr, null_mut()),
                texture_create_info: take(&mut self.texture_create_info),
                phantom_marker: PhantomData,
            })
        } else {
            Err(result)
        }
    }
}

impl KtxTexture<ImageSetInMemory> {
    pub fn set_compression(
        &mut self,
        compression_config: &CompressionConfig,
    ) -> Result<(), ktx_error_code_e> {
        match compression_config.config() {
            KTXCompressionConfig::BasisUniversalBasisLZETC1s(basis_universal_basis_lzetc1s) => {
                let (base_config, etc1s_config) = basis_universal_basis_lzetc1s.into();

                let result = unsafe {
                    ffi::ktxTexture2_CompressBasisEtc1s(
                        self.ktx_texture_2_ptr,
                        &base_config,
                        &etc1s_config,
                    )
                };

                if result != ktx_error_code_e::KTX_SUCCESS {
                    return Err(result);
                }
            }
            KTXCompressionConfig::BasisUniversalUASTC(basis_universal_uastc) => {
                let (base_config, uastc_config) = basis_universal_uastc.into();

                let result = unsafe {
                    ffi::ktxTexture2_CompressBasisUastc(
                        self.ktx_texture_2_ptr,
                        &base_config,
                        &uastc_config,
                    )
                };

                if result != ktx_error_code_e::KTX_SUCCESS {
                    return Err(result);
                }
            }
            KTXCompressionConfig::ASTC(astc) => {
                let config = astc.into();

                let result = unsafe {
                    ffi::ktxTexture2_CompressAstcExWrapped(self.ktx_texture_2_ptr, config)
                };

                if result != ktx_error_code_e::KTX_SUCCESS {
                    return Err(result);
                }
            }
            KTXCompressionConfig::ZLib(zlib) => {
                let result = unsafe {
                    ffi::ktxTexture2_DeflateZLIB(
                        self.ktx_texture_2_ptr,
                        zlib.deflation_value.into(),
                    )
                };

                if result != ktx_error_code_e::KTX_SUCCESS {
                    return Err(result);
                }
            }
            KTXCompressionConfig::Zstd(zstd) => {
                let result = unsafe {
                    ffi::ktxTexture2_DeflateZstd(
                        self.ktx_texture_2_ptr,
                        zstd.deflation_value.into(),
                    )
                };

                if result != ktx_error_code_e::KTX_SUCCESS {
                    return Err(result);
                }
            }
        }
        Ok(())
    }
    pub fn write_image_to_disk(&self, path: &Path) -> Result<(), ktx_error_code_e> {
        let path = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
        let result =
            unsafe { ktxTexture_WriteToNamedFileWrapped(self.ktx_texture_2_ptr, path.as_ptr()) };
        if result == ktx_error_code_e::KTX_SUCCESS {
            Ok(())
        } else {
            Err(result)
        }
    }
}

// Normally, we would only specify this trait for the ImageSetInMemory generic, however Drop is a special
// trait that cannot be specialized. Further, run time type inspection/specialization isn't close to stable,
// So instead we use some logic to deduce which stage the KtxTexture is at (the only time the ptr should not be
// null is on the ImageSetInMemory stage/generic)
impl<Stage> Drop for KtxTexture<Stage> {
    fn drop(&mut self) {
        if !self.ktx_texture_2_ptr.is_null() {
            unsafe {
                ktxTexture_DestroyWrapped(self.ktx_texture_2_ptr);
            }
        }
    }
}
