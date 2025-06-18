use std::{
    error::Error,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use strum::Display;

pub type LocalError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Display, PartialEq, Eq)]
pub enum SupportedImages {
    Png,
    Jpeg,
}

impl SupportedImages {
    pub fn is_valid_path<P: AsRef<Path>>(input: P) -> bool {
        Self::try_from_path(input).is_ok()
    }

    fn try_from_path<P: AsRef<Path>>(input: P) -> Result<Self, LocalError> {
        let path = input.as_ref();
        let extension = path.extension().ok_or(LocalError::from(format!(
            "Could not find extension for path {}",
            path.display()
        )))?;
        extension.try_into()
    }

    fn try_from_string<S: AsRef<OsStr>>(input: S) -> Result<Self, LocalError> {
        let potential_image_str: &OsStr = input.as_ref();

        match potential_image_str.to_ascii_lowercase().to_str() {
            Some("png") => Ok(SupportedImages::Png),
            Some("jpeg" | "jpg") => Ok(SupportedImages::Jpeg),
            Some(unknown_extension) => {
                Err(format!("Unsupported image type {unknown_extension} found").into())
            }
            _ => Err("Unsupportd image type, type is not valid utf-8".into()),
        }
    }
}

// Can't use generics on TryFrom so using this macro to manually build a generic
macro_rules! create_path_variant_try_froms {
    ($($path_type:ty), *) => {
        $(
            impl TryFrom<$path_type> for SupportedImages {
                type Error = LocalError;

                fn try_from(value: $path_type) -> Result<Self, Self::Error> {
                    Self::try_from_path(value)
                }
            }

        )*
    };
}

macro_rules! create_string_variant_try_froms {
    ($($string_type:ty), *) => {
        $(
            impl TryFrom<$string_type> for SupportedImages {
                type Error = LocalError;

                fn try_from(value: $string_type) -> Result<Self, Self::Error> {
                    Self::try_from_string(value)
                }
            }

        )*
    };
}

create_path_variant_try_froms!(&Path, PathBuf, &PathBuf);
create_string_variant_try_froms!(&str, String, &String, &OsStr, OsString, &OsString);
