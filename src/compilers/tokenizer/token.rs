use std::collections::VecDeque;
use std::fmt;
use crate::core::Symbols;
use crate::core::value::number::Number;

#[derive(PartialEq, Clone, Copy)]
pub enum TokenType {
    Unknown,

    Number,
    String,
    Symbol,
    Identifier,
    Paren,
    Keyword,

    Annotation,
}

// --- --- --- --- --- ---

#[derive(PartialEq, Clone)]
pub enum Token {
    Number(Number),
    String(String),
    Symbol(Symbols),
    Paren(Paren),
    Id(String),
    Keyword(Keyword),

    Divider(Divider),
//    Annotation(ValueType),
}
pub type TokenVec = VecDeque<Token>;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_content_display = match self {
            Self::Number(num) => format!("Number: {}", num),
            Self::String(str) => format!("String: {}", str),
            Self::Symbol(sym) => format!("Symbol: {}", sym),
            Self::Paren(par) => format!("Paren: {:#?}", par),
            Self::Id(ide) => format!("Identifier: {}", ide),
            Self::Keyword(key) => format!("Keywords: {}", key),
            Self::Divider(div) => format!("Divider: {:?}", div),
//            Self::Annotation(type__) => format!("Annotation: {}", type__),
        };
        write!(f, "Token({})", token_content_display)
    }
}
#[derive(PartialEq, Clone, Debug)]
pub enum Divider {
    Comma,     // ','
    Colon,     // ':'
    Semicolon, // ';'
}

impl From<char> for Divider {
    fn from(value: char) -> Self {
        match value {
            ',' => Self::Comma,
            ':' => Self::Colon,
            ';' => Self::Semicolon,
            _ => unreachable!(),
        }
    }
}


#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Keyword {
    Out,

    For,
    If,

    Continue,
    Break,

    Import,

    Function,
    Class,
    New,
}

impl Keyword {
    pub fn is_keyword(word: &str) -> Option<Self> {
        // check is keyword
        let keyword: Self;

        let mut index = 0;
        while index < KEYWORD_PAIRS.len() {
            let current = KEYWORD_PAIRS[index];

            if word.eq(current.0) {
                keyword = current.1;
                return Some(keyword);
            }
            index += 1;
        }
        None
    }
}

pub const KEYWORD_PAIRS: [(&'static str, Keyword); 9] = [
    ("out", Keyword::Out),
    ("for", Keyword::For),
    ("if", Keyword::If),
    ("ctn", Keyword::Continue),
    ("brk", Keyword::Break),
    ("import", Keyword::Import),
    ("fn", Keyword::Function),
    ("cl", Keyword::Class),
    ("new", Keyword::New),
];

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Out => write!(f, "out"),
            Keyword::For => write!(f, "for"),
            Keyword::If => write!(f, "if"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::Break => write!(f, "break"),
            Keyword::Import => write!(f, "import"),
            Keyword::Function => write!(f, "function"),
            Keyword::Class => write!(f, "class"),
            Keyword::New => write!(f, "new"),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Paren {
    // ()
    LeftParen,
    RightParen,
    // []
    LeftBracket,
    RightBracket,
    // {}
    LeftBrace,
    RightBrace,
}

impl From<char> for Paren {
    fn from(value: char) -> Self {
        match value {
            '(' => Self::LeftParen,
            ')' => Self::RightParen,
            '[' => Self::LeftBracket,
            ']' => Self::RightBracket,
            '{' => Self::LeftBrace,
            '}' => Self::RightBrace,
            _ => unreachable!(),
        }
    }
}

