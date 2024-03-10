#[derive(PartialEq, Debug)]
pub enum ParseError {
    UnexpectedEnd(UnexpectedEnd),
    InvalidValue(InvalidValue),
}

#[derive(PartialEq, Debug)]
struct UnexpectedEnd {
    pos: usize,
}

#[derive(PartialEq, Debug)]
struct InvalidValue {
    pos: usize,
    message: String,
}

impl ParseError {
    pub fn invalid_value(pos: usize, message: String) -> ParseError {
        ParseError::InvalidValue(InvalidValue { pos, message })
    }

    pub fn unexpected_end(pos: usize) -> ParseError {
        ParseError::UnexpectedEnd(UnexpectedEnd { pos })
    }

    pub fn format(&self) -> String {
        match self {
            ParseError::UnexpectedEnd(e) => format!("Unexpected end at {}", e.pos),
            ParseError::InvalidValue(e) => format!("Invalid value at {}: {}", e.pos, e.message),
        }
    }
}

pub fn show_parse_error(input: &str, e: &ParseError) -> String {
    let mut res = String::new();
    let mut current_pos = 0;
    let mut line_endings = vec![];
    let mut attach_error_line = |line: &str, line_start: usize, line_end: usize| {
        match e {
            ParseError::UnexpectedEnd(ue) => {
                if line_end >= ue.pos {
                    res.push_str(&format!("{}\n", line));
                    res.push_str(&format!("{}^", " ".repeat(ue.pos - line_start - 1)));
                    res.push_str(&format!("{}\n", e.format()));
                    return Some(());
                }
            }
            ParseError::InvalidValue(iv) => {
                if line_end >= iv.pos {
                    res.push_str(&format!("{}\n", line));
                    res.push_str(&format!("{}^", " ".repeat(iv.pos - line_start - 1)));
                    res.push_str(&format!("{}\n", e.format()));
                    return Some(());
                }
            }
        }
        None
    };
    while let Some(line_ending) = input[current_pos..].find('\n') {
        let line = &input[current_pos..current_pos + line_ending];
        line_endings.push(current_pos + line_ending);
        if attach_error_line(line, current_pos, current_pos + line_ending).is_some() {
            break;
        }
        current_pos += line_ending + 1;
    }

    attach_error_line(&input[current_pos..], current_pos, input.len());
    res
}
