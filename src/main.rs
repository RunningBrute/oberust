use std::path::Path;

use crate::{configuration::Config, utils::FileReader};

mod configuration;
mod utils;

fn main() {
    let mut args = std::env::args();
    let config = Config::new(&mut args).unwrap_or_else(|e: String| panic!("{}", e));
    let file_reader = FileReader::new(Path::new(&config.input_filename)).unwrap();

    println!("File content: {:?}", file_reader.content);
}
