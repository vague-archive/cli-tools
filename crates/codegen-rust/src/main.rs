//! Code Generation tool for Rust.
//!
//! This tool takes an input file (e.g. manifest.json) with a json formatted
//! schema, and outputs a file (e.g. generated.rs) with Rust code which can
//! then be used to read/write messages in that schema.
//!
//! See [`../README.md`]

use std::{
    ffi::OsStr,
    fs::{read_to_string, remove_file, write, File},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Error, Result};
use clap::Parser;
use convert_case::{Case, Casing};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlatformLibrary {
    name: String,
    functions: Vec<Callable>,
    fbs: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Callable {
    name: String,
    parameter_data: Option<String>,
    return_value: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input = PathBuf::from(args.input);

    // Parse json into a `PlatformLibrary`.
    let platform_library = match parse_input(&input) {
        Ok(val) => val,
        Err(err) => {
            panic!("input error: {err} for {}", input.display());
        }
    };

    // Store output path.
    let output_path = args.output.map_or_else(
        || {
            input
                .with_file_name(platform_library.name.to_case(Case::Snake))
                .with_extension("rs")
        },
        PathBuf::from,
    );

    // Generate events with `flatc` (flatbuffers compiler).
    let generated_events = match generate_events(&platform_library, input.parent().unwrap()) {
        Ok(val) => val,
        Err(err) => {
            panic!("flatc error: {err}");
        }
    };

    // Generate rust output.
    let generated_string = generate_output(&platform_library, &generated_events);

    // Write to file.
    write(&output_path, generated_string).unwrap_or_else(|e| {
        panic!(
            "\
            {}: Error writing generated .rs file at {}, check \
            that the directory is writable and that there is disk \
            space available.",
            e,
            output_path.display(),
        )
    });
}

fn parse_input(input: &Path) -> Result<PlatformLibrary> {
    if input.extension() != Some(OsStr::new("json")) {
        return Err(Error::msg(
            "The input file must be a json file (with a .json file extension).",
        ));
    }

    serde_json::from_reader(File::open(input)?).map_err(Error::from)
}

fn generate_events(platform_library: &PlatformLibrary, dir: &Path) -> Result<String> {
    let fbs_path = dir.join("events.fbs");

    // write .fbs file from .json
    write(&fbs_path, &platform_library.fbs)?;

    // generate events .rs file
    Command::new("flatc")
        .arg("--rust")
        .arg("-o")
        .arg(dir)
        .arg(&fbs_path)
        .arg("--gen-object-api")
        .arg("--gen-name-strings")
        .spawn()
        .map_err(|err| Error::msg(format!("could not find flatc: {err}")))?
        .wait()?;

    let output_path = fbs_path.with_file_name("events_generated.rs");

    let events_generated = read_to_string(&output_path).map_err(Error::from)?;

    // cleanup generated files
    remove_file(&fbs_path)?;
    remove_file(&output_path)?;

    Ok(events_generated)
}

fn generate_output(platform_library: &PlatformLibrary, events_generated: &str) -> String {
    let mut output = String::new();

    output += "// The generated code may contain build warnings. Consider using\n";
    output += "// an annotation such as the following where this file is included:\n";
    output += "// `#![allow(clippy::all, clippy::pedantic, warnings, unused)]`\n\n";

    // Write each of the free functions.
    for function in platform_library
        .functions
        .iter()
        .filter(|f| !f.name.contains("::"))
    {
        let function_name = function.name.to_case(Case::Pascal);

        append_callable(&mut output, platform_library, function, &function_name, "");
    }

    // Find all system structs and put their functions in a `mod`.

    let mut prefixes = platform_library
        .functions
        .iter()
        .filter_map(|f| f.name.split_once("::"))
        .map(|(s, _)| s)
        .collect::<Vec<_>>();

    prefixes.sort_unstable();
    prefixes.dedup();

    for prefix in prefixes {
        output += &format!("pub mod {} {{\n", prefix.to_case(Case::Pascal));
        output += "    use super::*;\n\n";

        // iterate struct impl functions
        for function in platform_library
            .functions
            .iter()
            .filter(|f| f.name.starts_with(prefix))
        {
            let function_name = function
                .name
                .split_once("::")
                .unwrap()
                .1
                .to_case(Case::Pascal);

            append_callable(
                &mut output,
                platform_library,
                function,
                &function_name,
                "    ",
            );
        }

        output += "}\n\n";
    }

    output += events_generated;

    output
}

/// Writes a callable, which is a struct definition + impls
fn append_callable(
    output: &mut String,
    platform_library: &PlatformLibrary,
    function: &Callable,
    function_name: &str,
    indentation: &str,
) {
    // generate struct and Callable impl

    let parameters = function.parameter_data.as_ref().map_or_else(
        || "::void_public::callable::Pod<()>".to_owned(),
        |ident| {
            let mut ident = ident.clone();

            let regex = Regex::new(&format!("table\\s{ident}")).unwrap();
            if regex.is_match(&platform_library.fbs) {
                ident.push_str("<'a>");
            }

            ident
        },
    );

    let return_value = function.return_value.as_ref().map_or_else(
        || "::void_public::callable::Pod<()>".to_owned(),
        |ident| {
            let mut ident = ident.clone();

            let regex = Regex::new(&format!("table\\s{ident}")).unwrap();
            if regex.is_match(&platform_library.fbs) {
                ident.push_str("<'a>");
            }

            ident
        },
    );

    *output += &format!("{indentation}pub struct {function_name}Fn;\n\n");
    *output +=
        &format!("{indentation}impl ::void_public::callable::Callable for {function_name}Fn {{\n");
    *output += &format!("{indentation}    type Parameters<'a> = {parameters};\n");
    *output += &format!("{indentation}    type ReturnValue<'a> = {return_value};\n");
    *output += &format!("{indentation}}}\n\n");

    // generate EcsType impl

    let mut cid_var_name = function_name.to_case(Case::ScreamingSnake);
    cid_var_name.insert(0, '_');
    cid_var_name.push_str("_CID");

    let mut component_string_name = platform_library.name.clone();
    component_string_name.push_str("::");
    component_string_name.push_str(&function.name);

    *output += &format!(
        "{indentation}static mut {}: Option<::void_public::ComponentId> = None;\n\n",
        cid_var_name
    );

    *output += &format!("{indentation}impl ::void_public::EcsType for {function_name}Fn {{\n");
    *output += &format!("{indentation}    fn id() -> ::void_public::ComponentId {{\n");
    *output += &format!(
        "{indentation}        unsafe {{ {cid_var_name}.expect(\"ComponentId unassigned\") }}\n"
    );
    *output += &format!("{indentation}    }}\n\n");
    *output += &format!("{indentation}    fn set_id(id: ::void_public::ComponentId) {{\n");
    *output += &format!("{indentation}        unsafe {{ {cid_var_name} = Some(id); }}\n");
    *output += &format!("{indentation}    }}\n\n");
    *output += &format!("{indentation}    fn string_id() -> &'static ::std::ffi::CStr {{\n");
    *output += &format!("{indentation}        c\"{component_string_name}\"\n");
    *output += &format!("{indentation}    }}\n");
    *output += &format!("{indentation}}}\n\n");
}
