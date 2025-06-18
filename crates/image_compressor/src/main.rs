use std::{
    error::Error,
    fmt,
    fs::{create_dir_all, remove_file, write, DirEntry, File},
    io::BufReader,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use clap::{arg, command, error::ErrorKind, value_parser, ArgAction, ArgMatches, Command};
use config::Config;
use image::{io::Reader, DynamicImage};
use ktx2_wrapper::write_texture_from_image;
use log::{info, trace};
use num_traits::Bounded;
use rayon::{prelude::*, ThreadPoolBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_string};
use shared_types::SupportedImages;
use strum::{EnumIter, EnumString};
use texpresso::{Format, Params};

mod config;

type LocalError = Box<dyn Error + Send + Sync>;

const DXT1_EXTENSION: &str = "dxt1";
const DXT4_EXTENSION: &str = "dxt4";

#[allow(clippy::upper_case_acronyms)]
#[derive(
    Clone, Copy, Debug, Default, strum::Display, EnumIter, EnumString, Serialize, Deserialize,
)]
enum CompressionTypes {
    DXT,
    #[default]
    KTX,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct NumberOfThreads(u8);

impl fmt::Display for NumberOfThreads {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for NumberOfThreads {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for NumberOfThreads {
    fn default() -> Self {
        Self(4)
    }
}

impl Bounded for NumberOfThreads {
    fn min_value() -> Self {
        Self(1)
    }

    fn max_value() -> Self {
        Self(20)
    }
}

impl From<u8> for NumberOfThreads {
    fn from(value: u8) -> Self {
        Self(value.clamp(
            NumberOfThreads::min_value().0,
            NumberOfThreads::max_value().0,
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImageMetadata {
    pub extension: &'static str,
    pub width: usize,
    pub height: usize,
}

fn premultiply_alpha(mut image: DynamicImage) -> Result<DynamicImage, LocalError> {
    let rgba = image
        .as_mut_rgba8()
        .ok_or("While premultiplying the alpha, could not process image a RGBA".to_string())?;
    for pixel in rgba.chunks_exact_mut(4) {
        pixel[0] = ((pixel[0] as u32 * pixel[3] as u32) / 255) as u8;
        pixel[1] = ((pixel[1] as u32 * pixel[3] as u32) / 255) as u8;
        pixel[2] = ((pixel[2] as u32 * pixel[3] as u32) / 255) as u8;
    }
    Ok(image)
}

fn handle_ktx_images_conversion(
    image_paths: &[PathBuf],
    config: &Config,
) -> Result<(), LocalError> {
    let images_processed = Arc::new(AtomicUsize::new(0));
    let total_images = image_paths.len();

    if config.verbose() {
        println!("Total images to be processed: {total_images}");
    }

    image_paths.par_iter().try_for_each(|path_buf| {
        let error_occured = if let Err(err) = convert_image_to_ktx(path_buf, config) {
            (true, Some(err))
        } else {
            (false, None)
        };
        let current_count = images_processed.fetch_add(1, Ordering::Acquire);
        if config.verbose() && total_images > 20 && current_count % (total_images / 20) == 0 {
            println!(
                "Processed {}% of images",
                (current_count * 100 / total_images) + 1
            );
        }
        if error_occured.0 {
            eprintln!(
                "Failed to convert {}: {}",
                path_buf.display(),
                error_occured.1.as_ref().unwrap()
            );
            if !config.skip_errors() {
                return Err(error_occured.1.unwrap());
            }
        }
        Ok(())
    })
}

fn convert_image_to_ktx(image_path: &Path, config: &Config) -> Result<(), LocalError> {
    trace!("Begin Converting {:?}", image_path);
    let image_path_out = if let Some(to_directory) = config.to_directory() {
        let image_path_out =
            to_directory.join(image_path.strip_prefix(config.from_directory.as_path())?);
        if !image_path_out.parent().unwrap().exists() {
            create_dir_all(image_path_out.parent().unwrap())?;
        }
        Some(image_path_out)
    } else {
        None
    };
    write_texture_from_image(
        image_path,
        image_path_out.as_deref(),
        config.compression_config(),
        &image_path.try_into()?,
    )?;
    if config.delete_original_images() {
        remove_file(image_path)?;
    }
    trace!("Finish Converting {:?}", image_path);
    Ok(())
}

fn handle_dxt_images_conversion(
    image_paths: &[PathBuf],
    config: &Config,
) -> Result<(), LocalError> {
    let images_processed = Arc::new(AtomicUsize::new(0));
    let total_images = image_paths.len();
    if config.verbose() {
        println!("Total images to be processed: {total_images}");
    }

    image_paths.par_iter().try_for_each(|path_buf| {
        let error_occured = if let Err(err) = convert_image_to_dxt(path_buf, config) {
            (true, Some(err))
        } else {
            (false, None)
        };
        let current_count = images_processed.fetch_add(1, Ordering::Acquire);
        if config.verbose() && total_images > 20 && current_count % (total_images / 20) == 0 {
            println!(
                "Processed {}% of images",
                (current_count * 100 / total_images) + 1
            );
        }
        if error_occured.0 {
            eprintln!(
                "Failed to convert {}: {}",
                path_buf.display(),
                error_occured.1.as_ref().unwrap()
            );
            if !config.skip_errors() {
                return Err(error_occured.1.unwrap());
            }
        }
        Ok(())
    })
}

fn convert_image_to_dxt(image_path: &Path, config: &Config) -> Result<(), LocalError> {
    trace!("Begin Converting {:?}", image_path);

    let mut image: DynamicImage = Reader::open(image_path)?.decode()?;
    if !image.color().has_alpha() {
        let mut image_with_alpha = image.to_rgba8();
        for pixel in image_with_alpha.pixels_mut() {
            pixel[3] = 255;
        }
        image = DynamicImage::ImageRgba8(image_with_alpha);
    }
    // For now, convert everything to RGBA
    if !matches!(image, DynamicImage::ImageRgba8(_)) {
        image = DynamicImage::ImageRgba8(image.into());
    }
    let has_alpha_mask = has_alpha_mask(&image)?;
    let width = image.width() as usize;
    let height = image.height() as usize;

    let should_premultiply = config.compression_config().premultiply.unwrap_or(true);
    let image_u8 = if should_premultiply {
        premultiply_alpha(image)?.as_bytes().to_vec()
    } else {
        image.as_bytes().to_vec()
    };

    // texpresso requires the output array be presized
    let compressed_image_size = if has_alpha_mask {
        Format::Bc3.compressed_size(width, height)
    } else {
        Format::Bc1.compressed_size(width, height)
    };
    let mut output = vec![0; compressed_image_size];
    if has_alpha_mask {
        Format::Bc3.compress(&image_u8, width, height, Params::default(), &mut output);
    } else {
        Format::Bc1.compress(&image_u8, width, height, Params::default(), &mut output);
    }

    let mut dxt_extension_path = if let Some(to_directory) = config.to_directory() {
        let dxt_extension_path = to_directory
            .join(image_path.strip_prefix(config.from_directory.as_path())?)
            .clone();
        if !dxt_extension_path.parent().unwrap().exists() {
            create_dir_all(dxt_extension_path.parent().unwrap())?;
        }
        dxt_extension_path
    } else {
        image_path.to_path_buf()
    };
    let dxt_extension = if has_alpha_mask {
        DXT4_EXTENSION
    } else {
        DXT1_EXTENSION
    };
    dxt_extension_path.set_extension(dxt_extension);
    write(dxt_extension_path.as_path(), output)?;

    let image_metadata = ImageMetadata {
        extension: dxt_extension,
        width,
        height,
    };
    let json_string = to_string(&image_metadata)?;
    let mut json_path = image_path.to_path_buf();
    json_path.set_extension("json");
    write(json_path, json_string)?;

    if config.delete_original_images() {
        remove_file(image_path)?;
    }

    trace!("Finish Converting {:?}", image_path);
    Ok(())
}

fn has_alpha_mask(image: &DynamicImage) -> Result<bool, LocalError> {
    let has_alpha_mask = image
        .as_rgba8()
        .ok_or("While scanning for alpha mask, could not convert image to rgba".to_string())?
        .chunks_exact(4)
        .any(|pixel| pixel[3] < u8::MAX);

    Ok(has_alpha_mask)
}

fn should_ignore_entry(entry: &DirEntry, ignore_list: &[PathBuf]) -> bool {
    ignore_list.iter().any(|ignore_list_entry| {
        if ignore_list_entry.extension().is_some() {
            entry.path().ends_with(ignore_list_entry)
        } else {
            entry.path().starts_with(ignore_list_entry)
        }
    })
}

fn find_images(dir_to_walk: &Path, ignore_list: &[PathBuf]) -> Result<Vec<PathBuf>, LocalError> {
    let directory = dir_to_walk.read_dir()?;

    let mut image_paths = vec![];

    for entry in directory {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => return Err(format!("Error reading {dir_to_walk:?}: {err}").into()),
        };

        let path = entry.path();
        let path_display = path.display();
        trace!("Begin scanning {path_display}");

        if should_ignore_entry(&entry, ignore_list) {
            continue;
        }

        if path.is_dir() {
            image_paths.extend(find_images(path.as_path(), ignore_list)?);
            continue;
        }

        trace!("Finish scanning {path_display}");

        if SupportedImages::is_valid_path(path.as_path()) {
            image_paths.push(path);
        }
    }

    Ok(image_paths)
}

fn handle_compress_subcommand(arg_matches: &ArgMatches, command: &mut Command) {
    let instant = std::time::Instant::now();
    info!("Begin config validation");
    let mut config = if let Some(config_string_path) = arg_matches.get_one::<String>("config") {
        let config_path = PathBuf::from(config_string_path);
        if !config_path.exists() {
            command
                .error(
                    ErrorKind::InvalidValue,
                    format!("Config does not exist at path {config_string_path})"),
                )
                .exit()
        }
        if !config_path.is_file() {
            command
                .error(
                    ErrorKind::InvalidValue,
                    format!("Config {config_string_path} must be a file"),
                )
                .exit()
        }
        let config_file = match File::open(config_path) {
            Ok(config_file) => config_file,
            Err(err) => command
                .error(
                    ErrorKind::Io,
                    format!("Could not open file at {config_string_path}: {err}"),
                )
                .exit(),
        };
        let config_reader = BufReader::new(config_file);
        match from_reader::<BufReader<File>, Config>(config_reader) {
            Ok(config) => config,
            Err(err) => command
                .error(
                    ErrorKind::Io,
                    format!("Could not deserialize {config_string_path} to Rust object: {err}"),
                )
                .exit(),
        }
    } else {
        Config::default()
    };
    info!("Finish config validation");

    if let Some(output_dir) = arg_matches.get_one::<String>("dir") {
        match PathBuf::from(output_dir).canonicalize() {
            Ok(canonicalized_path) => config.from_directory = canonicalized_path,
            Err(err) => command
                .error(
                    ErrorKind::InvalidValue,
                    format!(
                        "Path {output_dir} could not be canonicalized, it is likely invalid: {err}"
                    ),
                )
                .exit(),
        }
    }

    if let Some(threads) = arg_matches.get_one::<u8>("threads") {
        config.number_of_threads = Some((*threads).into());
    }

    info!("Begin Directory Validation");

    if !config.from_directory.is_dir() {
        command
            .error(
                ErrorKind::InvalidValue,
                format!(
                    "Directory {} either doesn't exist or is not actually a directory",
                    config.from_directory.display()
                ),
            )
            .exit();
    }

    if let Some(to_directory) = config.to_directory() {
        if !to_directory.exists() {
            if let Err(err) = create_dir_all(to_directory) {
                command
                    .error(
                        ErrorKind::Io,
                        format!(
                            "to_directory path {} does not exist, and could not be created: {err}",
                            to_directory.display()
                        ),
                    )
                    .exit()
            };
        }
    }

    if !config.from_directory.is_dir() {
        command
            .error(
                ErrorKind::Io,
                format!(
                    "from_directory path {} is not a directory",
                    config.from_directory.display()
                ),
            )
            .exit()
    }
    let ignore_list = config
        .ignore_list()
        .iter()
        .map(|ignore_list_item| {
            let mut item_path_buf: PathBuf = ignore_list_item.into();
            if item_path_buf.extension().is_none() {
                item_path_buf = config.from_directory.join(item_path_buf);
            }
            item_path_buf
        })
        .collect::<Vec<PathBuf>>();
    if let Some(number_of_threads) = config.number_of_threads {
        if let Err(err) = ThreadPoolBuilder::new()
            .num_threads(*number_of_threads as usize)
            .build_global()
        {
            command
                .error(
                    ErrorKind::InvalidValue,
                    format!("Invalid number of threads: {number_of_threads}: {:?}", err),
                )
                .exit();
        }
    };

    info!("Finish Directory Validation");
    info!("Begin Directory Scan And Conversion");
    let image_paths = match find_images(&config.from_directory, &ignore_list) {
        Ok(image_paths) => image_paths,
        Err(err) => {
            command
                .error(
                    ErrorKind::InvalidValue,
                    format!("Error finding images: {err}"),
                )
                .exit();
        }
    };

    if config.verbose() {
        println!("Beginning compression, compression config is: {config}");
    }
    match config.compression_container() {
        CompressionTypes::DXT => {
            if let Err(err) = handle_dxt_images_conversion(&image_paths, &config) {
                command
                    .error(
                        ErrorKind::InvalidValue,
                        format!("Error converting images: {err}"),
                    )
                    .exit();
            }
        }
        CompressionTypes::KTX => {
            if let Err(err) = handle_ktx_images_conversion(&image_paths, &config) {
                command
                    .error(
                        ErrorKind::InvalidValue,
                        format!("Error converting images: {err}"),
                    )
                    .exit()
            }
        }
    }
    let elapsed_time_message = format!("ELAPSED TIME {:?}", instant.elapsed());
    if config.verbose() {
        println!("{elapsed_time_message}");
    } else {
        trace!("{elapsed_time_message}");
    }
    info!("Finish Directory Scan and Conversion");
}

fn handle_config_subcommand(arg_matches: &ArgMatches, command: &mut Command) {
    let Some(config_string_path) = arg_matches.get_one::<String>("validate") else {
        command
            .error(
                ErrorKind::MissingRequiredArgument,
                "Could not find -v or --validation which should have a path to your config",
            )
            .exit();
    };
    let config_path = PathBuf::from(config_string_path);
    if !config_path.exists() {
        command
            .error(
                ErrorKind::InvalidValue,
                format!("Config does not exist at path {config_string_path})"),
            )
            .exit()
    }
    if !config_path.is_file() {
        command
            .error(
                ErrorKind::InvalidValue,
                format!("Config {config_string_path} must be a file"),
            )
            .exit()
    }
    let config_file = match File::open(config_path) {
        Ok(config_file) => config_file,
        Err(err) => command
            .error(
                ErrorKind::Io,
                format!("Could not open file at {config_string_path}: {err}"),
            )
            .exit(),
    };
    let config_reader = BufReader::new(config_file);
    match from_reader::<BufReader<File>, Config>(config_reader) {
        Ok(_) => {
            println!("Config is valid");
        }
        Err(err) => command
            .error(
                ErrorKind::Io,
                format!("Could not deserialize {config_string_path} to Rust object: {err}"),
            )
            .exit(),
    }
}

fn main() {
    env_logger::init();
    let mut command = command!()
        .subcommand(
            command!()
                .name("compress")
                .about("Compresses images to GPU format")
                .arg(
                    arg!(
                        -c --config <CONFIG> "Optional path to JSON config for config"
                    )
                    .required(false)
                    .num_args(1),
                )
                .arg(
                    arg!(
                        -d --dir <DIRECTORY> "Directory to recursively convert images to GPU format"
                    )
                    .required(false)
                    .num_args(1)
                    .action(ArgAction::Set),
                )
                .arg(
                    arg!(
                        -t --threads "The number of threads this process will use"
                    )
                    .required(false)
                    .num_args(1)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(u8)),
                ),
        )
        .subcommand(
            command!()
                .name("config")
                .about("Validate compression configs")
                .arg(
                    arg!(
                        -v --validate <VALIDATE> "Path to compression config to validate config"
                    )
                    .required(true)
                    .num_args(1),
                ),
        );
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("compress", sub_matches)) => handle_compress_subcommand(sub_matches, &mut command),
        Some(("config", sub_matches)) => handle_config_subcommand(sub_matches, &mut command),
        Some((unknown_command, _)) => command
            .error(
                ErrorKind::InvalidSubcommand,
                format!("Subcommand {unknown_command} is invalid"),
            )
            .exit(),
        None => command
            .error(ErrorKind::InvalidSubcommand, "A subcommand is required")
            .exit(),
    }
}
