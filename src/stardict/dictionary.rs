use super::{
    consult_result::ConsultResult, dict_content::DictContent, dict_index::DictIndex,
    dict_info::DictInfo,
};
use std::path::Path;
use super::sd_error::SdError;

#[derive(Debug)]
pub(crate) struct Dictionary {
    content: DictContent,
    index: DictIndex,
    info: DictInfo,
}

impl Dictionary {
    pub fn new<P: AsRef<Path>>(dir: P) -> Result<Self, SdError> {
        let mut content = DictContent::default();
        let mut index = DictIndex::default();
        let mut info = DictInfo::default();

        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();

            if !path.is_file() {
                continue;
            }

            if let Some(ext) = path.extension() {
                match ext.to_str() {
                    Some("dz") => {
                        content.read_from_file(path)?;
                    }
                    Some("idx") => {
                        index.read_from_file(path)?;
                    }
                    Some("ifo") => {
                        info.read_from_file(path)?;
                    }
                    _ => continue,
                }
            }
        }

        Ok(Dictionary {
            content,
            index,
            info,
        })
    }

    pub fn consult(&self, word: &str) -> Vec<ConsultResult> {
        self.index
            .consult(word)
            .iter()
            .map(|item| {
                let def = self.content.get(item);
                ConsultResult {
                    dict: &self.info.bookname,
                    word: &item.word,
                    definition: &def,
                }
            })
            .collect()
    }
}
