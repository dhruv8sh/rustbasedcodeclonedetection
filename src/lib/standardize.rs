use crate::lib::java_code;

pub(crate) fn remove_comments_and_standardize_whitespace(java_code: &str) -> String {
    enum State {
        Normal,
        InLineComment,
        InBlockComment,
        InString,
        InStringEscape,
    }
    let mut ans = String::new();
    let mut state = State::Normal;
    let mut chars = java_code.chars().peekable();
    let mut line_start = true;
    let mut last_whitespace = true;
    while let Some(ch) = chars.next() {
        match state {
            State::Normal => match ch {
                ' ' => {
                    if !last_whitespace && !line_start {
                        ans.push(' ');
                    }
                    last_whitespace = true;
                }
                '\n' => {}
                c if c == java_code::END_LINE_DELIMITER => {
                    if !last_whitespace {
                        ans.push(' ');
                    }
                    ans.push(java_code::END_LINE_DELIMITER);
                    ans.push('\n');
                    line_start = true;
                }
                c if java_code::SEPARATORS.contains(&c) => {
                    if !line_start {
                        ans.push('\n');
                    }
                    ans.push(ch);
                    ans.push('\n');
                    line_start = true;
                }
                '"' => {
                    ans.push('"');
                    state = State::InString;
                }
                '\'' => {
                    ans.push('\'');
                    let inside_char = chars.next().unwrap();
                    ans.push(inside_char);
                    if inside_char == '\\' {
                        ans.push(chars.next().unwrap());
                    }
                    ans.push('\'');
                    chars.next();
                }
                '/' => match chars.peek() {
                    Some('/') => {
                        state = State::InLineComment;
                        chars.next();
                    }
                    Some('*') => {
                        state = State::InBlockComment;
                        chars.next();
                    }
                    None => {}
                    _ => ans.push(ch),
                },
                '.' => {
                    if let Some(ch) = chars.peek()
                        && !ch.is_numeric()
                    {
                        if !last_whitespace {
                            ans.push(' ');
                        }
                        ans.push('.');
                        ans.push(' ');
                        last_whitespace = true;
                    } else {
                        ans.push('.')
                    }
                    line_start = false;
                }
                ch if ch.is_alphanumeric() => {
                    ans.push(ch);
                    line_start = false;
                    last_whitespace = false;
                }
                ch if java_code::operator_char_check(ch) => {
                    if !last_whitespace {
                        ans.push(' ');
                    }
                    let mut temp_operator = String::from(ch);
                    while let Some(&a) = chars.peek() {
                        temp_operator.push(a);
                        if java_code::operator_iterable_check(&temp_operator) {
                            chars.next();
                        } else {
                            temp_operator.pop();
                            break;
                        }
                    }
                    ans.push_str(&temp_operator);
                    ans.push(' ');
                    last_whitespace = true;
                }
                _ => {
                    ans.push(ch);
                    line_start = false;
                    last_whitespace = false;
                }
            },
            State::InLineComment => {
                if ch == '\n' {
                    state = State::Normal;
                }
            }
            State::InBlockComment => {
                if ch == '*'
                    && let Some(&'/') = chars.peek()
                {
                    state = State::Normal;
                    chars.next();
                }
            }
            State::InString => {
                if ch == '\\' {
                    state = State::InStringEscape;
                } else if ch == '"' {
                    state = State::Normal;
                    ans.push(ch);
                    ans.push(' ');
                    last_whitespace = true;
                } else {
                    ans.push(ch);
                }
            }
            State::InStringEscape => {
                ans.push(ch);
                state = State::InString;
            }
        }
    }
    ans
}
