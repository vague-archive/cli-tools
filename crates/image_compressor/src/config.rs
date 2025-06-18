use std::{env::current_dir, fmt::Display, path::PathBuf};

use ktx2_wrapper::{
    config::{
        BasisUniversalBasisLZETC1s, BasisUniversalUASTC, CompressionConfig, KTXCompressionConfig,
        KTXCompressionConfigTypes,
    },
    ffi2::KtxPackUastcFlags,
    ktx_types::UastcPackLevelOptions,
};
use serde::{de::Error, Deserialize, Deserializer, Serialize};

use crate::{CompressionTypes, NumberOfThreads};

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Config {
    #[serde(
        deserialize_with = "deserialize_from_directory",
        default = "default_from_directory"
    )]
    pub from_directory: PathBuf,
    #[serde(
        deserialize_with = "deserialize_to_directory",
        default = "default_to_directory"
    )]
    to_directory: Option<PathBuf>,
    #[serde(default = "default_delete_original_images")]
    delete_original_images: bool,
    #[serde(default = "default_ignore_list")]
    ignore_list: Vec<PathBuf>,
    #[serde(default = "default_compression_container")]
    compression_container: CompressionTypes,
    #[serde(default = "default_number_of_threads")]
    pub number_of_threads: Option<NumberOfThreads>,
    #[serde(default = "default_skip_errors")]
    skip_errors: bool,
    #[serde(default = "default_verbose")]
    verbose: bool,
    #[serde(default = "default_compression_config")]
    compression_config: CompressionConfig,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Config {
    pub fn to_directory(&self) -> &Option<PathBuf> {
        &self.to_directory
    }
    pub fn delete_original_images(&self) -> bool {
        self.delete_original_images
    }
    pub fn ignore_list(&self) -> &[PathBuf] {
        &self.ignore_list
    }
    pub fn compression_container(&self) -> CompressionTypes {
        self.compression_container
    }
    pub fn skip_errors(&self) -> bool {
        self.skip_errors
    }
    pub fn verbose(&self) -> bool {
        self.verbose
    }
    pub fn compression_config(&self) -> &CompressionConfig {
        &self.compression_config
    }
}

fn deserialize_from_directory<'a, D: Deserializer<'a>>(
    deserializer: D,
) -> Result<PathBuf, D::Error> {
    match PathBuf::deserialize(deserializer)?.canonicalize() {
        Ok(path_buf) => Ok(path_buf),
        Err(err) => Err(Error::custom(format!(
            "Must be able to canonicalize from_directory: {err}",
        ))),
    }
}

fn deserialize_to_directory<'a, D: Deserializer<'a>>(
    deserializer: D,
) -> Result<Option<PathBuf>, D::Error> {
    match Option::<PathBuf>::deserialize(deserializer)? {
        Some(path_buf) => match path_buf.canonicalize() {
            Ok(path_buf) => Ok(Some(path_buf)),
            Err(err) => Err(Error::custom(format!(
                "Must be able to canonicalize to_directory: {err}",
            ))),
        },
        None => Ok(None),
    }
}

fn default_from_directory() -> PathBuf {
    match current_dir() {
        Ok(directory) => directory
            .canonicalize()
            .expect("The current directory must be canonicalizable"),
        Err(err) => panic!("Could not find current_dir: {err}"),
    }
}

fn default_to_directory() -> Option<PathBuf> {
    None
}

fn default_delete_original_images() -> bool {
    false
}

fn default_ignore_list() -> Vec<PathBuf> {
    vec![]
}

fn default_compression_container() -> CompressionTypes {
    CompressionTypes::default()
}

fn default_number_of_threads() -> Option<NumberOfThreads> {
    Some(NumberOfThreads::default())
}

fn default_skip_errors() -> bool {
    false
}

fn default_verbose() -> bool {
    false
}

fn default_compression_config() -> CompressionConfig {
    let etc1s_config = BasisUniversalBasisLZETC1s {
        thread_count: Some(4),
        compression_level: Some(4),
        ..Default::default()
    };
    CompressionConfig {
        config_type: KTXCompressionConfigTypes::BasisUniversalBasisLZETC1s,
        config: KTXCompressionConfig::BasisUniversalBasisLZETC1s(etc1s_config),
        premultiply: Some(true),
    }
}

// In a future version, I want the CLI to be able to generate JSON configs, and
// this will be the UASTC config. It's dead right now but in the future it will not be
#[allow(dead_code)]
fn default_uastc_config() -> CompressionConfig {
    let uastc_config = BasisUniversalUASTC {
        thread_count: Some(4),
        uastc_flags: Some(KtxPackUastcFlags::new(&UastcPackLevelOptions::Slower)),
        ..Default::default()
    };
    CompressionConfig {
        config_type: KTXCompressionConfigTypes::BasisUniversalUASTC,
        config: KTXCompressionConfig::BasisUniversalUASTC(uastc_config),
        premultiply: Some(true),
    }
}
