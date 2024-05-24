use std::{iter::Peekable, path::PathBuf, str::Chars};

struct Lexer<'src> {
    chars: Peekable<Chars<'src>>,
    tokens: Vec<String>,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Lexer {
        Lexer {
            chars: source.chars().peekable(),
            tokens: Vec::new(),
        }
    }

    fn finish(self) -> Vec<String> {
        self.tokens
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn eat(&mut self) {
        self.chars.next();
    }
}

pub fn lex(source: &str, stem: bool) -> Vec<String> {
    let mut lexer = Lexer::new(source);

    while lexer.peek().is_some() {
        while let Some(c) = lexer.peek() {
            if !c.is_alphabetic() {
                lexer.eat();
            } else {
                break;
            }
        }

        let mut token = String::new();
        while let Some(c) = lexer.peek() {
            if c.is_alphabetic() {
                lexer.eat();
                token.push(c.to_ascii_lowercase());
            } else {
                break;
            }
        }

        if !token.is_empty() && !is_stop_word(&token) {
            if stem {
                lexer.tokens.push(porter_stemmer::stem(&token));
            } else {
                lexer.tokens.push(token);
            }
        }
    }

    lexer.finish()
}

const STOP_WORDS: [&'static str; 16] = [
    "a", "an", "as", "are", "and", "the", "via", "for", "is", "or", "in", "of", "it", "to", "on",
    "by",
];

fn is_stop_word(word: &str) -> bool {
    STOP_WORDS.iter().find(|&&v| v == word).is_some()
}
