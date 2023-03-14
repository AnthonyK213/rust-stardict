use crate::idx::IdxItem;
use anyhow::Result;
use std::{fs, path::Path};

pub struct Dict {
    content: String,
}

impl Dict {
    pub fn read_from_file<P: AsRef<Path>>(filepath: P) -> Result<Self> {
        let content = fs::read_to_string(filepath)?;
        Ok(Self { content })
    }

    pub fn get(&self, item: &IdxItem) -> String {
        self.content[(item.offset as usize)..((item.offset + item.length) as usize)].to_string()
    }
}
