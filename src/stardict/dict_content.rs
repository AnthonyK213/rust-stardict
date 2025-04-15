use super::dict_index::IndexItem;
use anyhow::Result;
use flate2::read::GzDecoder;
use std::{fs, io::Read, path::Path};

#[derive(Default)]
pub(crate) struct DictContent {
    content: String,
}

impl DictContent {
    pub fn read_from_file<P: AsRef<Path>>(&mut self, filepath: P) -> Result<()> {
        let buffer = fs::read(filepath)?;
        let mut dict = GzDecoder::new(buffer.as_slice());
        dict.read_to_string(&mut self.content)?;
        Ok(())
    }

    pub fn get(&self, item: &IndexItem) -> &str {
        &self.content[(item.offset as usize)..((item.offset + item.length) as usize)]
    }
}
