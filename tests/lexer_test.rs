mod tests {
    use oberust::lexer::tokenize;

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
