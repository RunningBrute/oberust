use std::fmt::Error;

pub fn tokenize(content: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut token_begin = 0;
    let mut token_end = find_end_of_current_token(&content).expect("ble vle ble");
    let content_size = content.chars().count();
    println!("content_size in chars: {}", content_size);

    loop {
        println!("Try to add range: {}:{}", token_begin, token_end);
        tokens.push(&content[token_begin..token_end]);
        println!("Token added: {}", &content[token_begin..token_end]);
        token_begin = token_end + 1;
        println!("Try to find end index: {}:{}", token_begin, content_size);
        token_end = match find_end_of_current_token(&content[token_begin..content_size]) {
            Some(value) => value,
            None => break,
        };
        token_end = token_end + (content_size - 2 - token_begin - 1);
    }

    tokens
}

pub fn find_beggining_of_next_token(content: &str) -> Option<usize> {
    match content.find(|c: char| c.is_whitespace()) {
        Some(value) => {
            if value < content.chars().count() - 1 {
                return Some(value + 1);
            } else {
                return None;
            }
        }
        None => None,
    }
}

pub fn find_end_of_current_token(content: &str) -> Option<usize> {
    println!("Searching slice: {}", content);
    match content.find(|c: char| c.is_whitespace()) {
        Some(value) => Some(value),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;
    use crate::lexer::{find_beggining_of_next_token, find_end_of_current_token};

    #[test]
    pub fn check_if_i_understand_whitspace_correctly() {
        assert_eq!(' '.is_whitespace(), true);
        assert_eq!('\n'.is_whitespace(), true);
    }

    #[test]
    pub fn find_text_token_starting_index() {
        let content = String::from("foo bar foo");
        let next_token_idx = find_beggining_of_next_token(&content);

        assert_eq!(next_token_idx, Some(4));
    }

    #[test]
    pub fn find_text_token_starting_index_for_single_token() {
        let content = String::from("fooBar");
        let next_token_idx = find_beggining_of_next_token(&content);

        assert_eq!(next_token_idx, None);
    }

    #[test]
    pub fn find_text_token_ending_index_for_single_token() {
        let content = String::from("fooBar");
        let end_token_idx = find_end_of_current_token(&content);

        assert_eq!(end_token_idx, Some(5));
    }

    #[test]
    pub fn find_text_token_ending_index_for_multiple_tokens() {
        let content = String::from("foo bar foo");
        let end_token_idx = find_end_of_current_token(&content);

        assert_eq!(end_token_idx, Some(3));
    }

    #[test]
    pub fn simple_tokenization_of_the_input() {
        let content = String::from("foo bar foo");
        let tokens = tokenize(&content);

        assert_eq!(tokens, vec!("foo", "bar", "foo"));
    }
}
