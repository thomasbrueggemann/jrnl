use anyhow::Result;
use std::fs::File;

use crate::output_folder::OutputFolder;

pub trait Strategy {
    fn derive_output_folder(&self, file: &File) -> Result<OutputFolder>;
}
