use super::util;
use anyhow::Result;
use regex::Regex;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DictInfo {
    pub version: String,
    pub wordcount: u32,
    pub idxfilesize: u32,
    pub bookname: String,
    pub author: String,
    pub description: String,
    pub date: String,
    pub sametypesequence: String,
}

impl Default for DictInfo {
    fn default() -> Self {
        Self {
            version: "unknown".into(),
            wordcount: 0,
            idxfilesize: 0,
            bookname: "unknown".into(),
            author: "unknown".into(),
            description: "unknown".into(),
            date: "unknown".into(),
            sametypesequence: "unknown".into(),
        }
    }
}

impl DictInfo {
    pub fn read_from_file<P: AsRef<Path>>(&mut self, filename: P) -> Result<()> {
        let re = Regex::new(r"^(\w+)=(.+)$")?;
        let lines = util::read_lines(filename)?;

        for line in lines {
            if let Ok(l) = line {
                if let Some(c) = re.captures(&l) {
                    match &c[1] {
                        "version" => self.version = c[2].parse()?,
                        "wordcount" => self.wordcount = c[2].parse()?,
                        "idxfilesize" => self.idxfilesize = c[2].parse()?,
                        "bookname" => self.bookname = c[2].parse()?,
                        "author" => self.author = c[2].parse()?,
                        "description" => self.description = c[2].parse()?,
                        "date" => self.date = c[2].parse()?,
                        "sametypesequence" => self.sametypesequence = c[2].parse()?,
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::stardict::{dict_info::DictInfo, util};

    #[test]
    fn read_ifo() {
        let mut ifo_path = util::get_stardict_dir().unwrap();
        ifo_path.push("stardict-langdao-ec-gb-2.4.2");
        ifo_path.push("langdao-ec-gb.ifo");
        let mut ifo = DictInfo::default();
        ifo.read_from_file(ifo_path).unwrap();
        assert_eq!(
            ifo,
            DictInfo {
                version: "2.4.2".into(),
                wordcount: 435468,
                idxfilesize: 10651674,
                bookname: "朗道英汉字典5.0".into(),
                author: "上海朗道电脑科技发展有限公司".into(),
                description: "罗小辉破解文件格式，胡正制作转换程序。".into(),
                date: "2003.08.26".into(),
                sametypesequence: "m".into(),
            }
        )
    }
}
