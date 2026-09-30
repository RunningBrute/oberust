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