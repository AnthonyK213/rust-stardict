use super::{consult_result::ConsultResult, dictionary::Dictionary};
use rayon::prelude::*;

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

    pub fn consult(&self, word: &str) -> Vec<ConsultResult> {
        self.dicts
            .par_iter()
            .flat_map_iter(|dict| dict.consult(word))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::stardict::{library::Library, util};

    #[test]
    fn look_up_the_word() {
        let dicts = Library::new(util::get_stardict_dir().unwrap().to_str().unwrap());
        println!("{:?}", dicts.consult("searches"));
        println!("{:?}", dicts.consult("搜索"));
    }
}
