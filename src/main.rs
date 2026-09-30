use std::path::Path;

use crate::{configuration::Config, utils::FileReader};

mod configuration;
mod utils {
    use std::{fs::File, io::Read, path::Path};
    pub struct FileReader {
        pub content: String,
    }

    impl FileReader {
        pub fn new(file_path: &Path) -> Result<Self, std::io::Error> {
            let file_name = file_path.display();
            println!("Open {:?} file.", file_name);

            let mut file_content = String::new();
            File::open(&file_path)?.read_to_string(&mut file_content)?;

            Ok(Self {
                content: file_content,
            })
        }
    }
}

fn main() {
    let mut args = std::env::args();
    let config = Config::new(&mut args).unwrap_or_else(|e: String| panic!("{}", e));
    let file_reader = FileReader::new(Path::new(&config.input_filename)).unwrap();

    println!("File content: {:?}", file_reader.content);
}
