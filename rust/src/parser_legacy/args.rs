use winnow::ascii::{alpha1, alphanumeric1, multispace1, space0, space1};
use winnow::combinator::{alt, opt};
use winnow::combinator::{delimited, preceded, repeat, terminated};
use winnow::stream::AsChar;
use winnow::token::{literal, one_of, take_until, take_while};
use winnow::PResult;
use winnow::Parser;

#[derive(Debug, PartialEq)]
pub struct Arg {
    name: String,
    typ: Option<String>,
    desc: String,
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let typ = self
            .typ
            .as_ref()
            .map(|s| format!("({})", s))
            .unwrap_or_default();
        write!(f, "{} {}: {}", self.name, typ, self.desc)
    }
}

// fn indent(input: &mut str) -> PResult<&str> {
//     space1.parse_next(input)
// }

pub fn args2_parser<'s>(input: &mut &'s str) -> PResult<&'s str> {
    let mut args_head = terminated("\nArgs:", space0);
    // let args_body = terminated("\n", multispace1);
    // preceded(args_head, args_body).parse_next(input)
    args_head.parse_next(input)
}

pub fn identifier<'s>(input: &mut &'s str) -> PResult<&'s str> {
    (
        one_of(|c: char| c.is_alpha() || c == '_'),
        take_while(0.., |c: char| c.is_alphanum() || c == '_'),
    )
        .recognize()
        .parse_next(input)
}

pub fn arg_parser(input: &mut &str) -> PResult<Arg> {
    let arg_name = identifier;
    let arg_name = terminated(arg_name, space0);
    let arg_type = delimited("(", take_until(0.., ")"), ")");
    let arg_type = terminated(arg_type, space0);
    let arg_name_with_type = (arg_name, opt(arg_type));
    let arg_name_with_type = terminated(arg_name_with_type, literal(":"));
    let arg_name_with_type = terminated(arg_name_with_type, space0);
    let arg_desc = take_until(0.., "\n");
    let ((name, typ), desc) = (arg_name_with_type, arg_desc).parse_next(input)?;
    Ok(Arg {
        name: name.to_owned(),
        typ: typ.map(|s| s.to_owned()),
        desc: desc.to_owned(),
    })
}

pub fn args_parser(input: &mut &str) -> PResult<Arg> {
    // let output = literal("arg1").parse_next(input)?;
    // Ok(output)
    arg_parser(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_parser() {
        let mut input = "arg1 (int): The desc of arg1\n";
        let arg = arg_parser(&mut input).unwrap();
        assert_eq!(
            arg,
            Arg {
                name: "arg1".to_owned(),
                typ: Some("int".to_owned()),
                desc: "The desc of arg1".to_owned(),
            }
        )
    }
}
