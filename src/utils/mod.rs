use core::panic;
use std::{
    env::current_dir,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn get_file_lines(rel_path: &str) -> Vec<String> {
    let root_dir = match current_dir() {
        Ok(path) => path,
        Err(err) => panic!("{}", err),
    };

    let file = match File::open(Path::join(&root_dir, rel_path)) {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| match line {
            Ok(ln) => ln,
            Err(_) => "".to_string(),
        })
        .collect::<Vec<String>>();

    return lines;
}
