use super::sd_error::SdError;
use super::util;
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
            version: "2.4.2".into(),
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
    pub fn read_from_file<P: AsRef<Path>>(&mut self, filename: P) -> Result<(), SdError> {
        let re = Regex::new(r"^(\w+)=(.+)$")?;
        let lines = util::read_lines(filename)?;

        for line in lines {
            if let Ok(l) = line {
                if let Some(c) = re.captures(&l) {
                    self.set_field(&c[1], &c[2])?;
                }
            }
        }

        Ok(())
    }

    fn set_field(&mut self, field: &str, value: &str) -> Result<(), SdError> {
        match field {
            "version" => {
                self.version = value
                    .parse()
                    .map_err(|_| SdError::ParseInfoError("version"))?
            }
            "wordcount" => {
                self.wordcount = value
                    .parse()
                    .map_err(|_| SdError::ParseInfoError("wordcount"))?
            }
            "idxfilesize" => {
                self.idxfilesize = value
                    .parse()
                    .map_err(|_| SdError::ParseInfoError("idxfilesize"))?
            }
            "bookname" => {
                self.bookname = value
                    .parse()
                    .map_err(|_| SdError::ParseInfoError("bookname"))?
            }
            "author" => {
                self.author = value
                    .parse()
                    .map_err(|_| SdError::ParseInfoError("author"))?
            }
            "description" => {
                self.description = value
                    .parse()
                    .map_err(|_| SdError::ParseInfoError("description"))?
            }
            "date" => self.date = value.parse().map_err(|_| SdError::ParseInfoError("date"))?,
            "sametypesequence" => {
                self.sametypesequence = value
                    .parse()
                    .map_err(|_| SdError::ParseInfoError("sametypesequence"))?
            }
            _ => {}
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
