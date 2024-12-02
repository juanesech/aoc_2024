use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};

pub fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}
