mod dict;
mod dictionary;
mod idx;
mod ifo;
mod util;

pub fn look_up(dict_dir: String, word: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use dict::Dict;
    use dictionary::Dictionary;
    use idx::Idx;
    use ifo::Ifo;

    #[test]
    fn look_up_word() {
        let mut idx = Idx::new();
        idx.read_from_file("test/stardict-langdao-ec-gb-2.4.2/langdao-ec-gb.idx")
            .unwrap();
        let dict =
            Dict::read_from_file("test/stardict-langdao-ec-gb-2.4.2/langdao-ec-gb.dict").unwrap();
        let ifo =
            Ifo::read_from_file("test/stardict-langdao-ec-gb-2.4.2/langdao-ec-gb.ifo").unwrap();

        let d = Dictionary::new(dict, idx, ifo);

        assert_eq!(
            d.search("search".into()).unwrap(),
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
