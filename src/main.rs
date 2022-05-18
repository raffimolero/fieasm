mod fie;
mod helpers;

use crate::{
    fie::{FieErr, FieProgram},
    helpers::find_file,
};
use std::{env, io, path::PathBuf};
use thiserror::Error;

type ClipErr = Box<dyn std::error::Error + Send + Sync>;

#[derive(Error, Debug)]
pub enum CLIErr {
    #[error("No argument. Please specify a file to compile.")]
    NoArgument,

    #[error("Could not find {0} inside target directory.")]
    FileNotFound(String),

    #[error(
        "Given a relative path, but could not find present working directory.\n\
        Path: {1}\n\
        IO Error: {0}"
    )]
    PWDNotFound(io::Error, PathBuf),

    #[error(
        "Could not write to file.\n\
        Path: {1}\n\
        IO Error: {0}"
    )]
    WriteFileErr(io::Error, PathBuf),

    #[error(
        "Could not open clipboard.\n\
        Exact error: {0}"
    )]
    OpenClipboardErr(ClipErr),

    #[error(
        "Could not write to clipboard.\n\
        Exact error: {0}"
    )]
    WriteClipboardErr(ClipErr),

    #[error("{0}")]
    FieErr(#[from] FieErr),
}

pub fn run_cli() -> Result<(), CLIErr> {
    use CLIErr::*;

    let mut args = env::args();
    let exec_dir = args.next();
    let mut filename = args.next().ok_or(NoArgument)?;

    // open fie file
    if !filename.ends_with(".fie") {
        filename.push_str(".fie");
    }
    let file = find_file(&filename, exec_dir).ok_or_else(|| FileNotFound(filename.to_owned()))?;

    // File -> IR -> RLE -> stdout
    let program_ir = FieProgram::try_from(file)?;
    eprintln!("{program_ir}");

    let rle = program_ir.build();
    println!("{rle}");

    eprintln!("Program successfully compiled.");
    Ok(())
}

fn main() {
    if let Err(e) = run_cli() {
        let red = "\x1b[0;31m";
        let default = "\x1b[0m";
        eprintln!("{red}Error: {e}{default}");
    }
}
