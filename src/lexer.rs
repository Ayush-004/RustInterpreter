use crate::token::Token;

struct Lexer{
    input: Vec<char>,
}
impl Lexer{
    pub fn new(input: &str) -> Lexer{
        todo!();
    }
    pub fn next_token()->Token{
        todo!()
    }
}
#[cfg(test)]
mod test{
    use crate::token::{Token,TokenKind};

    use super::Lexer;

    #[test]
    fn test_next_token(){
        let input = "=+(){},;";
        let expected: Vec<Token>=vec![
            Token{
                kind: TokenKind::Assign,
                literal: "=".to_string(),//simply "=" is a reference to a string, hence we used .to_string()
            },
                Token{
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token{
                kind: TokenKind::Lparen,
                literal: "(".to_string(),
            },
            Token{
                kind: TokenKind::Rparen,
                literal: ")".to_string(),
            },
            Token{
                kind: TokenKind::Lbrace,
                literal: "{".to_string(),
            },
            Token{
                kind: TokenKind::Rbrace,
                literal: "}".to_string(),
            },
            Token{
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token{
                kind: TokenKind::SemiColon,
                literal: ";".to_string(),
            },
            Token{
                kind: TokenKind::Eof,
                literal: "".to_string(),
            },
        ];
           let lexer = Lexer::new(input);
           for(idx,exp_token) in expected.into_iter().enumerate(){
               let recv_token=lexer.next_token();
               assert_eq!(exp_token.kind, recv_token.kind);
           }
    }
}