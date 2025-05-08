use super::{consult_option::ConsultOption, sd_error::SdError};
use edit_distance;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

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
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut word_buffer = vec![];

        loop {
            let word_bytes = reader.read_until(0, &mut word_buffer)?;
            if let Some(b'\0') = word_buffer.last() {
                word_buffer.pop();
            }
            if word_bytes == 0 {
                break;
            }

            let word = String::from_utf8(word_buffer.clone())?;
            word_buffer.clear();

            self.items.push(IndexItem {
                word,
                offset: read_u32(&mut reader)?,
                length: read_u32(&mut reader)?,
            });
        }

        Ok(())
    }

    pub(crate) fn consult_exact(&self, word: &str) -> Vec<&IndexItem> {
        if let Ok(i) = self
            .items
            .binary_search_by(|item| item.word.as_str().cmp(word))
        {
            vec![&self.items[i]]
        } else {
            vec![]
        }
    }

    pub(crate) fn consult_fuzzy(&self, word: &str, option: &ConsultOption) -> Vec<&IndexItem> {
        let lower = word.to_lowercase();
        let max_dist = option.max_dist as usize;

        self.items
            .iter()
            .filter_map(|item| {
                let item_word = &item.word.to_lowercase();

                if edit_distance::edit_distance(&lower, item_word) <= max_dist {
                    Some(item)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn read_u32<R>(reader: &mut BufReader<R>) -> Result<u32, SdError>
where
    R: Read,
{
    let mut bytes = [0; 4];
    reader.read_exact(&mut bytes)?;
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
