use super::sd_error::SdError;
use edit_distance;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct IndexItem {
    pub word: String,
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug, Default)]
pub(crate) struct DictIndex {
    items: Vec<IndexItem>,
}

impl DictIndex {
    pub fn read_from_file<P: AsRef<Path>>(&mut self, filename: P) -> Result<(), SdError> {
        let content = std::fs::read(filename)?;
        let mut word_buffer = Vec::new();
        let mut content_iter = content.iter();

        while let Some(&byte) = content_iter.next() {
            if byte == 0 {
                self.items.push(IndexItem {
                    word: String::from_utf8(std::mem::take(&mut word_buffer))?,
                    offset: take_u32(&mut content_iter)?,
                    length: take_u32(&mut content_iter)?,
                });
            } else {
                word_buffer.push(byte);
            }
        }

        Ok(())
    }

    pub(crate) fn consult(&self, word: &str) -> Vec<&IndexItem> {
        let mut result = Vec::<&IndexItem>::new();
        let lower = word.to_lowercase();
        let t_len = lower.chars().count() as isize;
        let max_dist = 3.min(t_len);

        for item in &self.items {
            let c_len = item.word.chars().count() as isize;
            if (t_len - c_len).abs() >= 3 {
                continue;
            }
            let a = &item.word.to_lowercase();
            if *a == lower {
                return vec![item];
            }
            if edit_distance::edit_distance(&lower, a) < max_dist.min(c_len) as usize {
                result.push(item);
            }
        }

        result
    }
}

fn take_u32(content_iter: &mut std::slice::Iter<'_, u8>) -> Result<u32, SdError> {
    let mut bytes: [u8; 4] = [0; 4];
    let mut iter = content_iter.by_ref().take(4);

    for i in 0..4 {
        if let Some(&byte) = iter.next() {
            bytes[i] = byte;
        } else {
            return Err(SdError::ParseIndexError);
        }
    }

    Ok(u32::from_be_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use crate::stardict::{dict_index::*, util};

    #[test]
    fn read_idx() {
        let mut idx_path = util::get_stardict_dir().unwrap();
        idx_path.push("stardict-langdao-ec-gb-2.4.2");
        idx_path.push("langdao-ec-gb.idx");

        let mut idx = DictIndex::default();
        idx.read_from_file(idx_path).unwrap();

        assert_eq!(
            idx.items[0],
            IndexItem {
                word: "a".to_string(),
                offset: 0,
                length: 132,
            }
        )
    }
}
