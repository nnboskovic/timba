use std::fs;
use std::ffi::OsStr;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};
use std::os::unix::fs::DirEntryExt2;
use std::str::Split;

#[derive(Clone, Debug)]
pub struct QuinielaNumber {
    pub(crate) number: String,
    pub(crate) lore: String,
}

impl QuinielaNumber {
    pub fn new(number: String, lore: String) -> QuinielaNumber {
        QuinielaNumber { number, lore }
    }

    pub fn populate_from_csv() -> Result<Vec<QuinielaNumber>, anyhow::Error> {
        let mut nums: Vec<QuinielaNumber> = vec![];
        /*let mut resources_path;

        let paths = fs::read_dir("./resources")?;
        for path in paths {
            if let Some("quiniela.csv") = path.extension().and_then(OsStr::to_str) {
                resources_path = path.unwrap();
            }
        }*/

        let input = File::open(/*path goes here*/)?;
        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            let line = line.unwrap();
            let mut split_line = line.split(",");
            let number = split_line.nth(0).unwrap().to_string();
            let lore = split_line.nth(1).unwrap().to_string();

            nums.push(QuinielaNumber::new(number, lore));
        }

        Ok(nums)
    }
}
