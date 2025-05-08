use super::{consult_option::ConsultOption, consult_result::ConsultResult, dictionary::Dictionary};
use rayon::prelude::*;

#[derive(Debug)]
pub struct Library {
    dicts: Vec<Dictionary>,
}

impl Library {
    pub fn new(dict_dir: &str) -> Self {
        let mut dicts = Vec::new();

        if let Ok(read_dir) = std::fs::read_dir(dict_dir) {
            for entry in read_dir {
                if let Ok(ent) = entry {
                    let path = ent.path();
                    if !path.is_dir() {
                        continue;
                    }
                    if let Ok(dict) = Dictionary::new(path) {
                        dicts.push(dict);
                    }
                }
            }
        }

        Self { dicts }
    }

    pub fn dict_count(&self) -> usize {
        self.dicts.len()
    }

    pub fn consult(&self, word: &str, option: &ConsultOption) -> Vec<ConsultResult> {
        self.dicts
            .par_iter()
            .flat_map_iter(|dict| dict.consult(word, &option))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::stardict::{consult_option::ConsultOption, library::Library, util};

    #[test]
    fn look_up_the_word() {
        let dicts = Library::new(util::get_stardict_dir().unwrap().to_str().unwrap());
        let option = ConsultOption {
            fuzzy: true,
            max_dist: 3,
        };

        let consult_english = dicts.consult("searches", &option);
        assert!(!consult_english.is_empty());
        println!("{:?}", &consult_english);

        let consult_chinese = dicts.consult("搜索", &option);
        assert!(!consult_chinese.is_empty());
        println!("{:?}", &consult_chinese);
    }
}
