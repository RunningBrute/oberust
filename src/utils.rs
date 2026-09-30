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

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write, path::Path};

    use crate::FileReader;

    #[test]
    fn file_reader_returns_empty_content_if_input_file_is_empty() {
        let path_to_empty_file = Path::new("empty_file.ob2");
        let mut empty_file = File::create(path_to_empty_file).unwrap();
        empty_file.write(String::from("").as_bytes()).unwrap();

        let file_reader = FileReader::new(path_to_empty_file).unwrap();

        assert_eq!(file_reader.content, "");

        std::fs::remove_file(path_to_empty_file).unwrap();
    }
}
