static DEFAULT_TAB_SIZE: usize = 8;
static MAX_SIZE: usize = std::usize::MAX;

pub fn expandtabs(s: &str, tab_size: Option<usize>) -> String {
    let tab_size = tab_size.unwrap_or(DEFAULT_TAB_SIZE);
    s.replace('\t', " ".repeat(tab_size).as_str())
    // result
}

pub fn cleandoc(doc: &str, tab_size: Option<usize>) -> String {
    let doc = expandtabs(doc, tab_size);
    let mut lines: Vec<_> = doc.split('\n').collect();
    // Find minimum indentation of any non-blank lines after first line.
    let mut margin = MAX_SIZE;
    for line in lines.iter().skip(1) {
        let content = line.trim_start().len();
        if content > 0 {
            let indent_size = line.len() - content;
            margin = margin.min(indent_size);
        }
    }
    // Remove indentation.
    if !lines.is_empty() {
        lines[0] = lines[0].trim_start();
    }
    if margin < MAX_SIZE {
        for line in lines.iter_mut().skip(1) {
            *line = &line[margin.min(line.len())..];
        }
    }
    // Remove any trailing or leading blank lines.
    while !lines.is_empty() && lines.last().unwrap().is_empty() {
        lines.pop();
    }
    while !lines.is_empty() && lines.first().unwrap().is_empty() {
        lines.remove(0);
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleandoc() {
        let doc = "This is a test string";
        assert_eq!(cleandoc(doc, None), "This is a test string");
    }

    #[test]
    fn test_cleandoc_with_indent() {
        let doc = "    This is a test string\n    This is the second line";
        assert_eq!(
            cleandoc(doc, Some(4)),
            "This is a test string\nThis is the second line"
        );
    }

    #[test]
    fn test_cleandoc_with_tabs() {
        let doc = "\tThis is a test string\n\tThis is the second line";
        assert_eq!(
            cleandoc(doc, Some(4)),
            "This is a test string\nThis is the second line"
        );
    }

    #[test]
    fn test_cleandoc_complex1() {
        let doc = r#"
        This is a test string
        This is the second line
        "#;
        assert_eq!(
            cleandoc(doc, Some(4)),
            "This is a test string\nThis is the second line"
        );
    }

    #[test]
    fn test_cleandoc_complex2() {
        let doc = r#"
        The first part
            With indent


        The second part
        "#;
        assert_eq!(
            cleandoc(doc, Some(4)),
            "The first part\n    With indent\n\n\nThe second part"
        );
    }

    #[test]
    fn test_cleandoc_complex3() {
        let doc = r#"


        The first part
            With indent

        The second part

        "#;
        assert_eq!(
            cleandoc(doc, Some(4)),
            "The first part\n    With indent\n\nThe second part"
        );
    }

    #[test]
    fn test_cleandoc_with_tab() {
        let doc = "
        The first part
        \tWith indent
        The second part
        ";
        assert_eq!(
            cleandoc(doc, Some(2)),
            "The first part\n  With indent\nThe second part"
        );
    }
}
