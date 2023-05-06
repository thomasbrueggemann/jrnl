use anyhow::{anyhow, Result};
use clap::Parser;
use colored::*;
use output_folder::OutputFolder;
use std::{ffi::OsStr, fs};
use strategies::{
    exif_strategy::ExifStrategy, filename_strategy::FilenameStrategy, mp4_strategy::Mp4Strategy,
    strategy::Strategy,
};
use walkdir::{DirEntry, WalkDir};

mod output_folder;
mod strategies;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input: String,

    #[clap(short, long)]
    output: String,

    #[clap(short, long)]
    dry_run: bool,
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn move_file(entry: DirEntry, output_dir: &str, dry_run: bool) {
    let file_path = entry.path().to_str().unwrap();

    match derive_output_folder(&entry) {
        Ok(output_folder_date) => {
            let new_path = format!("{}/{}", output_dir, output_folder_date.get_path());
            let new_file_path = format!("{}/{}", new_path, entry.file_name().to_str().unwrap());

            if dry_run {
                println!("Dry run: {} -> {}", file_path.red(), &new_file_path.green());

                return;
            }

            fs::create_dir_all(&new_path).unwrap();

            if let Ok(bytes_copied) = fs::copy(entry.path(), &new_file_path) {
                if bytes_copied > 0 {
                    fs::remove_file(entry.path()).unwrap();

                    println!("Moved {} -> {}", file_path.red(), &new_file_path.green());
                }
            }
        }
        Err(e) => {
            println!("Error {}: {}. Not moved!", file_path, e);
        }
    }
}

fn derive_output_folder(entry: &DirEntry) -> Result<OutputFolder> {
    let path = entry.path();

    if !path.is_file() {
        return Err(anyhow!(
            "{} is not a file, skipping",
            path.file_name().unwrap().to_str().unwrap()
        ));
    }

    let file_type = path
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_lowercase();

    let output_folder = match file_type.as_str() {
        "jpg" | "jpeg" | "png" | "tif" | "tiff" | "dng" => {
            ExifStrategy {}.derive_output_folder(&path)
        }
        //"mp4" | "mov" => Mp4Strategy {}.derive_output_folder(&file),
        _ => Err(anyhow!(
            "No strategy found for extension {}",
            file_type.clone()
        )),
    };

    match output_folder {
        Ok(folder) => Ok(folder),
        Err(e) => FilenameStrategy {}.derive_output_folder(&path),
    }
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    let output_folder = args.output.clone();
    let dry_run = args.dry_run.clone();

    WalkDir::new(args.input)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| move_file(x, &output_folder, dry_run));
}
