pub struct OutputFolder {
    year: u16,
    month: u8,
    day: u8,
}

impl OutputFolder {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn get_path(&self) -> String {
        format!(
            "{}/{}/{}",
            self.year,
            format!("{:0>2}", self.month),
            format!("{:0>2}", self.day)
        )
    }
}
