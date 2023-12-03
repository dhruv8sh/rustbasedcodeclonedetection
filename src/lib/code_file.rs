use crate::lib::code_file::Token::{
    BooleanLiteral, CharLiteral, Comment, Delimiter, FloatingLiteral, Identifier, IntLiteral,
    Keyword, Operator, StringLiteral,
};
use crate::lib::grammar::user_defined_members::{ClassOrRecord, Enum, Interface};
use crate::lib::java_code;

struct CodeFile {
    package: String,
    imports: Vec<String>,
    classes: Vec<ClassOrRecord>,
    enums: Vec<Enum>,
    interfaces: Vec<Interface>,
}

enum Token {
    Keyword(String),
    Operator(String),
    Delimiter(char),
    Identifier(String),
    IntLiteral(i64),
    FloatingLiteral(f64),
    CharLiteral(char),
    StringLiteral(String),
    BooleanLiteral(bool),
    Comment(String),
    CodeScopeStart,
    CodeScopeEnd,
    GroupScopeStart,
    GroupScopeEnd,
    ArrayScopeStart,
    ArrayScopeEnd,
}
impl From<&str> for Token {
    fn from(value: &str) -> Self {
        if let Some(val) = value.chars().nth(0)
            && java_code::SEPARATORS.contains(&val)
        {
            Delimiter(val)
        } else if java_code::KEYWORDS.contains(&value) {
            Keyword(value.to_string())
        } else if java_code::KEYWORDS.contains(&value) {
            Operator(value.to_string())
        } else if value.starts_with("//") || value.starts_with("//") {
            Comment(value[2..].to_string())
        } else if let Ok(val) = value.parse::<i64>() {
            IntLiteral(val)
        } else if let Ok(val) = value.parse::<f64>() {
            FloatingLiteral(val)
        } else if let Ok(val) = value.parse::<bool>() {
            BooleanLiteral(val)
        } else if value.starts_with('\'') && value.ends_with('\'') {
            CharLiteral(value.chars().nth(1).unwrap())
        } else if value.starts_with('\"') && value.ends_with('\"') {
            StringLiteral(value[1..value.len() - 1].to_string())
        } else {
            Identifier(value.to_string())
        }
    }
}
struct TokenLine(Vec<Token>);

impl From<&str> for CodeFile {
    fn from(value: &str) -> Self {
        let mut double_quotes_began = false;
        let mut multi_line_comments_began = false;
        let mut value_clone = value.clone();
        let mut package = String::new();
        let mut imports = Vec::new();
        let mut comments: Vec<String> = Vec::new();
        let (start, end): (u32, u32) = (0, 0);

        for ch in value.chars() {}

        CodeFile {
            package,
            imports,
            classes: vec![],
            enums: vec![],
            interfaces: vec![],
        }
    }
}
