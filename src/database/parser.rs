use crate::database::query::Identifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Literal(String),
    Operator(String),
    Whitespace,
    Comma,
    LeftParenthesis,
    RightParenthesis,
    Eof,
}

pub enum ASTNode {
    SelectStatement {
        projection: Vec<Identifier>,
        table: Identifier,
    },
    Identifier(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(token: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> ASTNode {
        self.parse_select_statement()
    }

    pub fn parse_select_statement(&mut self) -> ASTNode {

    }

    fn next_token(&mut self) -> Token {
        let token = self.tokens.get(self.current).cloned().unwrap_or(Token::Eof);
    }
}
























