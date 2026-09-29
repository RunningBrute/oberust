pub struct Config {
    input_filename: String,
    output_filename: Option<String>,
}

impl Config {
    pub fn new<T: Iterator<Item = String>>(args: &mut T) -> Result<Self, String> {
        let mut arguments = Config {
            input_filename: String::from(""),
            output_filename: None,
        };

        // program name should be skipped
        let mut args = args.skip(1);
        
        // first will be source file name
        arguments.input_filename = match args.next() {
            Some(name) => name,
            None => panic!("Source file needed"),
        };

        // then after -o option could be name of the output binary
        if args.next() == Some(String::from("-o")) {
            arguments.output_filename = match args.next() {
                Some(name) => Some(name),
                None => panic!("Binary name missing"),
            };
        };

        Ok(arguments)
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::Config;

    #[test]
    #[should_panic(expected = "Source file needed")]
    fn when_source_file_is_not_provided_then_panic() {
        let args = vec![String::from("home/my_app")];
        assert_eq!(args.len(), 1);

        let _config = Config::new(&mut args.into_iter());
    }

    #[test]
    fn when_source_file_is_provided_then_use_default_binary_name() {
        let args = vec![String::from("home/my_app"), String::from("source_file.Mod")];
        assert_eq!(args.len(), 2);

        let config = Config::new(&mut args.into_iter()).unwrap();
        assert_eq!(config.input_filename, String::from("source_file.Mod"));
        assert_eq!(
            config.output_filename,
            None
        );
    }

    #[test]
    #[should_panic(expected="")]
    fn when_binary_name_is_requested_but_not_provied_then_panic() {
        let args = vec![
            String::from("home/my_app"),
            String::from("source_file.Mod"),
            String::from("-o")];
        let _config = Config::new(&mut args.into_iter()).unwrap();
    }

    #[test]
    fn when_binary_name_and_source_file_is_provided_then_config_is_complete() {
        let args = vec![
            String::from("home/my_app"),
            String::from("source_file.Mod"),
            String::from("-o"),
            String::from("my_binary"),
        ];
        assert_eq!(args.len(), 4);

        let config = Config::new(&mut args.into_iter()).unwrap();
        assert_eq!(config.input_filename, String::from("source_file.Mod"));
        assert_eq!(config.output_filename, Some(String::from("my_binary")));
    }
}
