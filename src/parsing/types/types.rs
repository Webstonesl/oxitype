use std::fmt::Display;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterCategory {
    Escape,
    BeginGroup,
    EndGroup,
    MathShift,
    AlignmentTab,
    EndOfLine,
    Parameter,
    Superscript,
    Subscript,
    Ignored,
    Space,
    Letter,
    Other,
    Active,
    Comment,
    Invalid,
}

pub enum Source {
    Stdin,
    File(String),
}

impl Source {
    pub fn from_path(path: &str) -> Self {
        Source::File(path.to_string())
    }
    pub fn stdin() -> Self {
        Source::Stdin
    }
    pub fn as_str(&self) -> &str {
        match self {
            Source::Stdin => "stdin",
            Source::File(path) => path,
        }
    }
    pub fn get_source(&mut self) -> Box<dyn std::io::Read> {
        match self {
            Source::Stdin => Box::new(std::io::stdin()),
            Source::File(path) => Box::new(std::fs::File::open(path).expect("Failed to open file")),
        }
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Source::Stdin => write!(f, "stdin"),
            Source::File(path) => write!(f, "{}", path),
        }
    }
}

pub struct SourcePosition {
    pub source: Source,
    pub line: usize,
    pub column: usize,
}

impl Display for SourcePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.source, self.line, self.column)
    }
}

pub struct CategoryMap {
    pub parent: Option<Box<CategoryMap>>,
    pub map: std::collections::HashMap<char, CharacterCategory>,
}

impl CategoryMap {
    pub fn new() -> Self {
        CategoryMap {
            parent: None,
            map: std::collections::HashMap::new(),
        }
    }
    pub fn child(self) -> Self {
        CategoryMap {
            parent: Some(Box::new(self)),
            map: std::collections::HashMap::new(),
        }
    }
    pub fn get(&self, c: char) -> CharacterCategory {
        match self.map.get(&c) {
            Some(category) => *category,
            None => match &self.parent {
                Some(parent) => parent.get(c),
                None => CharacterCategory::Invalid,
            },
        }
    }
    pub fn set(&mut self, c: char, category: CharacterCategory) {
        self.map.insert(c, category);
    }
}

pub enum CommandType {
    BuiltIn(fn() -> ()),
    Macro,
}
