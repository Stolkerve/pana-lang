use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
    types::Numeric,
};

fn gen_tokens(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input.to_owned());
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token.r#type.eq(&TokenType::Eof) {
            tokens.push(token);
            return tokens;
        }
        tokens.push(token);
    }
}

fn cmp_tokens_types(test_tokens: Vec<TokenType>, tokens: Vec<Token>) {
    assert_eq!(
        test_tokens.len(),
        tokens.len(),
        "La longitud de test_tokes != al de tokens"
    );
    for (test_token, token) in test_tokens.iter().zip(tokens) {
        assert_eq!(
            *test_token, token.r#type,
            "El token `{}` es distinto al token `{}`",
            test_token, token.r#type
        )
    }
}

#[test]
fn one_char_tokens() {
    let input = "@ = + - / * < > ! . , ; ( ) { } [ ] :";
    let tokens = gen_tokens(input);

    let test_tokens = vec![
        TokenType::Illegal('@'),
        TokenType::Assign,
        TokenType::Plus,
        TokenType::Sub,
        TokenType::Div,
        TokenType::Mul,
        TokenType::Lt,
        TokenType::Gt,
        TokenType::Not,
        TokenType::Dot,
        TokenType::Comma,
        TokenType::SemiColon,
        TokenType::LParen,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::RBrace,
        TokenType::LBracket,
        TokenType::RBracket,
        TokenType::Colon,
        TokenType::Eof,
    ];

    cmp_tokens_types(test_tokens, tokens)
}

#[test]
fn two_char_token() {
    let input = "== != <= >=";
    let tokens = gen_tokens(input);

    let test_tokens = vec![
        TokenType::Eq,
        TokenType::NotEq,
        TokenType::LtEq,
        TokenType::GtEq,
        TokenType::Eof,
    ];

    cmp_tokens_types(test_tokens, tokens)
}

#[test]
fn keywords() {
    let input = "var fn si sino retornar verdad falso nulo";
    let tokens = gen_tokens(input);

    let test_tokens = vec![
        TokenType::Var,
        TokenType::Func,
        TokenType::If,
        TokenType::Else,
        TokenType::Return,
        TokenType::True,
        TokenType::False,
        TokenType::Null,
        TokenType::Eof,
    ];

    cmp_tokens_types(test_tokens, tokens)
}

#[test]
fn identifiers() {
    let input = "
        hola
        foo2
        fo9o2
        bar_tender
        bar2_tender
        real_4real
        real_4real2
        one_two_three4
    ";
    let tokens = gen_tokens(input);

    let test_tokens = vec![
        TokenType::Ident("hola".to_owned()),
        TokenType::Ident("foo2".to_owned()),
        TokenType::Ident("fo9o2".to_owned()),
        TokenType::Ident("bar_tender".to_owned()),
        TokenType::Ident("bar2_tender".to_owned()),
        TokenType::Ident("real_4real".to_owned()),
        TokenType::Ident("real_4real2".to_owned()),
        TokenType::Ident("one_two_three4".to_owned()),
        TokenType::Eof,
    ];
    // println!("{:?}", tokens);

    cmp_tokens_types(test_tokens, tokens)
}

// #[test]
// fn bad_identifies() {
//     let input = "
//         2hola
//     ";
//     let tokens = gen_tokens(input);

//     let test_tokens = vec![
//         TokenType::IllegalMsg("El formato del identificador 2hola es erroneo, debe ser snake case.".to_owned()),
//         TokenType::Eof
//     ];

//     cmp_tokens_types(test_tokens, tokens)
// }

/// No se puede probar negativos por ahora,
/// pero existen, se dividen en dos tokens
#[test]
fn numerics() {
    let input = "
        10
        129920313211231
        0.123456789
        123456789.3132
        0b1111
        0xFf
        0o10
    ";
    let tokens = gen_tokens(input);

    let test_tokens = vec![
        TokenType::Numeric(Numeric::Int(10)),
        TokenType::Numeric(Numeric::Int(129920313211231)),
        TokenType::Numeric(Numeric::Float(0.123456789)),
        TokenType::Numeric(Numeric::Float(123456789.3132)),
        TokenType::Numeric(Numeric::Int(15)),
        TokenType::Numeric(Numeric::Int(255)),
        TokenType::Numeric(Numeric::Int(8)),
        TokenType::Eof,
    ];

    cmp_tokens_types(test_tokens, tokens)
}

#[test]
fn strings() {
    let input = "
        \"Hola mundo\"
    ";
    let tokens = gen_tokens(input);

    let test_tokens = vec![TokenType::String("Hola mundo".to_owned()), TokenType::Eof];

    cmp_tokens_types(test_tokens, tokens)
}

#[test]
fn illegal() {
    let input = "
        @ ~ ^
    ";
    let tokens = gen_tokens(input);

    let test_tokens = vec![
        TokenType::Illegal('@'),
        TokenType::Illegal('~'),
        TokenType::Illegal('^'),
        TokenType::Eof,
    ];

    cmp_tokens_types(test_tokens, tokens)
}

#[test]
fn code() {
    let input = r#"
        si verdad {
            var a = "hola"; # Comentario
        } sino {
            retornar [0, 2];
        }
        var b = {"lunes": 0};
        #"Hola mundo"
        imprimir(a, " ", b);

        # recursivo
        fn chao() {
            chao();
        }
    "#;
    let tokens = gen_tokens(input);

    let test_tokens = vec![
        TokenType::If,
        TokenType::True,
        TokenType::LBrace,
        TokenType::Var,
        TokenType::Ident("a".to_owned()),
        TokenType::Assign,
        TokenType::String("hola".to_owned()),
        TokenType::SemiColon,
        TokenType::CommentLine,
        TokenType::RBrace,
        TokenType::Else,
        TokenType::LBrace,
        TokenType::Return,
        TokenType::LBracket,
        TokenType::Numeric(Numeric::Int(0)),
        TokenType::Comma,
        TokenType::Numeric(Numeric::Int(2)),
        TokenType::RBracket,
        TokenType::SemiColon,
        TokenType::RBrace,
        TokenType::Var,
        TokenType::Ident("b".to_owned()),
        TokenType::Assign,
        TokenType::LBrace,
        TokenType::String("lunes".to_owned()),
        TokenType::Colon,
        TokenType::Numeric(Numeric::Int(0)),
        TokenType::RBrace,
        TokenType::SemiColon,
        TokenType::CommentLine,
        TokenType::Ident("imprimir".to_owned()),
        TokenType::LParen,
        TokenType::Ident("a".to_owned()),
        TokenType::Comma,
        TokenType::String(" ".to_owned()),
        TokenType::Comma,
        TokenType::Ident("b".to_owned()),
        TokenType::RParen,
        TokenType::SemiColon,
        TokenType::CommentLine,
        TokenType::Func,
        TokenType::Ident("chao".to_owned()),
        TokenType::LParen,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::Ident("chao".to_owned()),
        TokenType::LParen,
        TokenType::RParen,
        TokenType::SemiColon,
        TokenType::RBrace,
        TokenType::Eof,
    ];

    cmp_tokens_types(test_tokens, tokens)
}
