pub struct Config {
    input_filename: String,
    output_filename: Option<String>,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {
        let input = args.get(1).expect("Source file needed");

        let output_opt = args.iter().find(|&x| *x == "-o");

        let mut arguments = Config {
            input_filename: input.clone(),
            output_filename: Some(String::from("default_bin_name")),
        };

        match output_opt {
            Some(_) => arguments.output_filename = args.get(3).cloned(),
            _ => {}
        }

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

        let _config = Config::new(&args);
    }

    #[test]
    fn when_source_file_is_provided_then_use_default_binary_name() {
        let args = vec![String::from("home/my_app"), String::from("source_file.Mod")];
        assert_eq!(args.len(), 2);

        let config = Config::new(&args).unwrap();
        assert_eq!(config.input_filename, String::from("source_file.Mod"));
        assert_eq!(
            config.output_filename,
            Some(String::from("default_bin_name"))
        );
    }

    #[test]
    #[should_panic(expected="")]
    fn when_binary_name_is_requested_but_not_provied_then_panic() {
        let args = vec![
            String::from("home/my_app"),
            String::from("source_file.Mod"),
            String::from("-o")];
        let _config = Config::new(&args).unwrap();
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

        let config = Config::new(&args).unwrap();
        assert_eq!(config.input_filename, String::from("source_file.Mod"));
        assert_eq!(config.output_filename, Some(String::from("my_binary")));
    }
}
