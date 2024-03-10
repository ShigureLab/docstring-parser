use crate::context::Context;
use crate::cursor::Cursor;
use crate::guard::Guard;
use crate::indent::{calc_indent_size, is_indent_char};
use crate::parser::argument::parse_args;
use crate::parser::common::{next_line, ParseResult};
use crate::parser::plain_paragraph::parse_plain_paragraph;
use crate::schema::{Argument, Docstring, DocstringParagraph};
use crate::utils::cleandoc;
pub enum DocstringTitle {
    Args,
    Returns,
}

impl DocstringTitle {
    pub fn args_heads() -> [&'static str; 2] {
        ["Args", "Parameters"]
    }

    pub fn returns_heads() -> [&'static str; 1] {
        ["Returns"]
    }

    pub fn yields_heads() -> [&'static str; 1] {
        ["Yields"]
    }

    pub fn raises_heads() -> [&'static str; 1] {
        ["Raises"]
    }

    pub fn examples_heads() -> [&'static str; 1] {
        ["Examples"]
    }

    pub fn notes_heads() -> [&'static str; 1] {
        ["Notes"]
    }

    pub fn references_heads() -> [&'static str; 1] {
        ["References"]
    }

    pub fn see_also_heads() -> [&'static str; 1] {
        ["See Also"]
    }

    pub fn warnings_heads() -> [&'static str; 1] {
        ["Warnings"]
    }
}

pub fn parse_docstring(input: &mut Cursor, ctx: &mut Context) -> ParseResult<Docstring> {
    let mut docstring = vec![];
    loop {
        if input.eof() {
            break;
        }

        let line = next_line(input, ctx)?;
        if DocstringTitle::args_heads()
            .iter()
            .any(|&head| line.starts_with(head))
        {
            let indent = line.chars().take_while(|c| is_indent_char(*c)).count();
            let ctx_guard = ctx.guard(Context::new(indent));
            docstring.push(DocstringParagraph::Args(parse_args(input, ctx)?));
            ctx_guard.restore(ctx);
        } else if DocstringTitle::returns_heads()
            .iter()
            .any(|&head| line.starts_with(head))
        {
            let indent = line.chars().take_while(|c| is_indent_char(*c)).count();
            let ctx_guard = ctx.guard(Context::new(indent));
            docstring.push(DocstringParagraph::Returns(parse_plain_paragraph(
                input, ctx,
            )?));
            ctx_guard.restore(ctx);
        } else if DocstringTitle::examples_heads()
            .iter()
            .any(|&head| line.starts_with(head))
        {
            let indent = line.chars().take_while(|c| is_indent_char(*c)).count();
            let ctx_guard = ctx.guard(Context::new(indent));
            docstring.push(DocstringParagraph::Examples(parse_plain_paragraph(
                input, ctx,
            )?));
            ctx_guard.restore(ctx);
        } else {
            docstring.push(DocstringParagraph::Raw(line));
        }
    }
    Ok(docstring)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_docstring() {
        let doc = cleandoc(
            "
        Args:
            arg1 (int): Description of arg1
        ",
            None,
        );
        let mut cursor = Cursor::new(&doc);
        let mut ctx = Context::new(4);
        assert_eq!(
            parse_docstring(&mut cursor, &mut ctx),
            Ok(vec![DocstringParagraph::Args(vec![Argument {
                name: "arg1".to_string(),
                r#type: Some("int".to_string()),
                desc: vec!["Description of arg1".to_string()]
            }])])
        );
    }

    #[test]
    fn test_parse_docstring_multi_line() {
        let doc = cleandoc(
            "
        Args:
            arg1 (int): Description of arg1
            arg2 (str): Description of arg2
                line 2 of arg2
                line 3 of arg2
            arg3 (float): Description of arg3
                line 2 of arg3
        ",
            None,
        );
        let mut cursor = Cursor::new(&doc);
        let mut ctx = Context::new(4);

        assert_eq!(
            parse_docstring(&mut cursor, &mut ctx),
            Ok(vec![DocstringParagraph::Args(vec![
                Argument {
                    name: "arg1".to_string(),
                    r#type: Some("int".to_string()),
                    desc: vec!["Description of arg1".to_string()]
                },
                Argument {
                    name: "arg2".to_string(),
                    r#type: Some("str".to_string()),
                    desc: vec![
                        "Description of arg2".to_string(),
                        "line 2 of arg2".to_string(),
                        "line 3 of arg2".to_string()
                    ]
                },
                Argument {
                    name: "arg3".to_string(),
                    r#type: Some("float".to_string()),
                    desc: vec![
                        "Description of arg3".to_string(),
                        "line 2 of arg3".to_string()
                    ]
                }
            ])])
        );
    }

    #[test]
    fn test_parse_docstring_with_returns() {
        let doc = cleandoc(
            "
        Args:
            arg1 (int): Description of arg1
        Returns:
            Description of return value
        ",
            None,
        );
        let mut cursor = Cursor::new(&doc);
        let mut ctx = Context::new(4);
        assert_eq!(
            parse_docstring(&mut cursor, &mut ctx),
            Ok(vec![
                DocstringParagraph::Args(vec![Argument {
                    name: "arg1".to_string(),
                    r#type: Some("int".to_string()),
                    desc: vec!["Description of arg1".to_string()]
                }]),
                DocstringParagraph::Returns(vec!["Description of return value".to_string()])
            ])
        );
    }

    #[test]
    fn test_parse_docstring_with_examples() {
        let doc = cleandoc(
            "
        Args:
            arg1 (int): Description of arg1
        Examples:

            >>> from docstring_parser import parse
            >>> parse('Args: arg1 (int): Description of arg1')
            {'Args': [{'arg1': {'type': 'int', 'description': 'Description of arg1'}}]}
        ",
            None,
        );
        let mut cursor = Cursor::new(&doc);
        let mut ctx = Context::new(4);
        assert_eq!(
            parse_docstring(&mut cursor, &mut ctx),
            Ok(vec![
                DocstringParagraph::Args(vec![Argument {
                    name: "arg1".to_string(),
                    r#type: Some("int".to_string()),
                    desc: vec!["Description of arg1".to_string()]
                }]),
                DocstringParagraph::Examples(vec![
                    "".to_string(),
                    ">>> from docstring_parser import parse".to_string(),
                    ">>> parse('Args: arg1 (int): Description of arg1')".to_string(),
                    "{'Args': [{'arg1': {'type': 'int', 'description': 'Description of arg1'}}]}"
                        .to_string()
                ])
            ])
        );
    }
}
