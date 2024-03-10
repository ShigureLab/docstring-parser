use crate::error::ParseError;
use crate::guard::Guard;
use crate::indent::is_indent_char;

pub struct Cursor<'a> {
    pub pos: usize,
    pub input: &'a str,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor {
        Cursor { pos: 0, input }
    }

    pub fn eof(&self) -> bool {
        if self.pos > self.input.len() {
            panic!("Cursor position is out of range");
        }
        self.pos == self.input.len()
    }

    pub fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    pub fn peek_n(&self, n: usize) -> Vec<char> {
        self.input[self.pos..].chars().take(n).collect()
    }

    pub fn peek_while<F>(&self, f: F) -> &str
    where
        F: Fn(char) -> bool,
    {
        let start = self.pos;
        let end: usize = self.input[self.pos..]
            .chars()
            .take_while(|c| f(*c))
            .map(|c| c.len_utf8())
            .sum();
        &self.input[start..start + end]
    }

    pub fn skip_n(&mut self, n: usize) {
        self.pos += self.input[self.pos..]
            .chars()
            .take(n)
            .map(|c| c.len_utf8())
            .sum::<usize>();
    }

    pub fn take(&mut self) -> Option<char> {
        let mut iter = self.input[self.pos..].char_indices();
        let next = iter.next();
        if let Some((_, ch)) = next {
            self.pos += ch.len_utf8();
        }
        next.map(|(_, ch)| ch)
    }

    pub fn take_n(&mut self, n: usize) -> &str {
        let start = self.pos;
        let end: usize = self.input[self.pos..]
            .chars()
            .take(n)
            .map(|c| c.len_utf8())
            .sum();
        self.pos += end;
        &self.input[start..start + end]
    }

    pub fn take_until<F>(&mut self, f: F) -> &str
    where
        F: Fn(char) -> bool,
    {
        let invert_fn = |c| !f(c);
        self.take_while(invert_fn)
    }

    pub fn take_while<F>(&mut self, f: F) -> &str
    where
        F: Fn(char) -> bool,
    {
        let start = self.pos;
        let end: usize = self.input[self.pos..]
            .chars()
            .take_while(|c| f(*c))
            .map(|c| c.len_utf8())
            .sum();
        self.pos += end;
        &self.input[start..start + end]
    }

    pub fn take_until_match_any(&mut self, candidates: Vec<&str>) -> Option<(String, &str)> {
        let start = self.pos;
        let mut end = self.pos;
        for c in self.input[self.pos..].chars() {
            end += c.len_utf8();
            for cand in &candidates {
                if self.input[end..].starts_with(cand) {
                    self.pos = end;
                    return Some((cand.to_string().clone(), &self.input[start..end]));
                }
            }
        }
        None
    }

    pub fn take_until_match_str(&mut self, s: &str) -> Option<&str> {
        let start = self.pos;
        let end = self.input[self.pos..].find(s)?;
        self.pos += end;
        Some(&self.input[start..start + end])
    }

    pub fn take_until_dedent(&mut self, indent: usize) -> Vec<&str> {
        let mut lines = vec![];
        let mut process_line = |start: usize, end: usize| {
            let line = &self.input[start..end];
            let indent_size = line.chars().take_while(|c| is_indent_char(*c)).count();

            if line.chars().all(is_indent_char) {
                lines.push("");
            } else if indent_size > indent {
                lines.push(&line[indent..]);
            } else {
                return None;
            }
            Some(end)
        };
        // process lines (except last one)
        while let Some(end) = self.input[self.pos..].find('\n') {
            if let Some(next) = process_line(self.pos, self.pos + end) {
                self.pos = next + 1;
            } else {
                break;
            }
        }
        // process last line
        if self.pos < self.input.len() {
            if let Some(next) = process_line(self.pos, self.input.len()) {
                self.pos = next;
            }
        }
        // trim ending empty lines
        while let Some(last) = lines.last() {
            if last.is_empty() {
                lines.pop();
            } else {
                break;
            }
        }
        lines
    }

    pub fn take_remaining(&mut self) -> &str {
        let remaining = &self.input[self.pos..];
        self.pos = self.input.len();
        remaining
    }

    pub fn eat_whitespace(&mut self) {
        self.pos += self.input[self.pos..]
            .chars()
            .take_while(|c| c.is_ascii_whitespace())
            .map(|c| c.len_utf8())
            .sum::<usize>();
    }

    pub fn eat_empty_lines(&mut self) {
        while let Some(line_ending) = self.input[self.pos..].find('\n') {
            let line = &self.input[self.pos..self.pos + line_ending];
            if line.chars().all(|c| c.is_whitespace()) {
                self.pos += line_ending + 1;
            } else {
                break;
            }
        }
    }

    pub fn eat_indent(&mut self) -> usize {
        let start = self.pos;
        self.pos += self.input[self.pos..]
            .chars()
            .take_while(|c| is_indent_char(*c))
            .map(|c| c.len_utf8())
            .sum::<usize>();
        self.pos - start
    }

    pub fn eat_string(&mut self, s: &str) -> Result<(), ParseError> {
        if self.input[self.pos..].starts_with(s) {
            self.pos += s.len();
            Ok(())
        } else {
            Err(ParseError::invalid_value(
                self.pos,
                format!("Expected {}", s),
            ))
        }
    }

    pub fn guard<'b>(&'b mut self) -> CursorGuard {
        CursorGuard::new(
            self,
            Cursor {
                pos: self.pos,
                input: self.input,
            },
        )
    }
}

pub struct CursorGuard {
    pos: usize,
}

impl<'a> Guard<Cursor<'a>> for CursorGuard {
    fn new(ctx: &mut Cursor, new_ctx: Cursor) -> CursorGuard {
        let original_pos = ctx.pos;
        ctx.pos = new_ctx.pos;
        CursorGuard { pos: original_pos }
    }

    fn restore(&self, ctx: &mut Cursor) {
        ctx.pos = self.pos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor() {
        let doc = r#"This is a test string
with multiple lines

many summary in github.


Args:
    ...

Returns:
    ...
        "#;
        let mut cursor = Cursor::new(doc);
        assert_eq!(
            cursor.take_until_match_any(vec!["\nA", "\nArgs:"]),
            Some((
                "\nA".to_owned(),
                r#"This is a test string
with multiple lines

many summary in github.

"#
            ))
        )
    }

    #[test]
    fn test_take_until_end_indent1() {
        let initial_indent = 4;
        let doc = r#"    This is a test string
        Next line"#;
        let mut cursor = Cursor::new(doc);
        let lines = cursor.take_until_dedent(initial_indent);
        assert_eq!(lines, Vec::<&str>::new());
    }

    #[test]
    fn test_take_until_end_indent2() {
        let initial_indent = 4;
        let doc = r#"     This is a test string
        Next line"#;
        let mut cursor = Cursor::new(doc);
        // cursor.eat_indent();
        let lines = cursor.take_until_dedent(initial_indent);
        assert_eq!(lines, vec![" This is a test string", "    Next line"]);
    }

    #[test]
    fn test_take_until_end_indent3() {
        let initial_indent = 4;
        let doc = r#"        This is a test string
        Next line
    dedent line"#;
        let mut cursor = Cursor::new(doc);
        let lines = cursor.take_until_dedent(initial_indent);
        assert_eq!(lines, vec!["    This is a test string", "    Next line"]);
        let remaining = cursor.take_remaining();
        assert_eq!(remaining, "    dedent line")
    }
}
