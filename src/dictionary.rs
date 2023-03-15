use crate::{dict, idx, ifo};
use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, path::Path};

use dict::Dict;
use idx::Idx;
use ifo::Ifo;

#[derive(Serialize, Deserialize)]
struct Payload {
    definition: String,
    word: String,
    dict: String,
}

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

    pub fn search(&self, word: &String) -> Vec<String> {
        self.idx
            .index(word)
            .iter()
            .map(|item| {
                let def = self.dict.get(item);
                let payload = Payload {
                    definition: def,
                    word: item.word.to_string(),
                    dict: self.ifo.bookname.to_string(),
                };
                serde_json::to_string(&payload).unwrap()
            })
            .collect()
    }
}
