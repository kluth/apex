#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Organism,
    Substrate,
    Bone,
    Joint,
    Muscle,

    // Identifiers and Literals
    Identifier(String),
    Number(f64),

    // Units
    Kg,
    M,
    Nm,
    Rad,

    // Delimiters
    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,
    Equal,
    Comma,
    Semicolon,

    // EOF
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Token::Eof;
        }

        let current_char = self.input[self.pos..].chars().next().unwrap();

        if current_char.is_alphabetic() || current_char == '_' {
            return self.read_identifier();
        }

        if current_char.is_ascii_digit() || current_char == '.' {
            return self.read_number();
        }

        self.pos += current_char.len_utf8();
        match current_char {
            '{' => Token::BraceOpen,
            '}' => Token::BraceClose,
            '(' => Token::ParenOpen,
            ')' => Token::ParenClose,
            '=' => Token::Equal,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            _ => self.next_token(), // Skip unknown characters
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.input[self.pos..].chars().next().unwrap();
            if ch.is_whitespace() {
                self.pos += ch.len_utf8();
            } else if ch == '#' {
                // Skip pragma/comments for now
                self.pos += ch.len_utf8();
                while self.pos < self.input.len() {
                    let ch_inner = self.input[self.pos..].chars().next().unwrap();
                    if ch_inner == '\n' {
                        break;
                    }
                    self.pos += ch_inner.len_utf8();
                }
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() {
            let ch = self.input[self.pos..].chars().next().unwrap();
            if ch.is_alphanumeric() || ch == '_' {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
        let identifier = &self.input[start..self.pos];
        match identifier {
            "organism" => Token::Organism,
            "substrate" => Token::Substrate,
            "bone" => Token::Bone,
            "joint" => Token::Joint,
            "muscle" => Token::Muscle,
            "kg" => Token::Kg,
            "m" => Token::M,
            "Nm" => Token::Nm,
            "rad" => Token::Rad,
            _ => Token::Identifier(identifier.to_string()),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() {
            let ch = self.input[self.pos..].chars().next().unwrap();
            if ch.is_ascii_digit() || ch == '.' {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
        let number_str = &self.input[start..self.pos];
        Token::Number(number_str.parse().unwrap_or(0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_tokenization() {
        let input = "organism Biped { bone Femur { mass = 2.0 kg; } }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Organism);
        assert_eq!(lexer.next_token(), Token::Identifier("Biped".to_string()));
        assert_eq!(lexer.next_token(), Token::BraceOpen);
        assert_eq!(lexer.next_token(), Token::Bone);
        assert_eq!(lexer.next_token(), Token::Identifier("Femur".to_string()));
        assert_eq!(lexer.next_token(), Token::BraceOpen);
        assert_eq!(lexer.next_token(), Token::Identifier("mass".to_string()));
        assert_eq!(lexer.next_token(), Token::Equal);
        assert_eq!(lexer.next_token(), Token::Number(2.0));
        assert_eq!(lexer.next_token(), Token::Kg);
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::BraceClose);
        assert_eq!(lexer.next_token(), Token::BraceClose);
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
