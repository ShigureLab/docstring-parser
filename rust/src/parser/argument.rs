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

pub fn parse_arg<'a>(input: &'a mut Cursor, ctx: &mut Context) -> ParseResult<Argument> {
    let name = identifier(input, ctx)?;

    input.eat_whitespace();
    let mut typ: Option<String> = None;
    if let Some(_c @ '(') = input.peek() {
        input.eat_string("(")?;
        typ = Some(input.take_until(|c| c == ')').to_string());
        input.eat_string(")")?;
        input.eat_whitespace();
    }
    input.eat_string(":")?;
    input.eat_whitespace();
    let desc_head = next_line(input, ctx)?;
    let mut desc: Vec<String> = vec![desc_head];
    let next_indent_size = calc_indent_size(input.peek_while(is_indent_char));
    if next_indent_size > ctx.indent {
        desc.extend(indented_paragraph(input, ctx)?)
    }
    Ok(Argument { name, typ, desc })
}

pub fn parse_args<'a>(
    input: &'a mut Cursor,
    ctx: &mut Context,
) -> Result<Vec<Argument>, ParseError> {
    let mut args: Vec<Argument> = vec![];
    input.eat_empty_lines();
    loop {
        let next_indent_size = calc_indent_size(input.peek_while(is_indent_char));
        if next_indent_size <= ctx.indent {
            break Ok(args);
        }

        let ctx_guard = ctx.guard(Context::new(next_indent_size));
        ctx.indent = next_indent_size;
        input.eat_indent();
        args.push(parse_arg(input, ctx)?);
        ctx_guard.restore(ctx);
        input.eat_empty_lines();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arg() {
        let doc = "arg1 (int): Description of arg1";
        let mut cursor = Cursor::new(doc);
        let mut ctx = Context::new(4);
        assert_eq!(
            parse_arg(&mut cursor, &mut ctx),
            Ok(Argument {
                name: "arg1".to_string(),
                typ: Some("int".to_string()),
                desc: vec!["Description of arg1".to_string(),]
            })
        );
    }

    #[test]
    fn test_parse_arg_no_type() {
        let doc = "arg1: Description of arg1\n";
        let mut cursor = Cursor::new(doc);
        let mut ctx = Context::new(4);
        assert_eq!(
            parse_arg(&mut cursor, &mut ctx),
            Ok(Argument {
                name: "arg1".to_string(),
                typ: None,
                desc: vec!["Description of arg1".to_string(),]
            })
        );
    }

    #[test]
    fn test_parse_arg_multi_line() {
        let doc = "arg1 (int): Description of arg1\n    multi line\n    description\n";
        let mut cursor = Cursor::new(doc);
        let mut ctx = Context::new(0);
        assert_eq!(
            parse_arg(&mut cursor, &mut ctx),
            Ok(Argument {
                name: "arg1".to_string(),
                typ: Some("int".to_string()),
                desc: vec![
                    "Description of arg1".to_string(),
                    "multi line".to_string(),
                    "description".to_string(),
                ]
            })
        );
    }

    #[test]
    fn test_parse_args() {
        let doc = "    arg1 (int): Description of arg1\n";
        let mut cursor = Cursor::new(doc);
        let mut ctx = Context::new(0);
        assert_eq!(
            parse_args(&mut cursor, &mut ctx),
            Ok(vec![Argument {
                name: "arg1".to_string(),
                typ: Some("int".to_string()),
                desc: vec!["Description of arg1".to_string(),]
            }])
        );
    }

    #[test]
    fn test_parse_args_multi_line() {
        let doc = "    arg1 (int): Description of arg1\n        multi line\n        description\n";
        let mut cursor = Cursor::new(doc);
        let mut ctx = Context::new(0);
        assert_eq!(
            parse_args(&mut cursor, &mut ctx),
            Ok(vec![Argument {
                name: "arg1".to_string(),
                typ: Some("int".to_string()),
                desc: vec![
                    "Description of arg1".to_string(),
                    "multi line".to_string(),
                    "description".to_string(),
                ]
            }])
        );
    }

    #[test]
    fn test_parse_args_multi_args() {
        let doc = "    arg1 (int): Description of arg1\n    arg2 (str): Description of arg2\n";
        let mut cursor = Cursor::new(doc);
        let mut ctx = Context::new(0);
        assert_eq!(
            parse_args(&mut cursor, &mut ctx),
            Ok(vec![
                Argument {
                    name: "arg1".to_string(),
                    typ: Some("int".to_string()),
                    desc: vec!["Description of arg1".to_string(),]
                },
                Argument {
                    name: "arg2".to_string(),
                    typ: Some("str".to_string()),
                    desc: vec!["Description of arg2".to_string(),]
                }
            ])
        );
    }

    #[test]
    fn test_parse_args_multi_args_with_blank_line() {
        let doc = indent(
            &cleandoc(
                "
            arg1 (int): Description of arg1
                multi line

                description


            arg2 (str): Description of arg2
                multi line
                description",
                None,
            ),
            "    ",
        );
        let mut cursor = Cursor::new(&doc);
        let mut ctx = Context::new(0);
        assert_eq!(
            parse_args(&mut cursor, &mut ctx),
            Ok(vec![
                Argument {
                    name: "arg1".to_string(),
                    typ: Some("int".to_string()),
                    desc: vec![
                        "Description of arg1".to_string(),
                        "multi line".to_string(),
                        "".to_string(),
                        "description".to_string(),
                    ]
                },
                Argument {
                    name: "arg2".to_string(),
                    typ: Some("str".to_string()),
                    desc: vec![
                        "Description of arg2".to_string(),
                        "multi line".to_string(),
                        "description".to_string(),
                    ]
                }
            ])
        );
    }
}
