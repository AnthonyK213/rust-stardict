use crate::idx::IdxItem;
use anyhow::Result;
use flate2::read::GzDecoder;
use std::{fs, io::Read, path::Path};

pub(crate) struct DictContent {
    content: String,
}

impl DictContent {
    pub fn read_from_file<P: AsRef<Path>>(filepath: P) -> Result<Self> {
        let buf = fs::read(filepath)?;
        let mut d = GzDecoder::new(buf.as_slice());
        let mut content = String::new();
        d.read_to_string(&mut content)?;
        Ok(Self { content })
    }

    pub fn get(&self, item: &IdxItem) -> String {
        self.content[(item.offset as usize)..((item.offset + item.length) as usize)].to_string()
    }
}
