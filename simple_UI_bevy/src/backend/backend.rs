use std::fs;

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "Error reading file".to_string())
}

pub struct Token {
    pub text: String,
    pub is_word: bool,
}

pub fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for c in text.chars() {
        if c.is_alphanumeric() {
            current.push(c);
        } else {
            if !current.is_empty() {
                tokens.push(Token {
                    text: current.clone(),
                    is_word: true,
                });
                current.clear();
            }
            if !c.is_whitespace() {
                tokens.push(Token {
                    text: c.to_string(),
                    is_word: false,
                });
            }
        }
    }
    if !current.is_empty() {
        tokens.push(Token {
            text: current,
            is_word: true,
        });
    }
    tokens
}
