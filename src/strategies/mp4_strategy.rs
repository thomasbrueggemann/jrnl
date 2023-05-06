use anyhow::{anyhow, Result};
use chrono::prelude::*;
use std::{
    io::BufReader,
    path::Path,
    time::{Duration, UNIX_EPOCH},
};

use crate::output_folder::OutputFolder;

use super::strategy::Strategy;

pub struct Mp4Strategy {}

impl Strategy for Mp4Strategy {
    fn derive_output_folder(&self, path: &Path) -> Result<OutputFolder> {
        let file = std::fs::File::open(path)?;
        let size = file.metadata()?.len();
        let reader = BufReader::new(file);

        match mp4::Mp4Reader::read_header(reader, size) {
            Ok(mp4) => {
                let created_timestamp = creation_time(mp4.moov.mvhd.creation_time);
                let d = UNIX_EPOCH + Duration::from_secs(created_timestamp);
                let created_datetime = DateTime::<Utc>::from(d);

                Ok(OutputFolder::new(
                    created_datetime.year() as u16,
                    created_datetime.month() as u8,
                    created_datetime.day() as u8,
                ))
            }
            Err(e) => Err(anyhow!("Unable to extract mp4/mov metadata")),
        }
    }
}

fn creation_time(creation_time: u64) -> u64 {
    println!("{} creation_time", creation_time);

    // convert from MP4 epoch (1904-01-01) to Unix epoch (1970-01-01)
    if creation_time >= 2082844800 {
        creation_time - 2082844800
    } else {
        creation_time
    }
}
