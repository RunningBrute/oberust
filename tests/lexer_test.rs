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

    #[test]
    fn tokenization_of_more_advanced_program() {
        let content = String::from(
            "MODULE Variables;\nVAR\nx, y: INTEGER;\nBEGIN\nx := 10;\ny := x + 20;\nEND Variables.");

        let tokens = tokenize(&content);

        assert_eq!(tokens.len(), 24);
        assert_eq!(tokens[0], "MODULE");
        assert_eq!(tokens[1], "Variables");
        assert_eq!(tokens[2], ";");
        assert_eq!(tokens[3], "VAR");
        assert_eq!(tokens[4], "x");
        assert_eq!(tokens[5], ",");
        assert_eq!(tokens[6], "y");
        assert_eq!(tokens[7], ":");
        assert_eq!(tokens[8], "INTEGER");
        assert_eq!(tokens[9], ";");
        assert_eq!(tokens[10], "BEGIN");
        assert_eq!(tokens[11], "x");
        assert_eq!(tokens[12], ":=");
        assert_eq!(tokens[13], "10");
        assert_eq!(tokens[14], ";");
        assert_eq!(tokens[15], "y");
        assert_eq!(tokens[16], ":=");
        assert_eq!(tokens[17], "x");
        assert_eq!(tokens[18], "+");
        assert_eq!(tokens[19], "20");
        assert_eq!(tokens[20], ";");
        assert_eq!(tokens[21], "END");
        assert_eq!(tokens[22], "Variables");
        assert_eq!(tokens[23], ".");
    }
}
