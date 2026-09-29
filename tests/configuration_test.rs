use oberust::configuration::Config;

mod tests {
    use super::*;

    #[test]
    fn configuration_works() {
        let args = vec![
            String::from("path/to/oberust"),
            String::from("example_program.Mod"),
            String::from("-o"),
            String::from("bin_name"),
        ];
        assert_eq!(args.len(), 4);

        let config = Config::new(&mut args.into_iter()).unwrap();
        assert_eq!(config.input_filename, String::from("example_program.Mod"));
        assert_eq!(config.output_filename, Some(String::from("bin_name")));
    }
}
