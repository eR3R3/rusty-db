use crate::database::query::Identifier;
use crate::error::Error;

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
            tokens: token,
            current: 0,
        }
    }

    // pub fn parse(&mut self) -> ASTNode {
    //
    // }

    pub fn parse_select_statement(&mut self) -> Result<ASTNode, Error> {
        let mut projection = Vec::new();
        let mut table: Identifier = Identifier("".to_string());
        if self.next_token() == Token::Keyword("SELECT".to_string()) {
            loop {
                match self.next_token() {
                    Token::Identifier(identifier) => projection.push(Identifier(identifier)),
                    Token::Comma => continue,
                    _ => break,
                }
            }
        } else if self.next_token() == Token::Keyword("FROM".to_string()) {
            if let Token::Identifier(identifier) = self.next_token() {
                table = Identifier(identifier);
            } else {
                return Err(Error::ParseError("More words are needed after FROM keyword".to_string()))
            }
        }
        Ok(
            ASTNode::SelectStatement {
                projection,
                table,
            }
        )
    }

    fn next_token(&mut self) -> Token {
        let token = self.tokens.get(self.current).cloned().unwrap_or(Token::Eof);
        self.current += 1;
        token
    }
}
























