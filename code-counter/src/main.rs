use std::env;

mod config;
use crate::config::Configuration;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let _config = Configuration::new(args[1..].into());
}
