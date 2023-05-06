use anyhow::{anyhow, Result};
use exif::{DateTime, In, Tag, Value};

use std::io::BufReader;
use std::path::Path;

use crate::output_folder::OutputFolder;

use super::strategy::Strategy;

pub struct ExifStrategy {}

impl Strategy for ExifStrategy {
    fn derive_output_folder(&self, path: &Path) -> Result<OutputFolder> {
        let file = std::fs::File::open(path)?;
        let mut bufreader = BufReader::new(file);
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
}
