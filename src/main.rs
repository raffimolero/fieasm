mod fie;
mod helpers;

use crate::{
    fie::{FieErr, FieProgram},
    helpers::find_file,
};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};
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
    let out_dir = args.next();

    // open fie file
    if !filename.ends_with(".fie") {
        filename.push_str(".fie");
    }
    let file = find_file(&filename, exec_dir).ok_or(FileNotFound(filename.to_owned()))?;
    let program = FieProgram::try_from(file)?;
    println!("{program}");

    // TODO: compile machine code into RLE
    let rle = program.build();

    // TODO: put in clipboard or output file if in arguments
    if let Some(mut out_dir) = out_dir {
        println!("Output file specified.");
        println!("Attempting to output to file.");
        println!("Given path: {out_dir}.");

        if !out_dir.ends_with(".rle") {
            println!("Appending `.rle` to path.");
            out_dir.push_str(".rle");
        }
        let dir = Path::new(&out_dir);

        let out_dir = if dir.is_absolute() {
            println!("Absolute path detected.");
            dir.to_owned()
        } else {
            println!("Relative path detected.");
            let mut out_dir = env::current_dir().map_err(|e| PWDNotFound(e, dir.to_owned()))?;
            out_dir.push(dir);

            out_dir
        };

        println!("Attempting to output to {out_dir:?}");
        fs::write(&out_dir, rle).map_err(|e| WriteFileErr(e, out_dir))?;

        println!("Success!");
        println!("Check output file.");
        // fs::write(out_dir, )
    } else {
        println!("Output file not specified.");
        println!("Writing to clipboard.");
        let mut clip = ClipboardContext::new().map_err(OpenClipboardErr)?;
        clip.set_contents(rle).map_err(WriteClipboardErr)?;

        println!("Success!");
        println!("Check clipboard.");
    }

    Ok(())
}

fn main() {
    if let Err(e) = run_cli() {
        let red = "\x1b[0;31m";
        let default = "\x1b[0m";
        eprintln!("{red}Error: {e}{default}");
    }
}
