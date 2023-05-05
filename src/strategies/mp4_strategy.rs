use anyhow::Result;
use chrono::prelude::*;
use std::{
    fs::File,
    io::BufReader,
    time::{Duration, UNIX_EPOCH},
};

use crate::output_folder::OutputFolder;

use super::strategy::Strategy;

pub struct Mp4Strategy {}

impl Strategy for Mp4Strategy {
    fn derive_output_folder(&self, file: &File) -> Result<OutputFolder> {
        let size = file.metadata()?.len();
        let reader = BufReader::new(file);

        let mp4 = mp4::Mp4Reader::read_header(reader, size)?;

        let created_timestamp = creation_time(mp4.moov.mvhd.creation_time);
        let d = UNIX_EPOCH + Duration::from_secs(created_timestamp);
        let created_datetime = DateTime::<Utc>::from(d);

        Ok(OutputFolder::new(
            created_datetime.year() as u16,
            created_datetime.month() as u8,
            created_datetime.day() as u8,
        ))
    }
}

fn creation_time(creation_time: u64) -> u64 {
    // convert from MP4 epoch (1904-01-01) to Unix epoch (1970-01-01)
    if creation_time >= 2082844800 {
        creation_time - 2082844800
    } else {
        creation_time
    }
}
