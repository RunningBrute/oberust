use std::fmt::Error;

pub fn tokenize(content: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut token_begin = 0;
    let mut token_end = 0;

    for (i, c) in content.char_indices() {
        if c.is_whitespace() {
            tokens.push(&content[token_begin..token_end + 1]);
            token_begin = token_end + 1;
            //token_end = token_end + 1;
        }
        if c == ';' {
            tokens.push(&content[i..i + 1]);
            token_begin = token_end + 1;
            //token_end = i + 1;
        }
        token_end = i;
    }

    tokens
}

pub fn find_beggining_of_next_token(content: &str) -> Option<usize> {
    match content.find(|c: char| c.is_whitespace()) {
        Some(value) => {
            if value < content.len() - 1 {
                return Some(value + 1);
            } else {
                return None;
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::find_beggining_of_next_token;
    use crate::lexer::tokenize;

    #[test]
    pub fn find_text_token_starting_index() {
        let content = String::from("foo bar foo");
        let next_token_starting_index = find_beggining_of_next_token(&content);

        assert_eq!(next_token_starting_index, Some(4));
    }

    #[test]
    pub fn find_text_token_starting_index_for_single_token() {
        let content = String::from("fooBar");
        let next_token_starting_index = find_beggining_of_next_token(&content);

        assert_eq!(next_token_starting_index, None);
    }

    #[test]
    pub fn simple_tokenization_of_the_input() {
        let content = String::from("foo bar foo");
        let tokens = tokenize(&content);

        assert_eq!(tokens, vec!("foo", "bar", "foo"));
    }
}
