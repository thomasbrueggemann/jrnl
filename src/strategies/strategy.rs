use anyhow::Result;
use std::path::Path;

use crate::output_folder::OutputFolder;

pub trait Strategy {
    fn derive_output_folder(&self, path: &Path) -> Result<OutputFolder>;
}
