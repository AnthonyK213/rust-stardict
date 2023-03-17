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
        assert_eq!(
            stardict(
                get_stardict_dir().unwrap().to_str().unwrap().to_string(),
                "搜索".into()
            ),
            "*[sә:tʃ]
n. 搜寻, 查究
vt. 搜寻, 搜查, 探求, 调查, 搜索
vi. 搜寻, 搜查, 探求, 调查, 搜索
【计】 搜索, 路径检索程序
【经】 搜索, 检索, 研究
相关词组:
  in search of"
        );
    }
}
