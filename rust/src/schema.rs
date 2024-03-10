#[derive(PartialEq, Debug)]
pub struct Argument {
    pub name: String,
    pub typ: Option<String>,
    pub desc: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub enum DescriptionParagraph {
    Raw(String),
    Warning(String),
    Note(String),
    Args(Vec<Argument>),
    Returns(Vec<String>),
    Examples(Vec<String>),
}

pub type Docstring = Vec<DescriptionParagraph>;
