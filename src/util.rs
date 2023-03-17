use anyhow::{anyhow, Result};
use directories::BaseDirs;
use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_u32(vec: &[u8]) -> Result<(u32, u32)> {
    if vec.len() == 8 {
        Ok((
            ((vec[0] as u32) << 24)
                + ((vec[1] as u32) << 16)
                + ((vec[2] as u32) << 8)
                + (vec[3] as u32),
            ((vec[4] as u32) << 24)
                + ((vec[5] as u32) << 16)
                + ((vec[6] as u32) << 8)
                + (vec[7] as u32),
        ))
    } else {
        Err(anyhow!("8!"))
    }
}

pub fn get_stardict_dir() -> Option<PathBuf> {
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
