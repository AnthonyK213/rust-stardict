use crate::util::get_u32;
use anyhow::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::{fs, path::Path};

enum Token {
    Null,
    Word,
    Pos,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IdxItem {
    pub word: String,
    pub offset: u32,
    pub length: u32,
}

pub struct Idx {
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

    pub fn index(&self, word: &String) -> Vec<&IdxItem> {
        let mut result = Vec::<&IdxItem>::new();
        let matcher: SkimMatcherV2 = SkimMatcherV2::default();
        let space = word.contains(" ");
        let lower = word.to_lowercase();
        let pass: i64 = (0.62
            * (matcher
                .fuzzy_match(&lower, &lower)
                .unwrap() as f32))
            .floor() as i64;
        for item in &self.items {
            let mut a = &item.word.to_lowercase();
            if *a == lower {
                return vec![item];
            }
            let mut b = &lower;
            if a.len() < b.len() {
                (a, b) = (b, a);
            }
            if let Some(score) = matcher.fuzzy_match(a, b) {
                if (space || !item.word.contains(" ")) && score >= pass {
                    result.push(item)
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::idx::*;

    #[test]
    fn read_idx() {
        let mut idx = Idx::new();
        idx.read_from_file("test/stardict-langdao-ec-gb-2.4.2/langdao-ec-gb.idx")
            .unwrap();
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
