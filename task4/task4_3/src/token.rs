use colored::Colorize;

/// Tokens
#[derive(Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum TokenType {
    //Literals
    Number(i32),
    Ident(String),

    //Keywords
    /// "int"
    Int,
    /// "Float"
    Float,
    /// "void"
    Void,
    /// "const"
    Const,
    /// "if"
    If,
    ///"Else"
    Else,
    /// "while"
    While,
    /// "break"
    Break,
    /// "continue"
    Continue,
    /// "return"
    Return,

    //Arithmetical operators
    /// =
    Assign,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Multiply,
    /// /
    Divide,
    /// %
    Modulus,

    //Relational operators
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// <
    LesserThan,
    /// >
    GreaterThan,
    /// <=
    LesserEqual,
    /// >=
    GreaterEqual,

    //Logical operators
    /// !
    Not,
    /// &&
    And,
    /// ||
    Or,

    //symbols
    /// ;
    Semicolon,
    /// ,
    Comma,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// {
    LeftBrace,
    /// }
    RightBrace,
}

impl TokenType {
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "int" => Some(Self::Int),
            "float" => Some(Self::Float),
            "void" => Some(Self::Void),
            "const" => Some(Self::Const),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "while" => Some(Self::While),
            "break" => Some(Self::Break),
            "continue" => Some(Self::Continue),
            "return" => Some(Self::Return),
            _ => None,
        }
    }
    pub fn from_single_symbol(c: char) -> Option<Self> {
        match c {
            '=' => Some(Self::Assign),
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Multiply),
            '/' => Some(Self::Divide),
            '%' => Some(Self::Modulus),
            '<' => Some(Self::LesserThan),
            '>' => Some(Self::GreaterThan),
            '!' => Some(Self::Not),
            ';' => Some(Self::Semicolon),
            ',' => Some(Self::Comma),
            '(' => Some(Self::LeftParen),
            ')' => Some(Self::RightParen),
            '[' => Some(Self::LeftBracket),
            ']' => Some(Self::RightBracket),
            '{' => Some(Self::LeftBrace),
            '}' => Some(Self::RightBrace),
            _ => None,
        }
    }
    pub fn from_double_symbol(symbol: &str) -> Option<Self> {
        match symbol {
            "==" => Some(Self::Equal),
            "!=" => Some(Self::NotEqual),
            "<=" => Some(Self::LesserEqual),
            ">=" => Some(Self::GreaterEqual),
            "&&" => Some(Self::And),
            "||" => Some(Self::Or),
            _ => None,
        }
    }
}

pub enum CharacterType {
    WhiteSpace,     // ' ','\'t'
    NewLine,        // '\n'
    NonDigit,       // 'a-z''A-Z'
    Digit,          // '0-9'
    NonAlpha(char), //todo: comment and divide
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct TokenRange {
    pub start: usize,
    pub end: usize,
}

impl TokenRange {
    pub fn from(start: usize, len: usize, line_start: usize) -> Self {
        TokenRange {
            start: start,
            end: start + len,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
    pub range: TokenRange,
}

impl Token {
    pub fn new(kind: TokenType, line: usize, range: TokenRange) -> Self {
        Self { kind, line, range }
    }
    pub fn is_terminal(&self) -> bool {
        match self.kind {
            TokenType::Number(_) | TokenType::Ident(_) => true,
            _ => false,
        }
    }
    pub fn name(&self) -> String {
        match &self.kind {
            TokenType::Number(n) => format!("{:?}-{}", self.kind, n.to_string()),
            TokenType::Ident(s) => format!("{:?}-{}", self.kind, s.clone()),
            _ => format!("{:?}", self.kind),
        }
    }
    // pub fn wrong_token(&self, expect: String) {
    //     let lstart = self.line;
    //     let errline: String = self.buf[self.line..self.end].iter().collect();

    //     //Error message
    //     println!(
    //         "{}: {}",
    //         "parser error".red().bold(),
    //         "Unexpected token".bold()
    //     );
    //     println!(
    //         "  {}:{}:{}",
    //         "-->".blue().bold(),
    //         self.lineno,
    //         self.start - lstart + 1
    //     );
    //     println!("   {}", "|".blue().bold());
    //     println!(
    //         "{:3}{} {}",
    //         self.lineno.to_string().blue().bold(),
    //         "|".blue().bold(),
    //         errline
    //     );

    //     //Suggestion message
    //     print!("   {}", "|".blue().bold());
    //     for _ in 0..self.start - lstart + 1 {
    //         print!("{}", ' ');
    //     }
    //     println!(
    //         "{} {}{}",
    //         "^".red().bold(),
    //         "Expect ".red().bold(),
    //         expect.red().bold()
    //     );

    //     println!("   {}", "|".blue().bold());
    //     println!("Unexpected token");
    // }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token{{  type:{:?}  start:{:}  end:{:}  lineno:{:}  }}",
            self.kind, self.range.start, self.range.end, self.line
        )
    }
}
