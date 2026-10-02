pub fn tokenize(content: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut token_begin = 0;
    let mut token_end = 0;
    
    for (i, c) in content.char_indices() {
        if c.is_whitespace() {
            tokens.push(&content[token_begin..token_end]);
            token_begin = i;
            //token_end = i + 1;
        }
        if c == ';' {
            tokens.push(&content[i..i+1]);
            token_begin = i;
            //token_end = i + 1;
        }
        token_end = i;
    }

    tokens
}
