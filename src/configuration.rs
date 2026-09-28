pub struct Config {
    _input_filename: String,
    output_filename: Option<String>,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {
        let input = args.get(1).expect("Source file needed");

        let output_opt = args.iter().find(|&x| *x == "-o");

        let mut arguments = Config {
            _input_filename: input.clone(),
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
    #[should_panic(expected="Source file needed")]
    fn source_file_is_needed()
    {
        let args = vec![String::from("home/some_app")];
        let _config = Config::new(&args);
    }

}