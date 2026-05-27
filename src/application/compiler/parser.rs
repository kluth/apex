use super::lexer::{Lexer, Token};
use crate::domain::ast::bone::{Bone, Mass};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token, String),
    InvalidMass(String),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(
                self.current_token.clone(),
                format!("Expected {:?}", expected),
            ))
        }
    }

    /// Parses an organism block and returns a list of Bones.
    /// organism Name { ... }
    pub fn parse_organism(&mut self) -> Result<(String, Vec<Bone>), ParseError> {
        self.expect(Token::Organism)?;
        
        let name = if let Token::Identifier(n) = &self.current_token {
            n.clone()
        } else {
            return Err(ParseError::UnexpectedToken(self.current_token.clone(), "Expected organism name".to_string()));
        };
        self.advance();

        self.expect(Token::BraceOpen)?;
        
        let mut bones = Vec::new();
        while self.current_token != Token::BraceClose && self.current_token != Token::Eof {
            match self.current_token {
                Token::Bone => {
                    bones.push(self.parse_bone()?);
                }
                _ => self.advance(), // Skip other tokens for now (Joints, Muscles)
            }
        }

        self.expect(Token::BraceClose)?;
        Ok((name, bones))
    }

    /// Parses a bone definition.
    /// bone Name { mass = 1.0 kg; }
    fn parse_bone(&mut self) -> Result<Bone, ParseError> {
        self.expect(Token::Bone)?;

        let id = if let Token::Identifier(n) = &self.current_token {
            n.clone()
        } else {
            return Err(ParseError::UnexpectedToken(self.current_token.clone(), "Expected bone id".to_string()));
        };
        self.advance();

        self.expect(Token::BraceOpen)?;

        let mut mass = None;

        while self.current_token != Token::BraceClose && self.current_token != Token::Eof {
            match &self.current_token {
                Token::Identifier(prop) if prop == "mass" => {
                    self.advance();
                    self.expect(Token::Equal)?;
                    if let Token::Number(val) = self.current_token {
                        self.advance();
                        self.expect(Token::Kg)?;
                        self.expect(Token::Semicolon)?;
                        mass = Some(Mass::new(val).map_err(|_| ParseError::InvalidMass(val.to_string()))?);
                    }
                }
                _ => self.advance(),
            }
        }

        self.expect(Token::BraceClose)?;

        let mass = mass.ok_or_else(|| ParseError::InvalidMass("Missing mass".to_string()))?;
        Ok(Bone::new(id, mass))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_bone_definition() {
        let input = "organism Biped { bone Femur { mass = 2.5 kg; } }";
        let mut parser = Parser::new(input);
        let (name, bones) = parser.parse_organism().unwrap();

        assert_eq!(name, "Biped");
        assert_eq!(bones.len(), 1);
        assert_eq!(bones[0].id(), "Femur");
        assert_eq!(bones[0].mass().value(), 2.5);
    }
}
