
pub struct Token{
   pub kind: TokenKind,
   pub literal: String,
}
pub enum TokenKind{
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    SemiColon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let
}