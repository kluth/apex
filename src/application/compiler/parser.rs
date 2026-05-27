use super::lexer::{Lexer, Token};
use crate::domain::ast::bone::{Bone, Mass};
use crate::domain::ast::joint::{Joint, JointAttachment, JointType};
use crate::domain::ast::muscle::Muscle;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token, String),
    InvalidMass(String),
    MissingProperty(String),
    BoneNotFound(String),
}

pub struct OrganismAst {
    pub name: String,
    pub bones: Vec<Bone>,
    pub joints: Vec<Joint>,
    pub muscles: Vec<Muscle>,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
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

    pub fn parse_organism(&mut self) -> Result<OrganismAst, ParseError> {
        self.expect(Token::Organism)?;

        let name = if let Token::Identifier(n) = &self.current_token {
            n.clone()
        } else {
            return Err(ParseError::UnexpectedToken(
                self.current_token.clone(),
                "Expected organism name".to_string(),
            ));
        };
        self.advance();

        self.expect(Token::BraceOpen)?;

        let mut bones = Vec::new();
        let mut joints = Vec::new();
        let mut muscles = Vec::new();

        // Temporary map to resolve bone references during parsing
        let mut bone_registry: HashMap<String, Bone> = HashMap::new();

        while self.current_token != Token::BraceClose && self.current_token != Token::Eof {
            match self.current_token {
                Token::Bone => {
                    let bone = self.parse_bone()?;
                    bone_registry.insert(bone.id().to_string(), bone.clone());
                    bones.push(bone);
                }
                Token::Joint => {
                    joints.push(self.parse_joint(&bone_registry)?);
                }
                Token::Muscle => {
                    muscles.push(self.parse_muscle(&bone_registry)?);
                }
                _ => self.advance(),
            }
        }

        self.expect(Token::BraceClose)?;
        Ok(OrganismAst {
            name,
            bones,
            joints,
            muscles,
        })
    }

    fn parse_bone(&mut self) -> Result<Bone, ParseError> {
        self.expect(Token::Bone)?;
        let id = self.read_identifier("Expected bone id")?;
        self.advance();

        self.expect(Token::BraceOpen)?;
        let mut mass = None;

        while self.current_token != Token::BraceClose && self.current_token != Token::Eof {
            if let Token::Identifier(prop) = &self.current_token {
                if prop == "mass" {
                    self.advance();
                    self.expect(Token::Equal)?;
                    if let Token::Number(val) = self.current_token {
                        self.advance();
                        self.expect(Token::Kg)?;
                        self.expect(Token::Semicolon)?;
                        mass = Some(
                            Mass::new(val).map_err(|_| ParseError::InvalidMass(val.to_string()))?,
                        );
                    }
                } else {
                    self.advance();
                }
            } else {
                self.advance();
            }
        }
        self.expect(Token::BraceClose)?;

        let mass = mass.ok_or_else(|| ParseError::MissingProperty("mass".to_string()))?;
        Ok(Bone::new(id, mass))
    }

    fn parse_joint(&mut self, bones: &HashMap<String, Bone>) -> Result<Joint, ParseError> {
        self.expect(Token::Joint)?;
        let id = self.read_identifier("Expected joint id")?;
        self.advance();

        self.expect(Token::BraceOpen)?;
        let mut source = None;
        let mut target = None;

        while self.current_token != Token::BraceClose && self.current_token != Token::Eof {
            if let Token::Identifier(prop) = &self.current_token {
                match prop.as_str() {
                    "source" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        source = Some(self.read_identifier("Expected bone id")?);
                        self.advance();
                        self.expect(Token::Semicolon)?;
                    }
                    "target" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        target = Some(self.read_identifier("Expected bone id")?);
                        self.advance();
                        self.expect(Token::Semicolon)?;
                    }
                    _ => self.advance(),
                }
            } else {
                self.advance();
            }
        }
        self.expect(Token::BraceClose)?;

        let s_id = source.ok_or_else(|| ParseError::MissingProperty("source".to_string()))?;
        let t_id = target.ok_or_else(|| ParseError::MissingProperty("target".to_string()))?;

        let b1 = bones.get(&s_id).ok_or(ParseError::BoneNotFound(s_id))?;
        let b2 = bones.get(&t_id).ok_or(ParseError::BoneNotFound(t_id))?;

        Joint::new(
            id,
            JointType::Spherical,
            b1,
            b2,
            JointAttachment::default(),
            JointAttachment::default(),
        )
        .map_err(|e| ParseError::UnexpectedToken(Token::Eof, format!("{:?}", e)))
    }

    fn parse_muscle(&mut self, bones: &HashMap<String, Bone>) -> Result<Muscle, ParseError> {
        self.expect(Token::Muscle)?;
        let id = self.read_identifier("Expected muscle id")?;
        self.advance();

        self.expect(Token::BraceOpen)?;
        let mut source = None;
        let mut target = None;
        let mut max_force = 100.0;

        while self.current_token != Token::BraceClose && self.current_token != Token::Eof {
            if let Token::Identifier(prop) = &self.current_token {
                match prop.as_str() {
                    "source" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        source = Some(self.read_identifier("Expected bone id")?);
                        self.advance();
                        self.expect(Token::Semicolon)?;
                    }
                    "target" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        target = Some(self.read_identifier("Expected bone id")?);
                        self.advance();
                        self.expect(Token::Semicolon)?;
                    }
                    "max_force" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        if let Token::Number(val) = self.current_token {
                            self.advance();
                            self.expect(Token::Nm)?;
                            self.expect(Token::Semicolon)?;
                            max_force = val;
                        }
                    }
                    _ => self.advance(),
                }
            } else {
                self.advance();
            }
        }
        self.expect(Token::BraceClose)?;

        let s_id = source.ok_or_else(|| ParseError::MissingProperty("source".to_string()))?;
        let t_id = target.ok_or_else(|| ParseError::MissingProperty("target".to_string()))?;

        let b1 = bones.get(&s_id).ok_or(ParseError::BoneNotFound(s_id))?;
        let b2 = bones.get(&t_id).ok_or(ParseError::BoneNotFound(t_id))?;

        Ok(Muscle::new(
            id,
            b1,
            b2,
            (0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0),
            max_force,
        ))
    }

    fn read_identifier(&self, msg: &str) -> Result<String, ParseError> {
        if let Token::Identifier(n) = &self.current_token {
            Ok(n.clone())
        } else {
            Err(ParseError::UnexpectedToken(
                self.current_token.clone(),
                msg.to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_complex_organism() {
        let input = "
            organism Worm {
                bone Head { mass = 1.0 kg; }
                bone Tail { mass = 1.0 kg; }
                joint Neck { source = Head; target = Tail; }
                muscle Motor { source = Head; target = Tail; max_force = 50.0 Nm; }
            }
        ";
        let mut parser = Parser::new(input);
        let ast = parser.parse_organism().unwrap();

        assert_eq!(ast.name, "Worm");
        assert_eq!(ast.bones.len(), 2);
        assert_eq!(ast.joints.len(), 1);
        assert_eq!(ast.muscles.len(), 1);
        assert_eq!(ast.muscles[0].max_force(), 50.0);
    }
}
