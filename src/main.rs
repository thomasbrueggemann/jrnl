use std::fs;

use anyhow::{anyhow, Result};
use clap::Parser;
use exif::{DateTime, In, Tag, Value};
use output_folder::OutputFolder;
use walkdir::{DirEntry, WalkDir};

mod output_folder;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input: String,

    #[clap(short, long)]
    output: String,
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn move_file(entry: DirEntry, output_dir: &str) {
    if let Ok(output_folder_date) = detect_date(&entry) {
        let new_path = format!("{}/{}", output_dir, output_folder_date.get_path());
        let new_file_path = format!("{}/{}", new_path, entry.file_name().to_str().unwrap());

        fs::create_dir_all(&new_path).unwrap();

        if let Ok(bytes_copied) = fs::copy(entry.path(), &new_file_path) {
            if bytes_copied > 0 {
                fs::remove_file(entry.path()).unwrap();
                println!("{} -> {}", entry.path().to_str().unwrap(), &new_file_path);
            }
        }
    }
}

fn detect_date(entry: &DirEntry) -> Result<OutputFolder> {
    let date = detect_date_from_exif(&entry)?;
    Ok(date)
}

fn detect_date_from_exif(entry: &DirEntry) -> Result<OutputFolder> {
    let path = entry.path();
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;

    if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
        match field.value {
            Value::Ascii(ref vec) if !vec.is_empty() => {
                if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                    let output_folder =
                        OutputFolder::new(datetime.year, datetime.month, datetime.day);

                    return Ok(output_folder);
                }

                return Err(anyhow!("DateTime is not valid."));
            }
            _ => return Err(anyhow!("No DateTime field not ASCII")),
        }
    }

    Err(anyhow!("No DateTime field found"))
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    let output_folder = args.output.clone();

    WalkDir::new(args.input)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| move_file(x, &output_folder));
}
