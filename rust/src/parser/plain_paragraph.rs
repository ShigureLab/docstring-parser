use crate::context::Context;
use crate::cursor::Cursor;
use crate::parser::common::{indented_paragraph, ParseResult};

pub fn parse_plain_paragraph(input: &mut Cursor, ctx: &mut Context) -> ParseResult<Vec<String>> {
    let paragraph = indented_paragraph(input, ctx)?;
    Ok(paragraph)
}
