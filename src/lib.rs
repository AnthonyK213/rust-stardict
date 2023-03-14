mod dict;
mod dictionary;
mod idx;
mod ifo;
mod util;

use dictionary::Dictionary;

pub fn stardict(dict_dir: String, word: String) -> Result<String, String> {
    Dictionary::from_dir(dict_dir)
        .map_err(|e| e.to_string())?
        .search(word)
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_up_word() {
        assert_eq!(
            stardict("test/stardict-langdao-ec-gb-2.4.2".into(), "search".into()).unwrap(),
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
