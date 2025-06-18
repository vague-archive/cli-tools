use autocxx::WithinUniquePtr;
use cxx::UniquePtr;
use lazy_regex::{lazy_regex, Lazy, Regex};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use strum::{Display, EnumDiscriminants};

use crate::{
    ffi::{
        ktxAstcParams, ktx_pack_astc_block_dimension_e, ktx_pack_astc_encoder_mode_e,
        ktx_pack_astc_quality_levels_e, KtxBasisETC1SParams, KtxBasisParams, KtxBasisUASTCParams,
    },
    ffi2::KtxPackUastcFlags,
    ktx_texture::{ZLibDeflationValue, ZstdDeflationValue},
};

static INPUT_SWIZZLE_REGEX: Lazy<Regex> = lazy_regex!("/^[rgba01]{4}$/");

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub config_type: KTXCompressionConfigTypes,
    pub config: KTXCompressionConfig,
    #[serde(default = "default_premultiply")]
    pub premultiply: Option<bool>,
}

fn default_premultiply() -> Option<bool> {
    Some(true)
}

impl CompressionConfig {
    pub fn config(&self) -> &KTXCompressionConfig {
        &self.config
    }
    pub fn config_type(&self) -> &KTXCompressionConfigTypes {
        &self.config_type
    }
}

#[derive(Debug, EnumDiscriminants, Serialize, Deserialize)]
#[strum_discriminants(
    derive(Serialize, Deserialize, Display),
    name(KTXCompressionConfigTypes),
    repr(u8)
)]
pub enum KTXCompressionConfig {
    BasisUniversalBasisLZETC1s(BasisUniversalBasisLZETC1s),
    BasisUniversalUASTC(BasisUniversalUASTC),
    ASTC(ASTC),
    ZLib(ZLib),
    Zstd(Zstd),
}

impl Default for KTXCompressionConfigTypes {
    fn default() -> Self {
        Self::BasisUniversalBasisLZETC1s
    }
}

impl Default for KTXCompressionConfig {
    fn default() -> Self {
        Self::BasisUniversalBasisLZETC1s(BasisUniversalBasisLZETC1s::default())
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BasisUniversalBasisLZETC1s {
    pub verbose: Option<bool>,
    pub no_sse: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_thread_count")]
    pub thread_count: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_compression_level")]
    pub compression_level: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_basis_quality_level")]
    pub quality_level: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_max_endpoints")]
    pub max_endpoints: Option<u32>,
    pub endpoint_rdo_threshold: Option<f32>,
    #[serde(default, deserialize_with = "deserialize_max_selectors")]
    pub max_selectors: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_input_swizzle")]
    pub input_swizzle: Option<String>,
    pub normal_map: Option<bool>,
    pub separate_rgt_to_rgba: Option<bool>,
    pub pre_swizzle: Option<bool>,
    pub no_endpoint_rdo: Option<bool>,
    pub no_selector_rdo: Option<bool>,
}

impl From<&BasisUniversalBasisLZETC1s> for (KtxBasisParams, KtxBasisETC1SParams) {
    fn from(value: &BasisUniversalBasisLZETC1s) -> Self {
        let mut ktx_basis_params = KtxBasisParams::default();
        let mut ktx_etc1s_params = KtxBasisETC1SParams::default();

        ktx_basis_params.uastc = false;
        if let Some(verbose) = value.verbose {
            let boxed_verbose = Box::new(verbose);
            ktx_basis_params.verbose = Box::into_raw(boxed_verbose);
        }
        if let Some(no_sse) = value.no_sse {
            let boxed_no_sse = Box::new(no_sse);
            ktx_basis_params.no_sse = Box::into_raw(boxed_no_sse);
        }
        if let Some(thread_count) = value.thread_count {
            let boxed_thread_count = Box::new(thread_count);
            ktx_basis_params.thread_count = Box::into_raw(boxed_thread_count);
        }
        if let Some(compression_level) = value.compression_level {
            let boxed_compression_level = Box::new(compression_level);
            ktx_etc1s_params.compression_level = Box::into_raw(boxed_compression_level);
        }
        if let Some(quality_level) = value.quality_level {
            let boxed_quality_level = Box::new(quality_level);
            ktx_etc1s_params.quality_level = Box::into_raw(boxed_quality_level);
        }
        if let Some(max_endpoints) = value.max_endpoints {
            let boxed_max_endpoints = Box::new(max_endpoints);
            ktx_etc1s_params.max_endpoints = Box::into_raw(boxed_max_endpoints);
        }
        if let Some(endpoint_rdo_threshold) = value.endpoint_rdo_threshold {
            let boxed_endpoint_rdo_threshold = Box::new(endpoint_rdo_threshold);
            ktx_etc1s_params.endpoint_rdo_threshold = Box::into_raw(boxed_endpoint_rdo_threshold);
        }
        if let Some(max_selectors) = value.max_selectors {
            let boxed_max_selectors = Box::new(max_selectors);
            ktx_etc1s_params.max_selectors = Box::into_raw(boxed_max_selectors);
        }
        if let Some(input_swizzle) = &value.input_swizzle {
            let boxed_input_swizzle = Box::new((*input_swizzle).clone());
            ktx_etc1s_params.input_swizzle = Box::into_raw(boxed_input_swizzle);
        }
        if let Some(normal_map) = value.normal_map {
            let boxed_normal_map = Box::new(normal_map);
            ktx_etc1s_params.normal_map = Box::into_raw(boxed_normal_map);
        }
        if let Some(separate_rgt_to_rgba) = value.separate_rgt_to_rgba {
            let boxed_separate_rgt_to_rgba = Box::new(separate_rgt_to_rgba);
            ktx_etc1s_params.separate_rgt_to_rgba = Box::into_raw(boxed_separate_rgt_to_rgba);
        }
        if let Some(pre_swizzle) = value.pre_swizzle {
            let boxed_pre_swizzle = Box::new(pre_swizzle);
            ktx_etc1s_params.pre_swizzle = Box::into_raw(boxed_pre_swizzle);
        }
        if let Some(no_endpoint_rdo) = value.no_endpoint_rdo {
            let boxed_no_endpoint_rdo = Box::new(no_endpoint_rdo);
            ktx_etc1s_params.no_endpoint_rdo = Box::into_raw(boxed_no_endpoint_rdo);
        }
        if let Some(no_selector_rdo) = value.no_selector_rdo {
            let boxed_no_selector_rdo = Box::new(no_selector_rdo);
            ktx_etc1s_params.no_selector_rdo = Box::into_raw(boxed_no_selector_rdo);
        }
        (ktx_basis_params, ktx_etc1s_params)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BasisUniversalUASTC {
    pub verbose: Option<bool>,
    pub no_sse: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_thread_count")]
    pub thread_count: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_input_swizzle")]
    pub input_swizzle: Option<String>,
    pub pre_swizzle: Option<bool>,
    #[serde(
        default,
        serialize_with = "serialize_uastc_flags",
        deserialize_with = "deserialize_uastc_flags"
    )]
    pub uastc_flags: Option<KtxPackUastcFlags>,
    pub uastc_rdo: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_uastc_rdo_quality_scalar")]
    pub uastc_rdo_quality_scalar: Option<f32>,
    #[serde(default, deserialize_with = "deserialize_uastc_rdo_dict_size")]
    pub uastc_rdo_dict_size: Option<u32>,
    #[serde(
        default,
        deserialize_with = "deserialize_uastc_rdo_max_smooth_block_error_scale"
    )]
    pub uastc_rdo_max_smooth_block_error_scale: Option<f32>,
    #[serde(
        default,
        deserialize_with = "deserialize_uastc_rdo_max_smooth_block_std_dev"
    )]
    pub uastc_rdo_max_smooth_block_std_dev: Option<f32>,
    pub uastc_rdo_dont_favor_simpler_modes: Option<bool>,
    pub uastc_rdo_no_multithreading: Option<bool>,
}

impl From<&BasisUniversalUASTC> for (KtxBasisParams, KtxBasisUASTCParams) {
    fn from(value: &BasisUniversalUASTC) -> Self {
        let mut ktx_basis_params = KtxBasisParams::default();
        let mut ktx_uastc_params = KtxBasisUASTCParams::default();

        ktx_basis_params.uastc = true;

        if let Some(verbose) = value.verbose {
            let boxed_verbose = Box::new(verbose);
            ktx_basis_params.verbose = Box::into_raw(boxed_verbose);
        }
        if let Some(no_sse) = value.no_sse {
            let boxed_no_sse = Box::new(no_sse);
            ktx_basis_params.no_sse = Box::into_raw(boxed_no_sse);
        }
        if let Some(thread_count) = value.thread_count {
            let boxed_thread_count = Box::new(thread_count);
            ktx_basis_params.thread_count = Box::into_raw(boxed_thread_count);
        }
        if let Some(input_swizzle) = &value.input_swizzle {
            let boxed_input_swizzle = Box::new((*input_swizzle).clone());
            ktx_uastc_params.input_swizzle = Box::into_raw(boxed_input_swizzle);
        }
        if let Some(pre_swizzle) = value.pre_swizzle {
            let boxed_pre_swizzle = Box::new(pre_swizzle);
            ktx_uastc_params.pre_swizzle = Box::into_raw(boxed_pre_swizzle);
        }
        if let Some(uastc_flags) = value.uastc_flags {
            let boxed_uastc_flags = Box::new(uastc_flags);
            ktx_uastc_params.uastc_flags = Box::into_raw(boxed_uastc_flags);
        }
        if let Some(uastc_rdo) = value.uastc_rdo {
            let boxed_uastc_rdo = Box::new(uastc_rdo);
            ktx_uastc_params.uastc_rdo = Box::into_raw(boxed_uastc_rdo);
        }
        if let Some(uastc_rdo_quality_scalar) = value.uastc_rdo_quality_scalar {
            let boxed_uastc_rdo_quality_scalar = Box::new(uastc_rdo_quality_scalar);
            ktx_uastc_params.uastc_rdo_quality_scalar =
                Box::into_raw(boxed_uastc_rdo_quality_scalar);
        }
        if let Some(uastc_rdo_dict_size) = value.uastc_rdo_dict_size {
            let boxed_uastc_rdo_dict_size = Box::new(uastc_rdo_dict_size);
            ktx_uastc_params.uastc_rdo_dict_size = Box::into_raw(boxed_uastc_rdo_dict_size);
        }
        if let Some(uastc_rdo_max_smooth_block_error_scale) =
            value.uastc_rdo_max_smooth_block_error_scale
        {
            let boxed_uastc_rdo_max_smooth_block_error_scale =
                Box::new(uastc_rdo_max_smooth_block_error_scale);
            ktx_uastc_params.uastc_rdo_max_smooth_block_error_scale =
                Box::into_raw(boxed_uastc_rdo_max_smooth_block_error_scale);
        }
        if let Some(uastc_rdo_max_smooth_block_std_dev) = value.uastc_rdo_max_smooth_block_std_dev {
            let boxed_uastc_rdo_max_smooth_block_std_dev =
                Box::new(uastc_rdo_max_smooth_block_std_dev);
            ktx_uastc_params.uastc_rdo_max_smooth_block_std_dev =
                Box::into_raw(boxed_uastc_rdo_max_smooth_block_std_dev);
        }
        if let Some(uastc_rdo_dont_favor_simpler_modes) = value.uastc_rdo_dont_favor_simpler_modes {
            let boxed_uastc_rdo_dont_favor_simpler_modes =
                Box::new(uastc_rdo_dont_favor_simpler_modes);
            ktx_uastc_params.uastc_rdo_dont_favor_simpler_modes =
                Box::into_raw(boxed_uastc_rdo_dont_favor_simpler_modes);
        }
        if let Some(uastc_rdo_no_multithreading) = value.uastc_rdo_no_multithreading {
            let boxed_uastc_rdo_no_multithreading = Box::new(uastc_rdo_no_multithreading);
            ktx_uastc_params.uastc_rdo_no_multithreading =
                Box::into_raw(boxed_uastc_rdo_no_multithreading);
        }
        (ktx_basis_params, ktx_uastc_params)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ASTC {
    pub verbose: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_thread_count")]
    pub thread_count: Option<u32>,
    #[serde(
        default,
        serialize_with = "serialize_block_dimension",
        deserialize_with = "deserialize_block_dimension"
    )]
    pub block_dimension: Option<ktx_pack_astc_block_dimension_e>,
    #[serde(
        default,
        serialize_with = "serialize_mode",
        deserialize_with = "deserialize_mode"
    )]
    pub mode: Option<ktx_pack_astc_encoder_mode_e>,
    #[serde(
        default,
        serialize_with = "serialize_astc_quality_level",
        deserialize_with = "deserialize_astc_quality_level"
    )]
    pub quality_level: Option<ktx_pack_astc_quality_levels_e>,
    pub normal_map: Option<bool>,
    pub perceptual: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_input_swizzle")]
    pub input_swizzle: Option<String>,
}

impl From<&ASTC> for UniquePtr<ktxAstcParams> {
    fn from(value: &ASTC) -> Self {
        let mut ktx_astc_params = ktxAstcParams::new().within_unique_ptr();
        if let Some(verbose) = value.verbose {
            ktx_astc_params.verbose = verbose;
        }
        if let Some(thread_count) = value.thread_count {
            ktx_astc_params.threadCount = thread_count;
        }
        if let Some(block_dimension) = value.block_dimension {
            ktx_astc_params.blockDimension = block_dimension as u32;
        }
        if let Some(mode) = value.mode {
            ktx_astc_params.mode = mode as u32;
        }
        if let Some(quality_level) = value.quality_level {
            ktx_astc_params.qualityLevel = quality_level as u32;
        }
        ktx_astc_params
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ZLib {
    pub deflation_value: ZLibDeflationValue,
}

macro_rules! create_deserialize_number_range {
    ($(($function_name:ident, $number_type:ty, $low_value:literal, $high_value:literal)), *) => {
        $(
            fn $function_name<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Option<$number_type>, D::Error> {
                let number_value = Option::<$number_type>::deserialize(deserializer)?;
                Ok(number_value.map(|number_value| number_value.clamp($low_value, $high_value)))
            }

        )*

    };
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Zstd {
    pub deflation_value: ZstdDeflationValue,
}

create_deserialize_number_range!(
    (deserialize_thread_count, u32, 1, 16),
    (deserialize_compression_level, u32, 0, 5),
    (deserialize_basis_quality_level, u32, 1, 255),
    (deserialize_max_endpoints, u32, 1, 16128),
    (deserialize_max_selectors, u32, 1, 16128),
    (deserialize_uastc_rdo_quality_scalar, f32, 0.001, 50.),
    (deserialize_uastc_rdo_dict_size, u32, 64, 65536),
    (
        deserialize_uastc_rdo_max_smooth_block_error_scale,
        f32,
        1.,
        300.
    ),
    (
        deserialize_uastc_rdo_max_smooth_block_std_dev,
        f32,
        0.01,
        65536.
    )
);

macro_rules! create_serialize_deserialize_enum {
    ($(($serialize_function_name:ident, $deserialize_function_name:ident, $enum_type:ty)), *) => {
        $(
            fn $serialize_function_name<S: Serializer>(value: &Option<$enum_type>, serializer: S) -> Result<S::Ok, S::Error> {
                if let Some(value) = value {
                    serializer.serialize_str(&value.to_string())
                } else {
                    serializer.serialize_none()
                }
            }

            fn $deserialize_function_name<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Option<$enum_type>, D::Error> {
                let string_value = String::deserialize(deserializer)?;
                if let Ok(u32_value) = string_value.parse::<u32>() {
                    match TryInto::<$enum_type>::try_into(u32_value) {
                        Ok(enum_value) => return Ok(Some(enum_value)),
                        Err(err) => return Err(Error::custom(err)),
                    }
                }
                match TryInto::<$enum_type>::try_into(string_value.as_str()) {
                    Ok(enum_value) => Ok(Some(enum_value)),
                    Err(err) => Err(Error::custom(err)),
                }
            }

        )*

    };
}

create_serialize_deserialize_enum!(
    (
        serialize_block_dimension,
        deserialize_block_dimension,
        ktx_pack_astc_block_dimension_e
    ),
    (
        serialize_mode,
        deserialize_mode,
        ktx_pack_astc_encoder_mode_e
    ),
    (
        serialize_astc_quality_level,
        deserialize_astc_quality_level,
        ktx_pack_astc_quality_levels_e
    )
);

fn deserialize_input_swizzle<'a, D: Deserializer<'a>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let string_value = Option::<String>::deserialize(deserializer)?;
    match string_value {
        Some(string_value) => {
            if INPUT_SWIZZLE_REGEX.is_match(&string_value) {
                Ok(Some(string_value))
            } else {
                let error_message = format!(
                    "input_swizzle {string_value} does not match regex, {}",
                    INPUT_SWIZZLE_REGEX.as_str()
                );
                Err(Error::custom(error_message))
            }
        }
        None => Ok(None),
    }
}

fn serialize_uastc_flags<S: Serializer>(
    value: &Option<KtxPackUastcFlags>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(value) = value {
        serializer.serialize_u32(value.value)
    } else {
        serializer.serialize_none()
    }
}

fn deserialize_uastc_flags<'a, D: Deserializer<'a>>(
    deserializer: D,
) -> Result<Option<KtxPackUastcFlags>, D::Error> {
    let u32_value = u32::deserialize(deserializer)?;

    Ok(Some(u32_value.into()))
}
