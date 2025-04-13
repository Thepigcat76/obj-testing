use super::{ast::Instruction, tokens::{Lexer, Token}};

pub struct Parser<'p> {
    lexer: Lexer<'p>,
    tok_index: usize,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        Self {
            lexer,
            tok_index: 0
        }
    }

    pub fn parse(&mut self) -> Option<Instruction> {
        match self.cur_tok()? {
            Token::Mov => self.parse_mov_ins(),
            Token::Ret => todo!(),
            Token::Add => todo!(),
            Token::Sub => todo!(),
            Token::Mul => todo!(),
            Token::Div => todo!(),
            Token::Syscall => Some(Instruction::Syscall),
            Token::Ident(_) => todo!(),
            Token::Number(_) => todo!(),
        }
    }

    fn parse_mov_ins(&mut self) -> Option<Instruction> {
        self.next_tok();

        let val = self.parse_operand();
        
        if *self.peek_tok()? == Token::Comma {

        }
        
        self.next_tok();
        
    }

    #[inline(always)]
    fn cur_tok(&self) -> Option<&Token<'p>> {
        self.lexer.tokens.get(self.tok_index)
    }

    #[inline(always)]
    fn peek_tok(&self) -> Option<&Token<'p>> {
        self.lexer.tokens.get(self.tok_index + 1)
    }

    #[inline(always)]
    fn next_tok(&mut self) {
        self.tok_index += 1;
    }

}