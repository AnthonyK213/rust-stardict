use directories::BaseDirs;
use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(unused)]
pub(crate) fn get_stardict_dir() -> Option<PathBuf> {
    if let Some(base_dirs) = BaseDirs::new() {
        let mut home = base_dirs.home_dir().to_path_buf();
        home.push(".stardict");
        home.push("dic");
        if home.is_dir() {
            return Some(home);
        }
    }
    None
}
