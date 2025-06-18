use std::{
    fmt::{Debug, Display},
    ptr::null,
};

use shared_types::LocalError;
use strum::Display;

use crate::{
    ffi::{
        ktxTextureCreateInfo, ktx_error_code_e, ktx_pack_astc_block_dimension_e,
        ktx_pack_astc_encoder_mode_e, ktx_pack_astc_quality_levels_e, KtxBasisETC1SParams,
        KtxBasisUASTCParams, VkFormat,
    },
    ffi2::{KtxBasisParams, KtxPackUastcFlags},
};

impl Copy for ktx_pack_astc_block_dimension_e {}

impl Debug for ktx_pack_astc_block_dimension_e {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_4x4")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x4 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_5x4")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_5x5")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x5 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_6x5")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_6x6")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x5 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_8x5")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x6 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_8x6")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x5 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_10x5")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x6 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_10x6")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x8 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_8x8")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x8 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_10x8")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x10 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_10x10")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_12x10 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_12x10")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_12x12 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_12x12")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_3x3x3 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_3x3x3")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x3x3 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_4x3x3")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x3 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x3")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x4 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x4")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x4x4 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_5x4x4")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x4 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x4")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x5 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x5")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x5x5 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_6x5x5")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x5 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x5")
            }
            Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x6 => {
                write!(f, "KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x6")
            }
        }
    }
}

impl Display for ktx_pack_astc_block_dimension_e {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u32> for ktx_pack_astc_block_dimension_e {
    type Error = LocalError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x4 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x4),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x5 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x5),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x5 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x5),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x6 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x6),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x5 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x5),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x6 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x6),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x8 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x8),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x8 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x8),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x10 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x10),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_12x10 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_12x10),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_12x12 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_12x12),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_3x3x3 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_3x3x3),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x3x3 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x3x3),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x3 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x3),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x4 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x4),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x4x4 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x4x4),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x4 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x4),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x5 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x5),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x5x5 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x5x5),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x5 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x5),
            x if x == Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x6 as u32 => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x6),
            _ => Err(format!("value {value} is outside the range of possible values for ktx_pack_astc_block_dimension").into())
        }
    }
}

impl TryFrom<&str> for ktx_pack_astc_block_dimension_e {
    type Error = LocalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "KTX_PACK_ASTC_BLOCK_DIMENSION_4x4" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_5x4" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x4),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_5x5" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_6x5" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x5),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_6x6" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_8x5" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x5),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_8x6" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x6),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_10x5" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x5),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_10x6" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x6),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_8x8" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_8x8),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_10x8" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x8),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_10x10" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_10x10),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_12x10" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_12x10),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_12x12" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_12x12),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_3x3x3" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_3x3x3),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_4x3x3" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x3x3),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x3" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x3),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x4" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_4x4x4),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_5x4x4" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x4x4),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x4" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x4),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x5" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_5x5x5),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_6x5x5" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x5x5),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x5" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x5),
            "KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x6" => Ok(Self::KTX_PACK_ASTC_BLOCK_DIMENSION_6x6x6),
            _ => Err(format!(
                "value {value} not expected as a member of ktx_pack_astc_block_dimension"
            )
            .into()),
        }
    }
}

impl TryFrom<String> for ktx_pack_astc_block_dimension_e {
    type Error = LocalError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl Copy for ktx_pack_astc_encoder_mode_e {}

impl Debug for ktx_pack_astc_encoder_mode_e {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KTX_PACK_ASTC_ENCODER_MODE_DEFAULT => {
                write!(f, "KTX_PACK_ASTC_ENCODER_MODE_DEFAULT")
            }
            Self::KTX_PACK_ASTC_ENCODER_MODE_LDR => write!(f, "KTX_PACK_ASTC_ENCODER_MODE_LDR"),
            Self::KTX_PACK_ASTC_ENCODER_MODE_HDR => write!(f, "KTX_PACK_ASTC_ENCODER_MODE_HDR"),
        }
    }
}

impl Display for ktx_pack_astc_encoder_mode_e {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u32> for ktx_pack_astc_encoder_mode_e {
    type Error = LocalError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::KTX_PACK_ASTC_ENCODER_MODE_DEFAULT as u32 => {
                Ok(Self::KTX_PACK_ASTC_ENCODER_MODE_DEFAULT)
            }
            x if x == Self::KTX_PACK_ASTC_ENCODER_MODE_HDR as u32 => {
                Ok(Self::KTX_PACK_ASTC_ENCODER_MODE_HDR)
            }
            x if x == Self::KTX_PACK_ASTC_ENCODER_MODE_LDR as u32 => {
                Ok(Self::KTX_PACK_ASTC_ENCODER_MODE_LDR)
            }
            _ => Err(format!(
                "value {value} not expected as a member of ktx_pack_astc_encoder_mode_e"
            )
            .into()),
        }
    }
}

impl TryFrom<&str> for ktx_pack_astc_encoder_mode_e {
    type Error = LocalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "KTX_PACK_ASTC_ENCODER_MODE_DEFAULT" => Ok(Self::KTX_PACK_ASTC_ENCODER_MODE_DEFAULT),
            "KTX_PACK_ASTC_ENCODER_MODE_HDR" => Ok(Self::KTX_PACK_ASTC_ENCODER_MODE_HDR),
            "KTX_PACK_ASTC_ENCODER_MODE_LDR" => Ok(Self::KTX_PACK_ASTC_ENCODER_MODE_LDR),
            _ => Err(format!(
                "value {value} not expected as a member of ktx_pack_astc_block_dimension"
            )
            .into()),
        }
    }
}

impl TryFrom<String> for ktx_pack_astc_encoder_mode_e {
    type Error = LocalError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl Copy for ktx_pack_astc_quality_levels_e {}

impl Debug for ktx_pack_astc_quality_levels_e {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KTX_PACK_ASTC_QUALITY_LEVEL_FASTEST => {
                write!(f, "KTX_PACK_ASTC_QUALITY_LEVEL_FASTEST")
            }
            Self::KTX_PACK_ASTC_QUALITY_LEVEL_FAST => write!(f, "KTX_PACK_ASTC_QUALITY_LEVEL_FAST"),
            Self::KTX_PACK_ASTC_QUALITY_LEVEL_MEDIUM => {
                write!(f, "KTX_PACK_ASTC_QUALITY_LEVEL_MEDIUM")
            }
            Self::KTX_PACK_ASTC_QUALITY_LEVEL_THOROUGH => {
                write!(f, "KTX_PACK_ASTC_QUALITY_LEVEL_THOROUGH")
            }
            Self::KTX_PACK_ASTC_QUALITY_LEVEL_EXHAUSTIVE => {
                write!(f, "KTX_PACK_ASTC_QUALITY_LEVEL_EXHAUSTIVE")
            }
        }
    }
}

impl Display for ktx_pack_astc_quality_levels_e {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u32> for ktx_pack_astc_quality_levels_e {
    type Error = LocalError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::KTX_PACK_ASTC_QUALITY_LEVEL_FASTEST as u32 => {
                Ok(Self::KTX_PACK_ASTC_QUALITY_LEVEL_FASTEST)
            }
            x if x == Self::KTX_PACK_ASTC_QUALITY_LEVEL_FAST as u32 => {
                Ok(Self::KTX_PACK_ASTC_QUALITY_LEVEL_FAST)
            }
            x if x == Self::KTX_PACK_ASTC_QUALITY_LEVEL_MEDIUM as u32 => {
                Ok(Self::KTX_PACK_ASTC_QUALITY_LEVEL_MEDIUM)
            }
            x if x == Self::KTX_PACK_ASTC_QUALITY_LEVEL_THOROUGH as u32 => {
                Ok(Self::KTX_PACK_ASTC_QUALITY_LEVEL_THOROUGH)
            }
            x if x == Self::KTX_PACK_ASTC_QUALITY_LEVEL_EXHAUSTIVE as u32 => {
                Ok(Self::KTX_PACK_ASTC_QUALITY_LEVEL_EXHAUSTIVE)
            }
            _ => Err(format!(
                "value {value} not expected as a member of ktx_pack_astc_encoder_mode_e"
            )
            .into()),
        }
    }
}

impl TryFrom<&str> for ktx_pack_astc_quality_levels_e {
    type Error = LocalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "KTX_PACK_ASTC_QUALITY_LEVEL_FASTEST" => Ok(Self::KTX_PACK_ASTC_QUALITY_LEVEL_FASTEST),
            "KTX_PACK_ASTC_QUALITY_LEVEL_FAST" => Ok(Self::KTX_PACK_ASTC_QUALITY_LEVEL_FAST),
            "KTX_PACK_ASTC_QUALITY_LEVEL_MEDIUM" => Ok(Self::KTX_PACK_ASTC_QUALITY_LEVEL_MEDIUM),
            "KTX_PACK_ASTC_QUALITY_LEVEL_THOROUGH" => {
                Ok(Self::KTX_PACK_ASTC_QUALITY_LEVEL_THOROUGH)
            }
            "KTX_PACK_ASTC_QUALITY_LEVEL_EXHAUSTIVE" => {
                Ok(Self::KTX_PACK_ASTC_QUALITY_LEVEL_EXHAUSTIVE)
            }
            _ => Err(format!(
                "value {value} not expected as a member of ktx_pack_astc_encoder_mode_e"
            )
            .into()),
        }
    }
}

impl TryFrom<String> for ktx_pack_astc_quality_levels_e {
    type Error = LocalError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl Default for KtxPackUastcFlags {
    fn default() -> Self {
        KtxPackUastcFlags::new(&UastcPackLevelOptions::default())
    }
}

impl From<u32> for KtxPackUastcFlags {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Copy, Clone, Debug, Default, Display, PartialEq, Eq)]
pub enum UastcPackLevelOptions {
    Fastest = 0,
    Faster = 1,
    #[default]
    Default = 2,
    Slower = 3,
    VerySlow = 4,
}

impl TryFrom<u32> for UastcPackLevelOptions {
    type Error = LocalError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => UastcPackLevelOptions::Fastest,
            1 => UastcPackLevelOptions::Faster,
            2 => UastcPackLevelOptions::Default,
            3 => UastcPackLevelOptions::Slower,
            4 => UastcPackLevelOptions::VerySlow,
            unexpected_value => return Err(format!("Unexpected value {unexpected_value}").into()),
        })
    }
}

trait UastcFlags {
    const FAVOR_UASTC_ERROR_BIT_POSITION: u32 = 3;
    const FAVOR_BC7_ERROR_BIT_POSITION: u32 = 4;
    const ETC1_FASTER_HINTS_BIT_POSITION: u32 = 5;
    const ETC1_FASTEST_HINTS_BIT_POSIITON: u32 = 6;
    const DISABLE_FLIP_AND_INDIVIDUAL_BIT_POSITION: u32 = 7;
}

impl UastcFlags for KtxPackUastcFlags {}

impl KtxPackUastcFlags {
    pub fn new(pack_level: &UastcPackLevelOptions) -> Self {
        Self {
            value: *pack_level as u32,
        }
    }

    pub fn get_pack_level(&self) -> Result<UastcPackLevelOptions, LocalError> {
        let first_three_bits = self.value & 0b111;
        first_three_bits.try_into()
    }

    pub fn flip_favor_uastc_error(&mut self) {
        self.value ^= 1 << Self::FAVOR_UASTC_ERROR_BIT_POSITION;
    }

    pub fn get_favor_uastc_error(&self) -> bool {
        ((self.value >> Self::FAVOR_UASTC_ERROR_BIT_POSITION) & 1) == 1
    }

    pub fn flip_favor_bc7_error(&mut self) {
        self.value ^= 1 << Self::FAVOR_BC7_ERROR_BIT_POSITION;
    }

    pub fn get_favor_bc7_error(&self) -> bool {
        ((self.value >> Self::FAVOR_BC7_ERROR_BIT_POSITION) & 1) == 1
    }

    pub fn flip_etc1_faster_hints(&mut self) {
        self.value ^= 1 << Self::ETC1_FASTER_HINTS_BIT_POSITION;
    }

    pub fn get_etc1_faster_hints(&self) -> bool {
        ((self.value >> Self::ETC1_FASTER_HINTS_BIT_POSITION) & 1) == 1
    }

    pub fn flip_etc1_fastest_hints(&mut self) {
        self.value ^= 1 << Self::ETC1_FASTEST_HINTS_BIT_POSIITON;
    }

    pub fn get_etc1_fastest_hints(&self) -> bool {
        ((self.value >> Self::ETC1_FASTEST_HINTS_BIT_POSIITON) & 1) == 1
    }

    pub fn flip_disable_flip_and_individual(&mut self) {
        self.value ^= 1 << Self::DISABLE_FLIP_AND_INDIVIDUAL_BIT_POSITION;
    }

    pub fn get_disable_flip_and_individual(&self) -> bool {
        ((self.value >> Self::DISABLE_FLIP_AND_INDIVIDUAL_BIT_POSITION) & 1) == 1
    }
}

impl Default for KtxBasisETC1SParams {
    fn default() -> Self {
        Self {
            compression_level: null(),
            quality_level: null(),
            max_endpoints: null(),
            endpoint_rdo_threshold: null(),
            max_selectors: null(),
            selector_rdo_threshold: null(),
            input_swizzle: null(),
            normal_map: null(),
            separate_rgt_to_rgba: null(),
            pre_swizzle: null(),
            no_endpoint_rdo: null(),
            no_selector_rdo: null(),
        }
    }
}

impl Default for KtxBasisParams {
    fn default() -> Self {
        Self {
            struct_size: null(),
            uastc: false,
            verbose: null(),
            no_sse: null(),
            thread_count: null(),
        }
    }
}

impl Default for KtxBasisUASTCParams {
    fn default() -> Self {
        Self {
            uastc_flags: null(),
            uastc_rdo: null(),
            uastc_rdo_quality_scalar: null(),
            uastc_rdo_dict_size: null(),
            uastc_rdo_max_smooth_block_error_scale: null(),
            uastc_rdo_max_smooth_block_std_dev: null(),
            uastc_rdo_dont_favor_simpler_modes: null(),
            uastc_rdo_no_multithreading: null(),
            input_swizzle: null(),
            pre_swizzle: null(),
        }
    }
}

impl From<ktx_error_code_e> for String {
    fn from(value: ktx_error_code_e) -> Self {
        match value {
            ktx_error_code_e::KTX_SUCCESS => "KTX_SUCCESS",
            ktx_error_code_e::KTX_FILE_DATA_ERROR => "KTX_FILE_DATA_ERROR",
            ktx_error_code_e::KTX_FILE_ISPIPE => "KTX_FILE_ISPIPE",
            ktx_error_code_e::KTX_FILE_OPEN_FAILED => "KTX_FILE_OPEN_FAILED",
            ktx_error_code_e::KTX_FILE_OVERFLOW => "KTX_FILE_OVERFLOW",
            ktx_error_code_e::KTX_FILE_READ_ERROR => "KTX_FILE_READ_ERROR",
            ktx_error_code_e::KTX_FILE_SEEK_ERROR => "KTX_FILE_SEEK_ERROR",
            ktx_error_code_e::KTX_FILE_UNEXPECTED_EOF => "KTX_FILE_UNEXPECTED_EOF",
            ktx_error_code_e::KTX_FILE_WRITE_ERROR => "KTX_FILE_WRITE_ERROR",
            ktx_error_code_e::KTX_GL_ERROR => "KTX_GL_ERROR",
            ktx_error_code_e::KTX_INVALID_OPERATION => "KTX_INVALID_OPERATION",
            ktx_error_code_e::KTX_INVALID_VALUE => "KTX_INVALID_VALUE",
            ktx_error_code_e::KTX_NOT_FOUND => "KTX_NOT_FOUND",
            ktx_error_code_e::KTX_OUT_OF_MEMORY => "KTX_OUT_OF_MEMORY",
            ktx_error_code_e::KTX_TRANSCODE_FAILED => "KTX_TRANSCODE_FAILED",
            ktx_error_code_e::KTX_UNKNOWN_FILE_FORMAT => "KTX_UNKNOWN_FILE_FORMAT",
            ktx_error_code_e::KTX_UNSUPPORTED_TEXTURE_TYPE => "KTX_UNSUPPORTED_TEXTURE_TYPE",
            ktx_error_code_e::KTX_UNSUPPORTED_FEATURE => "KTX_UNSUPPORTED_FEATURE",
            ktx_error_code_e::KTX_LIBRARY_NOT_LINKED => "KTX_LIBRARY_NOT_LINKED",
            ktx_error_code_e::KTX_DECOMPRESS_LENGTH_ERROR => "KTX_DECOMPRESS_LENGTH_ERROR",
            ktx_error_code_e::KTX_DECOMPRESS_CHECKSUM_ERROR => "KTX_DECOMPRESS_CHECKSUM_ERROR",
        }
        .to_string()
    }
}

impl ktxTextureCreateInfo {
    pub fn new(width: u32, height: u32, format: VkFormat) -> Self {
        Self {
            baseWidth: width,
            baseHeight: height,
            vkFormat: format as u32,
            ..Default::default()
        }
    }

    pub fn image_size(&self) -> usize {
        (self.baseWidth * self.baseHeight * 4) as usize
    }
}

impl Default for ktxTextureCreateInfo {
    fn default() -> Self {
        Self {
            glInternalformat: 0,
            vkFormat: VkFormat::VK_FORMAT_UNDEFINED as u32,
            pDfd: std::ptr::null_mut(),
            baseWidth: 0,
            baseHeight: 0,
            baseDepth: 1,
            numDimensions: 2,
            numLevels: 1,
            numLayers: 1,
            numFaces: 1,
            isArray: false,
            generateMipmaps: false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ffi2::KtxPackUastcFlags, ktx_types::UastcPackLevelOptions};

    #[test]
    fn uastc_flags_set_as_expected() {
        let uastc_flag = KtxPackUastcFlags::new(&UastcPackLevelOptions::Fastest);
        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Fastest
        );
        assert_eq!(uastc_flag.get_pack_level().unwrap() as u32, 0);

        let uastc_flag = KtxPackUastcFlags::new(&UastcPackLevelOptions::Faster);
        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Faster
        );
        assert_eq!(uastc_flag.get_pack_level().unwrap() as u32, 1);

        let uastc_flag = KtxPackUastcFlags::new(&UastcPackLevelOptions::Default);
        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Default
        );
        assert_eq!(uastc_flag.get_pack_level().unwrap() as u32, 2);

        let uastc_flag = KtxPackUastcFlags::new(&UastcPackLevelOptions::Slower);
        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Slower
        );
        assert_eq!(uastc_flag.get_pack_level().unwrap() as u32, 3);

        let uastc_flag = KtxPackUastcFlags::new(&UastcPackLevelOptions::VerySlow);
        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::VerySlow
        );
        assert_eq!(uastc_flag.get_pack_level().unwrap() as u32, 4);
    }

    #[test]
    fn changing_uastc_flags_works_as_expected() {
        let mut uastc_flag = KtxPackUastcFlags::new(&UastcPackLevelOptions::Slower);
        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Slower
        );
        assert!(!uastc_flag.get_favor_uastc_error());
        uastc_flag.flip_favor_uastc_error();

        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Slower
        );
        assert!(uastc_flag.get_favor_uastc_error());
        assert!(!uastc_flag.get_favor_bc7_error());
        uastc_flag.flip_favor_bc7_error();

        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Slower
        );
        assert!(uastc_flag.get_favor_uastc_error());
        assert!(uastc_flag.get_favor_bc7_error());
        assert!(!uastc_flag.get_etc1_faster_hints());
        uastc_flag.flip_etc1_faster_hints();

        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Slower
        );
        assert!(uastc_flag.get_favor_uastc_error());
        assert!(uastc_flag.get_favor_bc7_error());
        assert!(uastc_flag.get_etc1_faster_hints());
        assert!(!uastc_flag.get_etc1_fastest_hints());
        uastc_flag.flip_etc1_fastest_hints();

        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Slower
        );
        assert!(uastc_flag.get_favor_uastc_error());
        assert!(uastc_flag.get_favor_bc7_error());
        assert!(uastc_flag.get_etc1_faster_hints());
        assert!(uastc_flag.get_etc1_fastest_hints());
        assert!(!uastc_flag.get_disable_flip_and_individual());
        uastc_flag.flip_disable_flip_and_individual();

        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Slower
        );
        assert!(uastc_flag.get_favor_uastc_error());
        assert!(uastc_flag.get_favor_bc7_error());
        assert!(uastc_flag.get_etc1_faster_hints());
        assert!(uastc_flag.get_etc1_fastest_hints());
        assert!(uastc_flag.get_disable_flip_and_individual());
        uastc_flag.flip_etc1_fastest_hints();
        uastc_flag.flip_favor_bc7_error();

        assert_eq!(
            uastc_flag.get_pack_level().unwrap(),
            UastcPackLevelOptions::Slower
        );
        assert!(uastc_flag.get_favor_uastc_error());
        assert!(!uastc_flag.get_favor_bc7_error());
        assert!(uastc_flag.get_etc1_faster_hints());
        assert!(!uastc_flag.get_etc1_fastest_hints());
        assert!(uastc_flag.get_disable_flip_and_individual());
        uastc_flag.value = 5;
        uastc_flag.flip_favor_bc7_error();
        uastc_flag.flip_etc1_fastest_hints();

        assert!(uastc_flag.get_favor_bc7_error());
        assert!(!uastc_flag.get_etc1_faster_hints());
        assert!(uastc_flag.get_etc1_fastest_hints());
        assert!(uastc_flag.get_pack_level().is_err());
    }
}
