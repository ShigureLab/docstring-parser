use crate::context::Context;
use crate::cursor::Cursor;
use crate::error::ParseError;
use crate::indent::is_indent_char;

pub type ParseResult<T> = Result<T, ParseError>;

pub fn identifier(input: &mut Cursor, _ctx: &mut Context) -> ParseResult<String> {
    let start_pos = input.pos;
    match input.peek() {
        Some(_c @ ('A'..='Z' | 'a'..='z' | '0'..='9' | '_')) => input.skip_n(1),
        Some(c) => {
            return Err(ParseError::invalid_value(
                start_pos,
                format!("Invalid identifier start: {}", c),
            ))
        }
        None => return Err(ParseError::unexpected_end(start_pos)),
    };
    let _ = input.take_while(|c| c.is_ascii_alphanumeric() || c == '_');
    Ok(input.input[start_pos..input.pos].to_string())
}

pub fn next_line(input: &mut Cursor, _ctx: &mut Context) -> ParseResult<String> {
    let line = input.take_until(|c| c == '\n').to_string();
    match input.peek() {
        Some('\n') => input.skip_n(1),
        None => {}
        Some(_) => {
            return Err(ParseError::invalid_value(
                input.pos,
                "Expected newline".to_string(),
            ))
        }
    }
    Ok(line)
}

pub fn indented_paragraph(input: &mut Cursor, ctx: &mut Context) -> ParseResult<Vec<String>> {
    let lines = input.take_until_dedent(ctx.indent);
    let min_indent_size_in_lines = lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().take_while(|s| is_indent_char(*s)).count())
        .min()
        .unwrap_or(0);
    let trimed_lines: Vec<String> = lines
        .iter()
        .map(|s| {
            if s.is_empty() {
                "".to_string()
            } else {
                s.chars().skip(min_indent_size_in_lines).collect()
            }
        })
        .collect();
    Ok(trimed_lines)
}
