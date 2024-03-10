use crate::context::Context;
use crate::cursor::Cursor;
use crate::error::ParseError;

use crate::guard::Guard;
use crate::indent::{calc_indent_size, is_indent_char};
use crate::parser::common::next_line;
use crate::parser::common::{identifier, indented_paragraph, ParseResult};
use crate::schema::Argument;
use crate::utils::cleandoc;
use textwrap::indent;

pub fn parse_plain_paragraph(input: &mut Cursor, ctx: &mut Context) -> ParseResult<Vec<String>> {
    let paragraph = indented_paragraph(input, ctx)?;
    Ok(paragraph)
}
