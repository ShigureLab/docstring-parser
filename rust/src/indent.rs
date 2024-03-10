static TAB_SIZE: usize = 4;

pub fn is_indent_char(s: char) -> bool {
    s == ' ' || s == '\t'
}

pub fn calc_indent_size(s: &str) -> usize {
    s.chars()
        .map(|c| if c == '\t' { TAB_SIZE } else { 1 })
        .sum()
}
