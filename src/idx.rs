use crate::util::get_u32;
use anyhow::Result;
use edit_distance;
use std::{fs, path::Path};

enum Token {
    Null,
    Word,
    Pos,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct IdxItem {
    pub word: String,
    pub offset: u32,
    pub length: u32,
}

pub(crate) struct Idx {
    items: Vec<IdxItem>,
}

impl Idx {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn read_from_file<P: AsRef<Path>>(&mut self, filename: P) -> Result<()> {
        let content = fs::read(filename)?;
        let mut word = Vec::<u8>::new();
        let mut pos = [0; 8];
        let mut token = Token::Word;
        let mut count = 7;

        for c in content {
            match token {
                Token::Null => {
                    pos[0] = c;
                    token = Token::Pos;
                    count = 7;
                }
                Token::Word => {
                    if c == 0 {
                        token = Token::Null;
                    } else {
                        word.push(c);
                    }
                }
                Token::Pos => {
                    count -= 1;
                    pos[7 - count] = c;
                    if count == 0 {
                        let (o, l) = get_u32(&pos)?;
                        self.items.push(IdxItem {
                            word: String::from_utf8(word.clone())?,
                            offset: o,
                            length: l,
                        });
                        token = Token::Word;
                        word.clear();
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) fn index(&self, word: &String) -> Vec<&IdxItem> {
        let mut result = Vec::<&IdxItem>::new();
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

#[cfg(test)]
mod tests {
    use crate::{idx::*, util::get_stardict_dir};

    #[test]
    fn read_idx() {
        let mut idx = Idx::new();
        let mut idx_path = get_stardict_dir().unwrap();
        idx_path.push("stardict-langdao-ec-gb-2.4.2");
        idx_path.push("langdao-ec-gb.idx");
        idx.read_from_file(idx_path).unwrap();
        assert_eq!(
            idx.items[0],
            IdxItem {
                word: "a".to_string(),
                offset: 0,
                length: 132,
            }
        )
    }
}
