use std::fmt::Error;

pub fn tokenize(content: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut last_token_size: usize = 0;
    let mut remeining_context_size: usize = content.chars().count();

    for c in content.chars() {
        println!("next letter: {}", c);
    }

    loop {
        let new_token = get_next_token(&content[last_token_size..remeining_context_size]);
        match new_token {
            Some(value) => {
                tokens.push(value);
                last_token_size = value.chars().count() + 1;
                println!("Token added: {}, size: {}", value, last_token_size);
                //remeining_context_size = remeining_context_size - last_token_size;
            }
            None => break,
        }
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

pub fn get_next_token(content: &str) -> Option<&str> {
    println!("Searching for next token in slice: {}", content);
    println!("-----------------------------------------------");
    let is_end_of_token = |c: char| {
        return c.is_whitespace() || c == ';' || c == '.' || c == ':';
    };

    match content.find(is_end_of_token) {
        Some(value) => return Some(&content[0..value]),
        None => {
            println!("End of token not found");
            None
        }
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
