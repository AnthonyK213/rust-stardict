use crate::idx::IdxItem;
use anyhow::Result;
use std::{fs, path::Path, io::Read};
use flate2::read::GzDecoder;

pub struct Dict {
    content: String,
}

impl Dict {
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
