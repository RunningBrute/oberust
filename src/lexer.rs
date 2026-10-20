pub fn tokenize(content: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut last_token_size: usize = 0;
    let context_size: usize = content.chars().count();

    loop {
        if last_token_size > context_size {
            break;
        }

        for (_i, c) in content[last_token_size..context_size].char_indices() {
            if c.is_whitespace() {
                last_token_size = last_token_size + 1;
            } else {
                break;
            }
        }

        let new_token = get_next_token(&content[last_token_size..context_size]);
        match new_token {
            Some(value) => {
                tokens.push(value);
                last_token_size = last_token_size + value.chars().count();
                println!("Token added: {}, size: {}", value, last_token_size);
            }
            None => {
                if last_token_size != context_size {
                    tokens.push(&content[last_token_size..context_size]);
                }
                break;
            }
        }
    }

    tokens
}

pub fn get_next_token(content: &str) -> Option<&str> {
    println!("Searching for next token in slice: {}", content);
    println!("-----------------------------------------------");
    let is_end_of_token = |c: char| {
        return c.is_whitespace() || c == ';' || c == '.' || c == ':' || c == ',';
    };

    if content.starts_with(&[';', ',', '.', ':']) {
        return Some(&content[0..1]);
    }

    match content.find(is_end_of_token) {
        Some(value) => return Some(&content[0..value]),
        None => {
            println!("End of token not found");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    #[test]
    pub fn no_tokens_exist() {
        let content = String::from("");
        let tokens = tokenize(&content);

        assert_eq!(tokens.is_empty(), true);
    }

    #[test]
    pub fn no_tokens_exist_just_spaces() {
        let content = String::from("\n \t   \n  ");
        let tokens = tokenize(&content);

        assert_eq!(tokens.is_empty(), true);
    }

    #[test]
    pub fn only_one_simple_token_exist() {
        let content = String::from("foo");
        let tokens = tokenize(&content);

        assert_eq!(tokens, vec!("foo"));
    }

    #[test]
    pub fn one_simple_and_one_termination_token() {
        let content = String::from("foo.");
        let tokens = tokenize(&content);

        assert_eq!(tokens, vec!("foo", "."));
    }

    #[test]
    pub fn only_termination_token_exist() {
        let contents = vec![",", ";", ".", ":"];

        for content in contents {
            let tokens = tokenize(&content);
            assert_eq!(tokens[0], content);
        }
    }

    #[test]
    pub fn multiple_termination_tokens_should_be_ok_too() {
        let content = String::from(".:;");
        let tokens = tokenize(&content);

        assert_eq!(tokens, vec!(".", ":", ";"));
    }

    #[test]
    pub fn multiple_simple_tokens_exist() {
        let content = String::from("foo bar foo");
        let tokens = tokenize(&content);

        assert_eq!(tokens, vec!("foo", "bar", "foo"));
    }

    #[test]
    pub fn multiple_simple_tokens_exist_with_termination_tokens() {
        let content = String::from("foo; bar foo.");
        let tokens = tokenize(&content);

        assert_eq!(tokens, vec!("foo", ";", "bar", "foo", "."));
    }

    #[test]
    pub fn whitespaces_should_be_ommited() {
        let content = String::from("foo;  \t  bar: foo . \n \n");
        let tokens = tokenize(&content);

        assert_eq!(tokens, vec!("foo", ";", "bar", ":", "foo", "."));
    }
}
