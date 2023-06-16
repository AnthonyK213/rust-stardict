mod dict;
mod dictionary;
mod idx;
mod ifo;
mod util;

use dictionary::Dictionary;
use rayon::prelude::*;
use std::{fs, path::PathBuf};

pub fn stardict(dict_dir: String, word: String) -> String {
    let mut dict_dirs = Vec::<PathBuf>::new();
    if let Ok(read_dir) = fs::read_dir(dict_dir) {
        for entry in read_dir {
            if let Ok(ent) = entry {
                let path = ent.path();
                if path.is_dir() {
                    dict_dirs.push(path);
                }
            }
        }
    }

    format!(
        "[{}]",
        dict_dirs
            .par_iter()
            .flat_map_iter(|path| {
                if let Ok(dict) = Dictionary::from_dir(path) {
                    dict.search(&word)
                } else {
                    Vec::new()
                }
            })
            .collect::<Vec<String>>()
            .join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::*;

    #[test]
    fn look_up_word() {
        println!(
            "{}",
            stardict(
                get_stardict_dir().unwrap().to_str().unwrap().to_string(),
                "searches".into()
            )
        );
    }
}
