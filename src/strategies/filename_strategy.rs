use anyhow::{anyhow, Result};
use chrono::Datelike;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;

use crate::output_folder::OutputFolder;

use super::strategy::Strategy;

pub struct FilenameStrategy {}

impl Strategy for FilenameStrategy {
    fn derive_output_folder(&self, path: &Path) -> Result<OutputFolder> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"_(\d{4})(\d{2})(\d{2})_").unwrap();
        }

        let current_year = chrono::Utc::now().year();
        let filename = path.file_name().unwrap().to_str().unwrap();

        for cap in RE.captures_iter(filename) {
            let year = cap[1].parse::<u16>()?;
            let month = cap[2].parse::<u8>()?;
            let day = cap[3].parse::<u8>()?;

            if year >= 2004
                && year <= current_year as u16
                && month >= 1
                && month <= 12
                && day >= 1
                && day <= 31
            {
                return Ok(OutputFolder::new(year, month, day));
            }
        }

        return Err(anyhow!("No output folder pattern found"));
    }
}
