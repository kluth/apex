use super::lexer::{Lexer, Token};
use crate::domain::ast::bone::{AssetPath, Bone, Mass, MeshReference};
use crate::domain::ast::joint::{Joint, JointAttachment, JointType};
use crate::domain::ast::muscle::Muscle;
use crate::domain::ast::synapse::Synapse;
use crate::domain::movement::cpg::Cpg;
use crate::domain::biomechanics::rigid_body::Vector3;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token, String),
    InvalidMass(String),
    MissingProperty(String),
    BoneNotFound(String),
    MuscleNotFound(String),
    CpgNotFound(String),
}

pub struct OrganismAst {
    pub name: String,
    pub bones: Vec<Bone>,
    pub joints: Vec<Joint>,
    pub muscles: Vec<Muscle>,
    pub cpgs: Vec<Cpg>,
    pub synapses: Vec<Synapse>,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    base_path: String,
    bone_registry: HashMap<String, Bone>,
    muscle_registry: HashMap<String, Muscle>,
    cpg_registry: HashMap<String, Cpg>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            base_path: "examples/".to_string(),
            bone_registry: HashMap::new(),
            muscle_registry: HashMap::new(),
            cpg_registry: HashMap::new(),
        }
    }

    pub fn with_base_path(mut self, path: String) -> Self {
        self.base_path = path;
        self
    }

    pub fn with_registries(
        mut self,
        bones: HashMap<String, Bone>,
        muscles: HashMap<String, Muscle>,
        cpgs: HashMap<String, Cpg>,
    ) -> Self {
        self.bone_registry = bones;
        self.muscle_registry = muscles;
        self.cpg_registry = cpgs;
        self
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
        let mut cpgs = Vec::new();
        let mut synapses = Vec::new();

        self.parse_body_elements(&mut bones, &mut joints, &mut muscles, &mut cpgs, &mut synapses, false)?;

        self.expect(Token::BraceClose)?;
        Ok(OrganismAst {
            name,
            bones,
            joints,
            muscles,
            cpgs,
            synapses,
        })
    }

    fn parse_body_elements(
        &mut self,
        bones: &mut Vec<Bone>,
        joints: &mut Vec<Joint>,
        muscles: &mut Vec<Muscle>,
        cpgs: &mut Vec<Cpg>,
        synapses: &mut Vec<Synapse>,
        is_include: bool,
    ) -> Result<(), ParseError> {
        let terminal = if is_include { Token::Eof } else { Token::BraceClose };

        while self.current_token != terminal && self.current_token != Token::Eof {
            match self.current_token {
                Token::Include => {
                    self.advance();
                    if let Token::StringLiteral(path) = &self.current_token {
                        let full_path = format!("{}{}", self.base_path, path);
                        let source = fs::read_to_string(&full_path)
                            .map_err(|e| ParseError::UnexpectedToken(Token::Eof, e.to_string()))?;

                        let mut sub_parser = Parser::new(&source)
                            .with_base_path(self.base_path.clone())
                            .with_registries(
                                self.bone_registry.clone(),
                                self.muscle_registry.clone(),
                                self.cpg_registry.clone(),
                            );

                        sub_parser.parse_body_elements(bones, joints, muscles, cpgs, synapses, true)?;
                        
                        // Sync registries
                        self.bone_registry.extend(sub_parser.bone_registry);
                        self.muscle_registry.extend(sub_parser.muscle_registry);
                        self.cpg_registry.extend(sub_parser.cpg_registry);

                        self.advance();
                        self.expect(Token::Semicolon)?;
                    }
                }
                Token::Bone => {
                    let bone = self.parse_bone()?;
                    self.bone_registry.insert(bone.id().to_string(), bone.clone());
                    bones.push(bone);
                }
                Token::Joint => {
                    joints.push(self.parse_joint()?);
                }
                Token::Muscle => {
                    let muscle = self.parse_muscle()?;
                    self.muscle_registry.insert(muscle.id().to_string(), muscle.clone());
                    muscles.push(muscle);
                }
                Token::Cpg => {
                    let cpg = self.parse_cpg()?;
                    self.cpg_registry.insert(cpg.id().to_string(), cpg.clone());
                    cpgs.push(cpg);
                }
                Token::Synapse => {
                    synapses.push(self.parse_synapse()?);
                }
                _ => self.advance(),
            }
        }
        Ok(())
    }

    fn parse_bone(&mut self) -> Result<Bone, ParseError> {
        self.expect(Token::Bone)?;
        let id = self.read_identifier("Expected bone id")?;
        self.advance();

        self.expect(Token::BraceOpen)?;
        let mut mass = None;
        let mut position = Vector3::default();
        let mut mesh_ref = None;

        while self.current_token != Token::BraceClose && self.current_token != Token::Eof {
            if let Token::Identifier(prop) = &self.current_token {
                match prop.as_str() {
                    "mass" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        if let Token::Number(val) = self.current_token {
                            self.advance();
                            self.expect(Token::Kg)?;
                            self.expect(Token::Semicolon)?;
                            mass = Some(Mass::new(val).map_err(|_| ParseError::InvalidMass(val.to_string()))?);
                        }
                    }
                    "position" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        self.expect(Token::ParenOpen)?;
                        let x = if let Token::Number(val) = self.current_token { val } else { 0.0 };
                        self.advance();
                        self.expect(Token::Comma)?;
                        let y = if let Token::Number(val) = self.current_token { val } else { 0.0 };
                        self.advance();
                        self.expect(Token::Comma)?;
                        let z = if let Token::Number(val) = self.current_token { val } else { 0.0 };
                        self.advance();
                        self.expect(Token::ParenClose)?;
                        self.expect(Token::Semicolon)?;
                        position = Vector3 { x, y, z };
                    }
                    "mesh" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        if let Token::StringLiteral(path_str) = &self.current_token {
                            let path = AssetPath::new(path_str).map_err(|_| ParseError::UnexpectedToken(Token::Eof, "Invalid mesh path".to_string()))?;
                            mesh_ref = Some(MeshReference::new(path));
                            self.advance();
                            self.expect(Token::Semicolon)?;
                        }
                    }
                    _ => self.advance(),
                }
            } else {
                self.advance();
            }
        }
        self.expect(Token::BraceClose)?;

        let mass = mass.ok_or_else(|| ParseError::MissingProperty("mass".to_string()))?;
        let mut bone = Bone::new(id, mass, position);
        if let Some(mr) = mesh_ref {
            bone = bone.with_mesh(mr);
        }
        Ok(bone)
    }

    fn parse_joint(&mut self) -> Result<Joint, ParseError> {
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

        let b1 = self.bone_registry.get(&s_id).ok_or(ParseError::BoneNotFound(s_id))?;
        let b2 = self.bone_registry.get(&t_id).ok_or(ParseError::BoneNotFound(t_id))?;

        Joint::new(id, JointType::Spherical, b1, b2, JointAttachment::default(), JointAttachment::default())
            .map_err(|e| ParseError::UnexpectedToken(Token::Eof, format!("{:?}", e)))
    }

    fn parse_muscle(&mut self) -> Result<Muscle, ParseError> {
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

        let b1 = self.bone_registry.get(&s_id).ok_or(ParseError::BoneNotFound(s_id))?;
        let b2 = self.bone_registry.get(&t_id).ok_or(ParseError::BoneNotFound(t_id))?;

        Ok(Muscle::new(id, b1, b2, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), max_force))
    }

    fn parse_cpg(&mut self) -> Result<Cpg, ParseError> {
        self.expect(Token::Cpg)?;
        let id = self.read_identifier("Expected CPG id")?;
        self.advance();

        self.expect(Token::BraceOpen)?;
        let mut freq = 1.0;

        while self.current_token != Token::BraceClose && self.current_token != Token::Eof {
            if let Token::Identifier(prop) = &self.current_token {
                match prop.as_str() {
                    "frequency" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        if let Token::Number(val) = self.current_token {
                            self.advance();
                            self.expect(Token::Hz)?;
                            self.expect(Token::Semicolon)?;
                            freq = val;
                        }
                    }
                    _ => self.advance(),
                }
            } else {
                self.advance();
            }
        }
        self.expect(Token::BraceClose)?;
        Ok(Cpg::new(id, freq))
    }

    fn parse_synapse(&mut self) -> Result<Synapse, ParseError> {
        self.expect(Token::Synapse)?;
        let id = self.read_identifier("Expected synapse id")?;
        self.advance();

        self.expect(Token::BraceOpen)?;
        let mut source = None;
        let mut target = None;
        let mut weight = 1.0;

        while self.current_token != Token::BraceClose && self.current_token != Token::Eof {
            if let Token::Identifier(prop) = &self.current_token {
                match prop.as_str() {
                    "source" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        source = Some(self.read_identifier("Expected CPG id")?);
                        self.advance();
                        self.expect(Token::Semicolon)?;
                    }
                    "target" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        target = Some(self.read_identifier("Expected muscle id")?);
                        self.advance();
                        self.expect(Token::Semicolon)?;
                    }
                    "weight" => {
                        self.advance();
                        self.expect(Token::Equal)?;
                        if let Token::Number(val) = self.current_token {
                            self.advance();
                            self.expect(Token::Semicolon)?;
                            weight = val;
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

        let cpg = self.cpg_registry.get(&s_id).ok_or(ParseError::CpgNotFound(s_id))?;
        let muscle = self.muscle_registry.get(&t_id).ok_or(ParseError::MuscleNotFound(t_id))?;

        Ok(Synapse::new(id, cpg, muscle, weight))
    }

    fn read_identifier(&self, msg: &str) -> Result<String, ParseError> {
        if let Token::Identifier(n) = &self.current_token {
            Ok(n.clone())
        } else {
            Err(ParseError::UnexpectedToken(self.current_token.clone(), msg.to_string()))
        }
    }
}
