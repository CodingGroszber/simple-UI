use std::fs;

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "Error reading file".to_string())
}


#[derive(PartialEq)]
pub enum TokenType {
    Word,           // Alphanumeric sequences (e.g., "Heading", "bold")
    Space,          // Space characters (" ")
    Tab,            // Tab characters ("\t")
    Newline,        // Newline characters ("\n")
    Punctuation,    // General punctuation not specific to Markdown (e.g., ",", ".")
    MarkdownHeader, // "#" for headers
    MarkdownBold,   // "**" for bold text
    MarkdownItalic, // "*" for italic text
    MarkdownOther,  // Other Markdown syntax (e.g., "-", "```")
}


pub struct Token {
    pub text: String,
    pub token_type: TokenType,
}

pub fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if current.is_empty() {
            current.push(c);
        } else {
            let last_type = determine_type(&current);
            let c_type = determine_type(&c.to_string());

            // Check for Markdown-specific sequences
            if c == '*' && chars.peek() == Some(&'*') {
                if !current.is_empty() {
                    tokens.push(Token {
                        text: current.clone(),
                        token_type: last_type,
                    });
                    current.clear();
                }
                current.push(c);
                current.push(chars.next().unwrap()); // Consume the second '*'
                tokens.push(Token {
                    text: current.clone(),
                    token_type: TokenType::MarkdownBold,
                });
                current.clear();
                continue;
            } else if c == '*' {
                if !current.is_empty() {
                    tokens.push(Token {
                        text: current.clone(),
                        token_type: last_type,
                    });
                    current.clear();
                }
                current.push(c);
                tokens.push(Token {
                    text: current.clone(),
                    token_type: TokenType::MarkdownItalic,
                });
                current.clear();
                continue;
            } else if c == '#' && (current.is_empty() || current.chars().all(|ch| ch.is_whitespace())) {
                if !current.is_empty() {
                    tokens.push(Token {
                        text: current.clone(),
                        token_type: last_type,
                    });
                    current.clear();
                }
                current.push(c);
                tokens.push(Token {
                    text: current.clone(),
                    token_type: TokenType::MarkdownHeader,
                });
                current.clear();
                continue;
            } else if c == '\n' {
                if !current.is_empty() {
                    tokens.push(Token {
                        text: current.clone(),
                        token_type: last_type,
                    });
                    current.clear();
                }
                current.push(c);
                tokens.push(Token {
                    text: current.clone(),
                    token_type: TokenType::Newline,
                });
                current.clear();
                continue;
            } else if c == '\t' {
                if !current.is_empty() {
                    tokens.push(Token {
                        text: current.clone(),
                        token_type: last_type,
                    });
                    current.clear();
                }
                current.push(c);
                tokens.push(Token {
                    text: current.clone(),
                    token_type: TokenType::Tab,
                });
                current.clear();
                continue;
            } else if c.is_whitespace() && c != '\n' && c != '\t' {
                if last_type != TokenType::Space {
                    if !current.is_empty() {
                        tokens.push(Token {
                            text: current.clone(),
                            token_type: last_type,
                        });
                        current.clear();
                    }
                    current.push(c);
                } else {
                    current.push(c);
                }
            } else if c.is_alphanumeric() || c == '_' {
                if last_type != TokenType::Word {
                    if !current.is_empty() {
                        tokens.push(Token {
                            text: current.clone(),
                            token_type: last_type,
                        });
                        current.clear();
                    }
                    current.push(c);
                } else {
                    current.push(c);
                }
            } else {
                if last_type != TokenType::Punctuation && last_type != TokenType::MarkdownOther {
                    if !current.is_empty() {
                        tokens.push(Token {
                            text: current.clone(),
                            token_type: last_type,
                        });
                        current.clear();
                    }
                    current.push(c);
                } else {
                    current.push(c);
                }
            }
        }
    }

    if !current.is_empty() {
        tokens.push(Token {
            token_type: determine_type(&current),
            text: current,
        });
    }

    tokens
}

fn determine_type(text: &str) -> TokenType {
    if text == "\n" {
        TokenType::Newline
    } else if text == "\t" {
        TokenType::Tab
    } else if text == "**" {
        TokenType::MarkdownBold
    } else if text == "*" {
        TokenType::MarkdownItalic
    } else if text == "#" {
        TokenType::MarkdownHeader
    } else if text.chars().all(|c| c.is_alphanumeric() || c == '_') {
        TokenType::Word
    } else if text.chars().all(|c| c.is_whitespace() && c != '\n' && c != '\t') {
        TokenType::Space
    } else {
        TokenType::Punctuation
    }
}