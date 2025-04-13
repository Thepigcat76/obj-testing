use logos::Logos;

pub struct Lexer<'l> {
    pub tokens: Vec<Token<'l>>,
}

impl<'l> Lexer<'l> {
    fn new(source: &'l str) -> Self {
        Self {
            tokens: Token::lexer(source)
                .map(|tok| match tok {
                    Ok(tok) => tok,
                    Err(_) => todo!(),
                })
                .collect(),
        }
    }
}

#[derive(Logos)]
#[logos(skip r#"(?:\/\/[^\n]*|\t|\s|\f|\n)*"#)]
pub enum Token<'t> {
    #[token("mov")]
    Mov,
    #[token("ret")]
    Ret,
    #[token("add")]
    Add,
    #[token("sub")]
    Sub,
    #[token("mul")]
    Mul,
    #[token("div")]
    Div,
    #[token("syscall")]
    Syscall,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'t str),
    #[regex(r"-?[0-9]+")]
    Number(&'t str),

    #[token(",")]
    Comma,
}
