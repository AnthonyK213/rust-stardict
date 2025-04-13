use crate::util::read_lines;
use anyhow::{anyhow, Result};
use regex::Regex;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Ifo {
    version: String,
    wordcount: u32,
    idxfilesize: u32,
    pub bookname: String,
    author: String,
    description: String,
    date: String,
    sametypesequence: String,
}

impl Ifo {
    pub fn read_from_file<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let mut ifo = Self {
            version: "2.4.2".into(),
            wordcount: 0,
            idxfilesize: 0,
            bookname: "".into(),
            author: "".into(),
            description: "".into(),
            date: "".into(),
            sametypesequence: "".into(),
        };
        let re = Regex::new(r"^(\w+)=(.+)$")?;
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(l) = line {
                    if let Some(c) = re.captures(&l) {
                        match &c[1] {
                            "version" => ifo.version = c[2].parse()?,
                            "wordcount" => ifo.wordcount = c[2].parse()?,
                            "idxfilesize" => ifo.idxfilesize = c[2].parse()?,
                            "bookname" => ifo.bookname = c[2].parse()?,
                            "author" => ifo.author = c[2].parse()?,
                            "description" => ifo.description = c[2].parse()?,
                            "date" => ifo.date = c[2].parse()?,
                            "sametypesequence" => ifo.sametypesequence = c[2].parse()?,
                            _ => {}
                        }
                    }
                }
            }
            Ok(ifo)
        } else {
            Err(anyhow!("Invalid lines!"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ifo::Ifo, util::get_stardict_dir};

    #[test]
    fn read_ifo() {
        let mut ifo_path = get_stardict_dir().unwrap();
        ifo_path.push("stardict-langdao-ec-gb-2.4.2");
        ifo_path.push("langdao-ec-gb.ifo");
        let ifo = Ifo::read_from_file(ifo_path).unwrap();
        assert_eq!(
            ifo,
            Ifo {
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
