use std::{
    fs::{copy, create_dir},
    path::PathBuf,
};

use autocxx_build::Builder;
use cfg_if::cfg_if;
use cmake::Config;
use miette::Result;

fn main() -> Result<()> {
    let ktx_path = PathBuf::from("../../extern/ktx").canonicalize().unwrap();
    let out_directory = ktx_path.join("build");
    if !out_directory.exists() {
        create_dir(out_directory.as_path()).unwrap();
    }
    let mut ktx_builder = Config::new(&ktx_path);

    // Need this lint because for the linux builds this variable must be mut
    #[cfg_attr(not(target_os = "linux"), allow(unused_mut))]
    let mut ktx_builder = ktx_builder
        .define("CMAKE_BUILD_PARALLEL_LEVEL", "12")
        .define("KTX_FEATURE_STATIC_LIBRARY", "ON")
        .define("KTX_FEATURE_TESTS", "OFF")
        .define("KTX_FEATURE_TOOLS", "OFF")
        .out_dir(out_directory);

    cfg_if! {
        if #[cfg(target_os = "linux")] {
            ktx_builder.define("ASTCENC_SHAREDLIB", "ON");
        }
    }

    let build_output_path = ktx_builder.build();
    let ktx_include_path = build_output_path.join("include");
    let ktx_lib_path = build_output_path.join("lib");

    let support_headers = [
        "vk_format.h",
        "vkformat_enum.h",
        "gl_format.h",
        "formatsize.h",
    ];
    for support_header in support_headers {
        let support_header_path = ktx_path.join(format!("lib/{support_header}"));
        copy(support_header_path, ktx_include_path.join(support_header)).unwrap();
    }
    println!("cargo:rustc-link-search=native={}", ktx_lib_path.display());
    println!("cargo::rustc-link-lib=static=ktx");
    cfg_if! {
        if #[cfg(target_os = "linux")] {
            println!("cargo::rustc-link-lib=static=astcenc-avx2-static");
        }
    }

    let local_src_path = PathBuf::from("src");
    let mut builder = Builder::new(
        "src/lib.rs",
        [&ktx_include_path, &ktx_lib_path, &local_src_path],
    )
    .build()?;

    builder
        .flag_if_supported("-std=c++20")
        .file(local_src_path.join("ktx_wrappers.cpp"))
        .compile("ktx2_wrapper");

    let files_to_rerun_on = [
        "src/config.rs",
        "src/lib.rs",
        "src/ktx_wrappers.h",
        "src/ktx_wrappers.cpp",
        "src/ktx_types.rs",
        "src/ktx_texture.rs",
    ];

    for file_to_rerun_on in files_to_rerun_on {
        println!("cargo:rerun-if-changed={file_to_rerun_on}");
    }

    Ok(())
}
