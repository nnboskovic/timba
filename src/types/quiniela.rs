use std::fs;
use std::ffi::OsStr;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};
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

    /// Read data from resources/quiniela.csv and populate a vector for later reading.
    pub fn populate_from_csv() -> Result<Vec<QuinielaNumber>, anyhow::Error> {
        let mut nums: Vec<QuinielaNumber> = vec![];

        let input = fs::read_to_string("./resources/quiniela.csv");
        let binding = input.unwrap();
        let lines = binding.split("\n");

        for line in lines {
            let mut split_line = line.split(", ").collect::<Vec<&str>>();
            let number = split_line[0].to_string();
            let lore = split_line[1].to_string();

            nums.push(QuinielaNumber::new(number, lore));
        }

        Ok(nums)
    }
}
