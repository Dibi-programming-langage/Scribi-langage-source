mod parse_variables;
mod parse_values;

use crate::tokens::{ModifierKeyword, Token};
use skribi_language_source::error;

pub enum ValueNode {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Unset,
}

pub enum Node {
    Scope(Vec<Node>),
    NewVariable(Vec<ModifierKeyword>, String, ValueNode),
    NewValue(String, ValueNode),
    NativeCall(String, Vec<ValueNode>),
}

struct ParseFunction {
    name: String,
    arguments: Vec<String>,
    return_type: String,
}

/// Only used to check if a variable exists in a scope
struct ParseScope {
    /// Variables that can be used in this scope
    variables: Vec<String>,
    /// Types that can be used in this scope
    types: Vec<String>,
    /// Functions that can be used in this scope. UNUSED FOR NOW
    functions: Vec<ParseFunction>,
    parent: Option<Box<ParseScope>>,
}

impl ParseScope {
    fn new(parent: Option<Box<ParseScope>>) -> Self {
        ParseScope {
            variables: Vec::new(),
            types: Vec::new(),
            functions: Vec::new(),
            parent,
        }
    }

    fn base() -> Self {
        ParseScope {
            variables: Vec::new(),
            types: vec![
                "skr".to_string(),
                "int".to_string(),
                "dar".to_string(),
                "ioi".to_string(),
            ],
            functions: Vec::new(),
            parent: None,
        }
    }

    /// Check if a name can be used in this scope for a variable
    fn is_valid_name_for_variable(&self, name: String) -> bool {
        !(
            self.variables.contains(&name)
            || self.types.contains(&name)
            || self.functions.iter().any(|f| f.name == name)
            || (
                if let Some(parent) = &self.parent {
                    parent.is_valid_name_for_variable(name)
                } else {
                    false
                }
            )
        )
    }

    /// Check if a type exists in this scope
    fn is_valid_type(&self, name: String) -> bool {
        self.types.contains(&name)
        || (
            if let Some(parent) = &self.parent {
                parent.is_valid_type(name)
            } else {
                false
            }
        )
    }

    /// Check if a variable exists in this scope
    fn is_valid_variable(&self, name: String) -> bool {
        self.variables.contains(&name)
        || (
            if let Some(parent) = &self.parent {
                parent.is_valid_variable(name)
            } else {
                false
            }
        )
    }
}

fn parse_scope(
    tokens: &Vec<Token>,
    i: &mut usize,
    line: &mut u16,
    variables: &Vec<Vec<String>>,
) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut not_finished = true;

    // Start an iterator with a index
    while not_finished && *i < tokens.len() {
        match tokens[*i] {
            Token::KeywordModifier(_) => {
                // Start a new variable
            }
            Token::KeywordNativeCall => {
                // Start a new native call
            }
            Token::Identifier(_) => {
                // Check if the identifier exists in this scope, and his type
            }
            Token::OpenBrace => {
                // Start a new scope
                *i += 1;
                let scope_nodes = parse_scope(tokens, i, line, variables);
                nodes.push(Node::Scope(scope_nodes));
            }
            Token::CloseBrace => {
                // Close the current scope
                not_finished = false;
            }
            Token::NewLine => {
                *line += 1;
            }
            // Ignored tokens
            Token::Semicolon => {}
            _ => {
                error("[PARSE] Invalid token or not implemented yet!", *line);
            }
        }
        *i += 1;
    }

    nodes
}

pub fn main(tokens: Vec<Token>) -> Vec<Node> {
    let mut line = 0;
    let mut i = 0;
    let vec: Vec<Vec<String>> = Vec::new();
    let nodes = parse_scope(&tokens, &mut i, &mut line, &vec);
    if i != tokens.len() {
        error("Scope closed with } before the end", line);
    }
    nodes
}
