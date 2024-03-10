#[derive(PartialEq, Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub r#type: Option<String>,
    pub desc: Vec<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum DocstringParagraph {
    Raw(String),
    Warning(String),
    Note(String),
    Args(Vec<Argument>),
    Returns(Vec<String>),
    Examples(Vec<String>),
}

pub type Docstring = Vec<DocstringParagraph>;
