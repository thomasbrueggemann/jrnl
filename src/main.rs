use anyhow::{anyhow, Result};
use clap::Parser;
use exif::{DateTime, In, Tag, Value};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input: String,

    #[clap(short, long)]
    ouput: String,
}

struct OutputFolder {
    year: String,
    month: String,
    day: String,
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn move_file(entry: DirEntry) {
    detect_date(&entry);
    println!("{}", entry.path().display());
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
                    let output_folder = OutputFolder {
                        year: datetime.year.to_string(),
                        month: datetime.month.to_string(),
                        day: datetime.day.to_string(),
                    };

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

    WalkDir::new(args.input)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| move_file(x));
}
