use oberust::configuration::Config;

mod tests {
    use oberust::lexer::tokenize;

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

    #[test]
    fn tokenization_of_simple_program() {
        let content = String::from(
            "MODULE Empty; \n BEGIN \n END Empty."
        );

        let tokens = tokenize(&content);

        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0], "MODULE");
        assert_eq!(tokens[1], "Empty");
        assert_eq!(tokens[2], ";");
        assert_eq!(tokens[3], "BEGIN");
        assert_eq!(tokens[4], "END");
        assert_eq!(tokens[5], "Empty");
        assert_eq!(tokens[6], ".");
    }
}
