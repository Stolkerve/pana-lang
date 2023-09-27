// use std::collections::HashMap;

// use crate::{
//     lexer::Lexer,
//     parser::{expression::Expression, statement::Statement, Parser},
//     token::TokenType,
//     types::Numeric,
// };

// fn create_parser(input: &str) -> Parser {
//     let lexer = Lexer::new(input.to_owned());
//     Parser::new(lexer)
// }

// fn assert_parser_err(parser: &Parser) {
//     assert!(parser.error.is_none());
// }

// fn cmp_stmts_expr(stmts: Vec<Statement>, test_exprs: Vec<Expression>) {
//     assert_eq!(
//         test_exprs.len(),
//         stmts.len(),
//         "La longitud de test_exprs != al de stmts"
//     );

//     for (test_expr, stmt) in test_exprs.iter().zip(stmts) {
//         if let Statement::Expression(expr) = stmt {
//             assert_eq!(expr, *test_expr)
//         } else {
//             panic!("El Statement {:?} no es un Statement::Expression", stmt)
//         }
//     }
// }

// fn cmp_stmts_var(stmts: Vec<Statement>, test_stmts: Vec<Statement>) {
//     assert_eq!(
//         test_stmts.len(),
//         stmts.len(),
//         "La longitud de test_exprs != al de stmts"
//     );

//     for (test_stmt, stmt) in test_stmts.iter().zip(stmts) {
//         if let Statement::Var { name, value } = stmt {
//             if let Statement::Var {
//                 name: name_test,
//                 value: value_test,
//             } = test_stmt
//             {
//                 assert_eq!(name, *name_test);
//                 assert_eq!(value, *value_test);
//             } else {
//                 panic!(
//                     "El Statement {:?} no es un Statement::Expression",
//                     test_stmts
//                 )
//             }
//         } else {
//             panic!("El Statement {:?} no es un Statement::Expression", stmt)
//         }
//     }
// }

// #[test]
// fn literals() {
//     let input = r#"
//         10;
//         123.456;
//         "hola";
//         nulo;
//         [0, 2, ""];
//         {"lunes": 0};
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);

//     let test_exprs = vec![
//         Expression::NumericLiteral(Numeric::Int(10)),
//         Expression::NumericLiteral(Numeric::Float(123.456)),
//         Expression::StringLiteral("hola".to_owned()),
//         Expression::NullLiteral,
//         Expression::ListLiteral {
//             elements: vec![
//                 Expression::NumericLiteral(Numeric::Int(0)),
//                 Expression::NumericLiteral(Numeric::Int(2)),
//                 Expression::StringLiteral("".to_owned()),
//             ],
//         },
//         Expression::DictionaryLiteral {
//             pairs: HashMap::from([(
//                 Expression::StringLiteral("lunes".to_owned()),
//                 Expression::NumericLiteral(Numeric::Int(0)),
//             )]),
//         },
//     ];

//     cmp_stmts_expr(stmts, test_exprs)
// }

// #[test]
// fn prefix() {
//     let input = r#"
//         +10;
//         -93;
//         -23.45;
//         !falso;
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);

//     let test_exprs = vec![
//         Expression::Prefix {
//             operator: TokenType::Plus,
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(10))),
//         },
//         Expression::Prefix {
//             operator: TokenType::Sub,
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(93))),
//         },
//         Expression::Prefix {
//             operator: TokenType::Sub,
//             right: Box::new(Expression::NumericLiteral(Numeric::Float(23.45))),
//         },
//         Expression::Prefix {
//             operator: TokenType::Not,
//             right: Box::new(Expression::BooleanLiteral(false)),
//         },
//     ];

//     cmp_stmts_expr(stmts, test_exprs)
// }

// #[test]
// fn infix() {
//     let input = r#"
//         falso == 0;
//         falso != verdad;
//         "hola" == "chao";
//         4 < 0;
//         9 > 8;
//         1 >= 1;
//         0 <= 1;
//         nulo != 2;
//         [nulo] != [2];
//         [nulo, nulo] > [falso];
//         1 + 2;
//         2 - 4;
//         4 * 4;
//         2 / 2;
//         verdad - 1;
//         "hola" + " " + "mundo";
//         [1, 2] + [3, 4];
//         "hola" * 2;
//         [8, 9] * 2;
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);

//     let test_exprs = vec![
//         Expression::Infix {
//             left: Box::new(Expression::BooleanLiteral(false)),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(0))),
//             operator: TokenType::Eq,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::BooleanLiteral(false)),
//             right: Box::new(Expression::BooleanLiteral(true)),
//             operator: TokenType::NotEq,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::StringLiteral("hola".to_owned())),
//             right: Box::new(Expression::StringLiteral("chao".to_owned())),
//             operator: TokenType::Eq,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::NumericLiteral(Numeric::Int(4))),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(0))),
//             operator: TokenType::Lt,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::NumericLiteral(Numeric::Int(9))),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(8))),
//             operator: TokenType::Gt,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::NumericLiteral(Numeric::Int(1))),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(1))),
//             operator: TokenType::GtEq,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::NumericLiteral(Numeric::Int(0))),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(1))),
//             operator: TokenType::LtEq,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::NullLiteral),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(2))),
//             operator: TokenType::NotEq,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::ListLiteral {
//                 elements: vec![Expression::NullLiteral],
//             }),
//             right: Box::new(Expression::ListLiteral {
//                 elements: vec![Expression::NumericLiteral(Numeric::Int(2))],
//             }),
//             operator: TokenType::NotEq,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::ListLiteral {
//                 elements: vec![Expression::NullLiteral, Expression::NullLiteral],
//             }),
//             right: Box::new(Expression::ListLiteral {
//                 elements: vec![Expression::BooleanLiteral(false)],
//             }),
//             operator: TokenType::Gt,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::NumericLiteral(Numeric::Int(1))),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(2))),
//             operator: TokenType::Plus,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::NumericLiteral(Numeric::Int(2))),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(4))),
//             operator: TokenType::Sub,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::NumericLiteral(Numeric::Int(4))),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(4))),
//             operator: TokenType::Mul,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::NumericLiteral(Numeric::Int(2))),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(2))),
//             operator: TokenType::Div,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::BooleanLiteral(true)),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(1))),
//             operator: TokenType::Sub,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::Infix {
//                 left: Box::new(Expression::StringLiteral("hola".to_owned())),
//                 right: Box::new(Expression::StringLiteral(" ".to_owned())),
//                 operator: TokenType::Plus,
//             }),
//             right: Box::new(Expression::StringLiteral("mundo".to_owned())),
//             operator: TokenType::Plus,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::ListLiteral {
//                 elements: vec![
//                     Expression::NumericLiteral(Numeric::Int(1)),
//                     Expression::NumericLiteral(Numeric::Int(2)),
//                 ],
//             }),
//             right: Box::new(Expression::ListLiteral {
//                 elements: vec![
//                     Expression::NumericLiteral(Numeric::Int(3)),
//                     Expression::NumericLiteral(Numeric::Int(4)),
//                 ],
//             }),
//             operator: TokenType::Plus,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::StringLiteral("hola".to_owned())),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(2))),
//             operator: TokenType::Mul,
//         },
//         Expression::Infix {
//             left: Box::new(Expression::ListLiteral {
//                 elements: vec![
//                     Expression::NumericLiteral(Numeric::Int(8)),
//                     Expression::NumericLiteral(Numeric::Int(9)),
//                 ],
//             }),
//             right: Box::new(Expression::NumericLiteral(Numeric::Int(2))),
//             operator: TokenType::Mul,
//         },
//     ];

//     cmp_stmts_expr(stmts, test_exprs)
// }

// #[test]
// fn var_statement() {
//     let input = r#"
//         var hola_mundo = "Hola mundo!";
//         var numero = 2;
//         var hola_mundo2 = hola_mundo * numero;
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);

//     let test_stmts = vec![
//         Statement::Var {
//             name: "hola_mundo".to_owned(),
//             value: Expression::StringLiteral("Hola mundo!".to_owned()),
//         },
//         Statement::Var {
//             name: "numero".to_owned(),
//             value: Expression::NumericLiteral(Numeric::Int(2)),
//         },
//         Statement::Var {
//             name: "hola_mundo2".to_owned(),
//             value: Expression::Infix {
//                 left: Box::new(Expression::Identifier("hola_mundo".to_owned())),
//                 right: Box::new(Expression::Identifier("numero".to_owned())),
//                 operator: TokenType::Mul,
//             },
//         },
//     ];

//     cmp_stmts_var(stmts, test_stmts);
// }

// #[test]
// fn fn_literal() {
//     let input = r#"
//         var despedirse = fn(nombre) {
//             retornar "Chao " + nombre;
//         };
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);
//     assert_eq!(stmts.len(), 1);

//     let stmt = stmts.get(0).unwrap();
//     if let Statement::Var { name, value } = stmt {
//         assert_eq!(name, "despedirse");
//         if let Expression::FnLiteral { params, body } = value {
//             assert_eq!(params.len(), 1);
//             if let Expression::Identifier(ident) = params.get(0).unwrap() {
//                 assert_eq!(ident, "nombre");
//             }
//             assert_eq!(body.len(), 1);
//             if let Statement::Return(expr) = body.get(0).unwrap() {
//                 assert_eq!(
//                     *expr,
//                     Expression::Infix {
//                         left: Box::new(Expression::StringLiteral("Chao ".to_owned())),
//                         operator: TokenType::Plus,
//                         right: Box::new(Expression::Identifier("nombre".to_owned()))
//                     }
//                 )
//             }
//         }
//     }
// }

// #[test]
// fn return_statement() {
//     let input = r#"
//         retornar "Hola " + "mundo";
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);
//     assert_eq!(stmts.len(), 1);

//     let stmt = stmts.get(0).unwrap();
//     if let Statement::Return(expr) = stmt {
//         assert_eq!(
//             *expr,
//             Expression::Infix {
//                 left: Box::new(Expression::StringLiteral("Hola ".to_owned())),
//                 operator: TokenType::Plus,
//                 right: Box::new(Expression::StringLiteral("mundo".to_owned()))
//             }
//         )
//     }
// }

// #[test]
// fn fn_statement() {
//     let input = r#"
//         fn sumar(a, b) {
//             retornar a + b;
//         }
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);
//     assert_eq!(stmts.len(), 1);

//     let stmt = stmts.get(0).unwrap();
//     if let Statement::Fn { name, params, body } = stmt {
//         assert_eq!(name, "sumar");
//         assert_eq!(Expression::Identifier("a".to_owned()), *params.get(0).unwrap());
//         assert_eq!(Expression::Identifier("b".to_owned()), *params.get(1).unwrap());

//         if let Statement::Return(expr) = body.get(0).unwrap() {
//             assert_eq!(
//                 *expr,
//                 Expression::Infix {
//                     left: Box::new(Expression::Identifier("a".to_owned())),
//                     operator: TokenType::Plus,
//                     right: Box::new(Expression::Identifier("b".to_owned()))
//                 }
//             )
//         }
//     }
// }

// #[test]
// fn if_expression() {
//     let input = r#"
//         si verdad != falso {
//             var bar = ":)";
//             bar = 20 * 10;
//         }
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);
//     assert_eq!(stmts.len(), 1);

//     let stmt = stmts.get(0).unwrap();
//     if let Statement::Expression(Expression::If { condition, consequence, alternative }) = stmt {
//         if let Expression::Infix { left, right, operator } = *condition.to_owned() {
//             assert_eq!(*left.to_owned(), Expression::BooleanLiteral(true));
//             assert_eq!(*right.to_owned(), Expression::BooleanLiteral(false));
//             assert_eq!(TokenType::NotEq, operator)
//         }
//         assert_eq!(consequence.len(), 2);
//         assert_eq!(alternative.len(), 0);
//     }
// }

// #[test]
// fn if_else_expression() {
//     let input = r#"
//         si verdad != falso {
//             var bar = ":)";
//             bar = 20 * 10;
//         } sino {
//             var bar = ":(";
//         }
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);
//     assert_eq!(stmts.len(), 1);

//     let stmt = stmts.get(0).unwrap();
//     if let Statement::Expression(Expression::If { condition, consequence, alternative }) = stmt {
//         if let Expression::Infix { left, right, operator } = *condition.to_owned() {
//             assert_eq!(*left.to_owned(), Expression::BooleanLiteral(true));
//             assert_eq!(*right.to_owned(), Expression::BooleanLiteral(false));
//             assert_eq!(TokenType::NotEq, operator)
//         }
//         assert_eq!(consequence.len(), 2);
//         assert_eq!(alternative.len(), 1);
//     }
// }

// #[test]
// fn call_expression() {
//     let input = r#"
//         imprimir("hola :)");
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);
//     assert_eq!(stmts.len(), 1);

//     let stmt = stmts.get(0).unwrap();
//     if let Statement::Expression(Expression::Call { function, arguments }) = stmt {
//         if let Expression::Identifier(ident) = *function.to_owned() {
//             assert_eq!(ident, "imprimir")
//         }
//         let arg = arguments.get(0).unwrap();
//         if let Expression::StringLiteral(s) = arg {
//             assert_eq!("hola :)", s)
//         }
//     }
// }

// #[test]
// fn assignment_expression() {
//     let input = r#"
//         bar = 10;
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);
//     assert_eq!(stmts.len(), 1);

//     let stmt = stmts.get(0).unwrap();
//     if let Statement::Expression(Expression::Assignment { left, right }) = stmt {
//         if let Expression::Identifier(ident) = *left.to_owned() {
//             assert_eq!(ident, "bar")
//         }
//         if let Expression::NumericLiteral(Numeric::Int(int)) = *right.to_owned() {
//             assert_eq!(int, 10)
//         }
//     }
// }

// #[test]
// fn identifier_expression() {
//     let input = r#"
//         bar
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);
//     assert_eq!(stmts.len(), 1);

//     let stmt = stmts.get(0).unwrap();
//     if let Statement::Expression(Expression::Identifier(ident)) = stmt {
//         assert_eq!(ident, "bar")
//     }
// }

// #[test]
// fn index_expression() {
//     let input = r#"
//         [0, 1][1]
//     "#;

//     let mut parser = create_parser(input);
//     let stmts = parser.parse();
//     assert_parser_err(&parser);
//     assert_eq!(stmts.len(), 1);

//     let stmt = stmts.get(0).unwrap();
//     if let Statement::Expression(Expression::Index { left, index }) = stmt {
//         if let Expression::ListLiteral {..} = *left.to_owned() {
//         }
//         if let Expression::NumericLiteral(Numeric::Int(_)) = *index.to_owned() {
//         }
//     }
// }
