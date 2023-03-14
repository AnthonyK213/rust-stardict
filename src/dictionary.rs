use crate::{dict, idx, ifo};
use anyhow::{anyhow, Result};
use std::{fs, path::Path};

use dict::Dict;
use idx::Idx;
use ifo::Ifo;

pub struct Dictionary {
    idx: Idx,
    dict: Dict,
    ifo: Ifo,
}

impl Dictionary {
    pub fn from_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let mut idx: Option<Idx> = None;
        let mut dict: Option<Dict> = None;
        let mut ifo: Option<Ifo> = None;
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    match ext.to_str() {
                        Some("dz") => {
                            dict = Dict::read_from_file(path).ok();
                        }
                        Some("idx") => {
                            let mut _idx = Idx::new();
                            _idx.read_from_file(path)?;
                            idx = Some(_idx);
                        }
                        Some("ifo") => {
                            ifo = Ifo::read_from_file(path).ok();
                        }
                        _ => {}
                    }
                }
            }
        }
        if !(idx.is_none() || dict.is_none() || ifo.is_none()) {
            Ok(Dictionary {
                dict: dict.unwrap(),
                idx: idx.unwrap(),
                ifo: ifo.unwrap(),
            })
        } else {
            Err(anyhow!("Failed to get the `Dictionary`"))
        }
    }

    pub fn search(&self, word: String) -> Result<String> {
        if let Some(item) = self.idx.index(&word) {
            Ok(self.dict.get(item))
        } else {
            Err(anyhow!("Nothing found"))
        }
    }
}
