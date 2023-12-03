pub const KEYWORDS: [&str; 49] = [
    "abstract",
    "assert",
    "boolean",
    "break",
    "byte",
    "case",
    "catch",
    "char",
    "class",
    "const",
    "continue",
    "default",
    "do",
    "double",
    "else",
    "enum",
    "extends",
    "final",
    "finally",
    "float",
    "for",
    "if",
    "implements",
    "import",
    "instanceof",
    "int",
    "interface",
    "long",
    "native",
    "new",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "short",
    "static",
    "strictfp",
    "super",
    "switch",
    "synchronized",
    "this",
    "throw",
    "throws",
    "transient",
    "try",
    "void",
    "volatile",
    "while",
];
pub const SEPARATORS: [char; 4] = ['[', ']', '{', '}'];
pub const OPERATORS: [&str; 43] = [
    "+", "-", "*", "/", "%", "++", "--", "==", "!=", ">", "<", ">=", "<=", "&&", "||", "!", "&",
    "|", "^", "~", "<<", ">>", ">>>", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=",
    ">>>=", "?", ":", "=", "->", ".", "::", ",", "(", ")",
];
pub fn operator_iterable_check(dat: &str) -> bool {
    OPERATORS.iter().any(|operator| operator.starts_with(dat))
}
pub fn operator_exact_check(dat: &str) -> bool {
    OPERATORS.contains(&dat)
}
pub fn operator_char_check(dat: char) -> bool {
    OPERATORS.iter().any(|operator| operator.starts_with(dat))
}
pub const END_LINE_DELIMITER: char = ';';
