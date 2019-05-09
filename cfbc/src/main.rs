#[macro_use]
extern crate clap;

mod cli_options;
mod exit_code;
mod generator;
mod helpers;
mod templates;

use cfb_schema::Schema;
use cli_options::CliOptions;
use exit_code::{ExitCode, ExitCodeWithMessage};
use generator::Generator;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{
    fs,
    io::{self, Read},
};

fn read_file_to_vec(path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut file = io::BufReader::new(fs::File::open(path)?);
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

fn build_output_file_name(dir: &PathBuf, stem: &OsStr, suffix: &str) -> PathBuf {
    let mut file_name = stem.to_os_string();
    file_name.push(suffix);
    dir.join(file_name)
}

fn open_output_file<P: AsRef<Path>>(path: P) -> io::Result<impl io::Write> {
    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    Ok(io::BufWriter::new(file))
}

fn run_app() -> Result<(), ExitCode> {
    let options = CliOptions::parse()?;

    let schema_buffer = read_file_to_vec(&options.bfbs)?;
    let schema = Schema::from_bytes(&schema_buffer);
    let g = Generator::new(schema);

    let stem = options
        .bfbs
        .file_stem()
        .ok_or_else(|| ExitCodeWithMessage::cli("<bfbs> is not a valid file name".to_string()))?;

    g.generate_builder(open_output_file(build_output_file_name(
        &options.output,
        stem,
        "_builder.rs",
    ))?)?;

    Ok(())
}

fn main() {
    if let Some(exit_code) = run_app().err() {
        exit_code.exit();
    }
}
