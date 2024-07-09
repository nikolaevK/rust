use std::{env, fs::File, io::{BufRead, BufReader}};
mod config;
use crate::config::Configuration;

fn read_file(file_name: &str) -> Result<Vec<String>, ()> {
    if let Ok(file) = File::open(file_name) {
        let reader = BufReader::new(file);
        Ok(reader
            .lines()
            .map(|line| match line {
                Ok(s) => s,
                Err(_) => String::from("")
            })
            .collect()
        )
    } else {
        Err(())
    }
}

fn is_code_line(line: &str) -> bool {
    let line = line.trim();
    if line.len() < 4 || line.starts_with("//") {
        return false;
    }
    true
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let config = Configuration::new(args[1..].into());

    let mut total_count = 0_usize;
    for file in config.file_names {
        let file_content = read_file(&file).expect(&format!("Couldn't read file {}", file));
        let local_count = file_content
            .iter()
            .filter(|line| is_code_line(line))
            .count();

        total_count += local_count;
        println!("Local Total: {} in {}", local_count, file)
    }
    println!("Total: {}", total_count);
}
