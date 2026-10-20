mod tests {
    use oberust::lexer::tokenize;

    #[test]
    fn tokenization_of_simple_program() {
        let content = String::from(
            "MODULE Empty; \n BEGIN \n END Empty."
        );

        let tokens = tokenize(&content);

        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0..3], ["MODULE", "Empty", ";"]);
        assert_eq!(tokens[3], "BEGIN");
        assert_eq!(tokens[4..7], ["END", "Empty", "."]);
    }

    #[test]
    fn tokenization_of_more_advanced_program() {
        let content = String::from(
            "MODULE Variables;\nVAR\nx, y: INTEGER;\nBEGIN\nx := 10;\ny := x + 20;\nEND Variables.");

        let tokens = tokenize(&content);

        assert_eq!(tokens[0..3], ["MODULE", "Variables", ";"]);
        assert_eq!(tokens[3], "VAR");
        assert_eq!(tokens[4..10], ["x", ",", "y", ":", "INTEGER", ";"]);
        assert_eq!(tokens[10], "BEGIN");
        assert_eq!(tokens[11..15], ["x", ":=", "10", ";"]);
        assert_eq!(tokens[15..21], ["y", ":=", "x", "+", "20", ";"]);
        assert_eq!(tokens[21..24], ["END", "Variables", "."]);

        assert_eq!(tokens.len(), 24);
    }
}
