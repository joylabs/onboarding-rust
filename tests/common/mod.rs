use std;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn slurp(f: &str) -> std::io::Result<String> {
    let mut file = File::open(f)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[allow(dead_code)]
pub fn slurp_lines(f: &str) -> Vec<String> {
    let file = File::open(f).expect("couldn't find file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("can't parse file")).collect()
}
