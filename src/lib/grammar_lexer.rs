// use crate::lib::grammar::expression::*;
// use crate::lib::grammar::user_defined_members::{ClassOrRecord, Enum, Interface, Method};
// use crate::lib::grammar::{JavaCodeFile, Statement};
// use regex_automata::Input;
//
// const NEWLINE_DELIMITERS: [char; 2] = ['{', '}'];
// const LINE_END_DELIMITERS: char = ';';
// const DELIMITERS: [&str; 17] = ["(", ")", "[", "]", ":", ",", ".", "?"];
//
// fn remove_comments(input: &str) -> String {
//     let mut multi_comment_continue = false;
//     input
//         .lines()
//         .map(|line| line.trim())
//         .filter(|line| !line.is_empty())
//         .filter_map(|line| {
//             if multi_comment_continue {
//                 if let Some(index) = line.find("*/") {
//                     multi_comment_continue = false;
//                     Some(line[index + 1..].to_string())
//                 } else {
//                     None
//                 }
//             } else if let Some(index) = line.find("//") {
//                 Some(line[..index].to_string())
//             } else if let Some(index) = line.find("/*") {
//                 multi_comment_continue = true;
//                 Some(line[..index].to_string())
//             } else {
//                 Some(line.to_string())
//             }
//         })
//         .collect::<String>()
// }
// fn lint(input: &str) -> String {
//     let mut multi_line_comment_continue = false;
//     let mid = input
//         .lines()
//         .map(|line| line.trim()[..line.find("//").unwrap_or(line.len())].to_string())
//         .filter(|line| !line.is_empty())
//         .collect::<String>();
//     String::new()
// }
//
// pub(crate) fn tokenize(input: &str) -> JavaCodeFile {
//     let mut last_char: char = '\n';
//     let list = input
//         .split(|c: char| c.is_ascii_whitespace())
//         .collect::<Vec<&str>>();
//     let ans = JavaCodeFile {
//         package: None,
//         imports: None,
//         classes: vec![],
//         enums: vec![],
//         interfaces: vec![],
//     };
//
//     //look for comments
//
//     //look for package
//
//     //look for imports
//
//     //look for classes
//
//     //look for enums
//
//     //look for interfaces
//
//     ans
// }
// pub(super) fn unexpected(value: &str) {
//     println!("Unexpected value:{value}... dropped");
// }
// pub(super) fn tokenize_class(class: &Vec<&str>) -> ClassOrRecord {
//     let class_ob = ClassOrRecord {
//         var: Variable,
//         data_members: vec![],
//         methods: vec![],
//         extends: None,
//         implements: None,
//         inner_classes: None,
//         inner_enum: None,
//         inner_interface: None,
//     };
//     //get meta
//     //look for member variables
//     //look for methods
//     todo!()
// }
// pub(super) fn tokenize_enum(enum_: &Vec<&str>) -> Enum {
//     //get meta
//     //get enumerations
//     //look for member variables
//     //look for methods
//     todo!()
// }
// pub(super) fn tokenize_interface(interface: &Vec<&str>) -> Interface {
//     //get meta
//     //look for member variables
//     //look for methods
//     todo!()
// }
// pub(super) fn tokenize_method(method: &Vec<&str>) -> Method {
//     //get meta
//     //get statements
//     //
//     todo!()
// }
//
// fn get_modifiers(mods: &Vec<&str>) -> Vec<Modifiers> {
//     mods.iter()
//         .map(|modifier| Modifiers::from(*modifier))
//         .collect::<Vec<Modifiers>>()
// }
//
// pub(crate) fn clean_tokenize(input: &str) -> Vec<Statement> {
//     unimplemented!()
// }
