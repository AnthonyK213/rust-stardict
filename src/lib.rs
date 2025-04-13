mod dict;
mod dictionary;
mod idx;
mod ifo;
mod util;

use dictionary::Dictionary;
use rayon::prelude::*;
use std::fs;

pub struct Dictionaries {
    dicts: Vec<Dictionary>,
}

impl Dictionaries {
    pub fn new(dict_dir: &str) -> Self {
        let mut dicts = Vec::new();
        if let Ok(read_dir) = fs::read_dir(dict_dir) {
            for entry in read_dir {
                if let Ok(ent) = entry {
                    let path = ent.path();
                    if !path.is_dir() {
                        continue;
                    }
                    if let Ok(dict) = Dictionary::from_dir(path) {
                        dicts.push(dict);
                    }
                }
            }
        }

        Self { dicts }
    }

    pub fn search_word_into_json(&self, word: &str) -> String {
        format!(
            "[{}]",
            self.dicts
                .par_iter()
                .flat_map_iter(|dict| { dict.search(word) })
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::*;

    #[test]
    fn look_up_word() {
        let dicts = Dictionaries::new(get_stardict_dir().unwrap().to_str().unwrap());
        println!("{}", dicts.search_word_into_json("searches"));
    }
}
