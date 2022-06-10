#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Unknown,
    End,
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    token_type: TokenType,
    start: usize,
    end: usize,
}

impl Token {
    pub fn new(token_type: TokenType, start: usize, end: usize) -> Token{
        Token {
            token_type: token_type,
            start: start,
            end: end
        }
    }
}

pub struct Lexer {
    source: Vec<char>,
    index: usize
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            index: 0
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let next_token = self.next_token();
            tokens.push(next_token);
            if next_token.token_type == TokenType::End {
                return tokens;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.peek() {
            Some(ch) => {
                if *ch == ' ' {
                    self.advance();
                    return self.next_token();
                }
                if ch.is_digit(10) {
                    return self.next_number();
                }
                let token = match ch {
                    '+' => Token::new(TokenType::Plus, self.index, self.index),
                    '-' => Token::new(TokenType::Minus, self.index, self.index),
                    '*' => Token::new(TokenType::Star, self.index, self.index),
                    '/' => Token::new(TokenType::Slash, self.index, self.index),
                    _ => Token::new(TokenType::Unknown, self.index, self.index)
                };

                self.advance();
                token 
            },
            None => {
                return Token::new(TokenType::End, self.index, self.index);
            }
        };
        token
    }

    pub fn next_number(&mut self) -> Token {
        let start_index = self.index;
        let mut string = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                string.push(*ch);
            } else {
                break;
            }
            self.advance();
        }
        match self.peek() {
            Some('.') => {
                self.advance();
                string.push('.');
                while let Some(ch) = self.peek() {
                    if ch.is_digit(10) {
                        string.push(*ch);
                    } else {
                        break;
                    }
                    self.advance();
                }
            },
            _ => ()
        }
        Token::new(TokenType::Number(string.parse().unwrap()), start_index, self.index - 1)
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn peek(&self) -> Option<&char> {
        self.source.get(self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn match_tokens(tokens: Vec<Token>, expected: Vec<Token>) {
        for (token, expected) in tokens.iter().zip(expected.iter()) {
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_lexer() {
        let tokens = Lexer::new("123.123 + 9 - 8 * 7 / 0.001".to_string()).lex();
        let expected = vec![
            Token::new(TokenType::Number(123.123), 0, 6),
            Token::new(TokenType::Plus, 8, 8),
            Token::new(TokenType::Number(9.0), 10, 10),
            Token::new(TokenType::Minus, 12, 12),
            Token::new(TokenType::Number(8.0), 14, 14),
            Token::new(TokenType::Star, 16, 16),
            Token::new(TokenType::Number(7.0), 18, 18),
            Token::new(TokenType::Slash, 20, 20),
            Token::new(TokenType::Number(0.001), 22, 26),
            Token::new(TokenType::End, 27, 27),
        ];
        assert_eq!(tokens, expected);
    }
}
