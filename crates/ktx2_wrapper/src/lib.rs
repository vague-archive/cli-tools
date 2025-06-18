#![allow(unsafe_code)]

use std::{fs::read, path::Path};

use autocxx::prelude::*;
use config::CompressionConfig;
use ktx_texture::KtxTexture;
use shared_types::{LocalError, SupportedImages};
use zune_core::{colorspace::ColorSpace, options::DecoderOptions};
use zune_jpeg::JpegDecoder;
use zune_png::PngDecoder;

pub mod config;
pub mod ktx_texture;
pub mod ktx_types;

include_cpp! {
    #include "ktx.h"
    #include "vk_format.h"
    #include "ktx_wrappers.h"
    safety!(unsafe)
    extern_cpp_type!("KtxBasisParams", crate::ffi2::KtxBasisParams)
    extern_cpp_type!("KtxPackUastcFlags", crate::ffi2::KtxPackUastcFlags)
    extern_cpp_type!("KtxBasisETC1SParams", crate::ffi2::KtxBasisETC1SParams)
    extern_cpp_type!("KtxBasisUASTCParams", crate::ffi2::KtxBasisUASTCParams)
    // What types and functions we want to generate
    generate!("ktxTexture2")
    generate_pod!("ktx_error_code_e")
    generate_pod!("ktx_pack_astc_block_dimension_e")
    generate_pod!("ktx_pack_astc_encoder_mode_e")
    generate_pod!("ktx_pack_astc_quality_levels_e")
    generate_pod!("ktxTextureCreateInfo")
    generate_pod!("VkFormat")
    generate_pod!("ktxAstcParams")
    generate!("ktxTextureCreateStorageEnum")
    generate!("ktxTexture2_CreateWrapped")
    generate!("ktxTexture2_CompressAstcExWrapped")
    generate!("ktxTexture2_CompressBasisUastc")
    generate!("ktxTexture2_CompressBasisEtc1s")
    generate!("ktxTexture2_DeflateZstd")
    generate!("ktxTexture2_DeflateZLIB")
    generate!("ktxTexture_SetImageFromMemoryWrapped")
    generate!("ktxTexture_WriteToNamedFileWrapped")
    generate!("ktxTexture_DestroyWrapped")
}

#[cxx::bridge]
pub mod ffi2 {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct KtxPackUastcFlags {
        value: u32,
    }

    #[derive(Debug)]
    pub struct KtxBasisETC1SParams {
        pub compression_level: *const u32,
        pub quality_level: *const u32,
        pub max_endpoints: *const u32,
        pub endpoint_rdo_threshold: *const f32,
        pub max_selectors: *const u32,
        pub selector_rdo_threshold: *const f32,
        pub input_swizzle: *const String,
        pub normal_map: *const bool,
        pub separate_rgt_to_rgba: *const bool,
        pub pre_swizzle: *const bool,
        pub no_endpoint_rdo: *const bool,
        pub no_selector_rdo: *const bool,
    }

    #[derive(Debug)]
    pub struct KtxBasisUASTCParams {
        pub uastc_flags: *const KtxPackUastcFlags,
        pub uastc_rdo: *const bool,
        pub uastc_rdo_quality_scalar: *const f32,
        pub uastc_rdo_dict_size: *const u32,
        pub uastc_rdo_max_smooth_block_error_scale: *const f32,
        pub uastc_rdo_max_smooth_block_std_dev: *const f32,
        pub uastc_rdo_dont_favor_simpler_modes: *const bool,
        pub uastc_rdo_no_multithreading: *const bool,
        pub input_swizzle: *const String,
        pub pre_swizzle: *const bool,
    }

    #[derive(Debug)]
    pub struct KtxBasisParams {
        pub struct_size: *const u32,
        pub uastc: bool,
        pub verbose: *const bool,
        pub no_sse: *const bool,
        pub thread_count: *const u32,
    }
}

fn premultiply_alpha(mut image_data: Vec<u8>) -> Result<Vec<u8>, LocalError> {
    if image_data.len() % 4 != 0 {
        return Err("Image does not appear to be rgba, cannot premultiply".into());
    }
    for pixel in image_data.chunks_mut(4) {
        pixel[0] = ((pixel[0] as u32 * pixel[3] as u32) / 255) as u8;
        pixel[1] = ((pixel[1] as u32 * pixel[3] as u32) / 255) as u8;
        pixel[2] = ((pixel[2] as u32 * pixel[3] as u32) / 255) as u8;
    }
    Ok(image_data)
}

fn extract_png_data(
    png_path: &Path,
    should_premultiply: bool,
) -> Result<(u32, u32, Vec<u8>), LocalError> {
    let png_file_contents = read(png_path)?;

    let decoder_options = DecoderOptions::default()
        .png_set_strip_to_8bit(true)
        .png_set_add_alpha_channel(true);
    let mut png_decoder =
        PngDecoder::new_with_options(png_file_contents.as_slice(), decoder_options);
    let png_data = png_decoder.decode_raw()?;
    let png_data = if should_premultiply {
        premultiply_alpha(png_data)?
    } else {
        png_data
    };
    let (width, height) = png_decoder.get_dimensions().ok_or(format!(
        "Cannot get dimensions for png: {}",
        png_path.display()
    ))?;
    Ok((width as u32, height as u32, png_data))
}

fn extract_jpeg_data(
    jpeg_path: &Path,
    should_premultiply: bool,
) -> Result<(u32, u32, Vec<u8>), LocalError> {
    let jpeg_file_contents = read(jpeg_path)?;

    let decoder_options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGBA);
    let mut jpeg_decoder =
        JpegDecoder::new_with_options(jpeg_file_contents.as_slice(), decoder_options);
    let jpeg_data = jpeg_decoder.decode()?;
    let jpeg_data = if should_premultiply {
        premultiply_alpha(jpeg_data)?
    } else {
        jpeg_data
    };
    let (width, height) = jpeg_decoder
        .dimensions()
        .ok_or(LocalError::from("JPEG decoding failed"))?;
    Ok((width as u32, height as u32, jpeg_data))
}

pub fn write_texture_from_image(
    image_input_path: &Path,
    image_write_path: Option<&Path>,
    config: &CompressionConfig,
    image_type: &SupportedImages,
) -> Result<(), LocalError> {
    let should_premultiply = config.premultiply.unwrap_or(true);
    let (width, height, mut image_data) = match image_type {
        SupportedImages::Png => extract_png_data(image_input_path, should_premultiply)?,
        SupportedImages::Jpeg => extract_jpeg_data(image_input_path, should_premultiply)?,
    };

    let ktx_texture = KtxTexture::new(width, height, ffi::VkFormat::VK_FORMAT_R8G8B8A8_UNORM)
        .map_err(|result| {
            LocalError::from(format!(
                "Error creating ktxTexture for {} when writing kxt, :{}",
                image_input_path.display(),
                String::from(result)
            ))
        })?;

    let mut ktx_texture = unsafe {
        ktx_texture
            .set_image_in_memory(image_data.as_mut_ptr())
            .map_err(|result| {
                LocalError::from(format!(
                    "Error setting image for {} from memory: {}",
                    image_input_path.display(),
                    String::from(result)
                ))
            })?
    };

    ktx_texture.set_compression(config).map_err(|result| {
        LocalError::from(format!(
            "Error settings compression type {:?}: {}",
            config.config_type(),
            String::from(result)
        ))
    })?;

    let mut canonicalized_path = if let Some(image_write_path) = image_write_path {
        image_write_path
            .parent()
            .unwrap()
            .canonicalize()?
            .join(image_write_path.file_name().unwrap())
    } else {
        image_input_path.canonicalize()?
    };
    if !canonicalized_path.set_extension("ktx") {
        return Err(format!(
            "Error changing image {} extension to ktx",
            image_input_path.display()
        )
        .into());
    }

    ktx_texture
        .write_image_to_disk(&canonicalized_path)
        .map_err(|result| {
            LocalError::from(format!(
                "Error writing image at {}: {}",
                image_input_path.display(),
                String::from(result)
            ))
        })?;

    Ok(())
}
