use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::buildins::{
    internal::{cadena, imprimir, leer, longitud, tipo, InternalFnPointer},
    member::match_member_fn,
};
use crate::parser::expression::{ExprType, Expression, FnParams};
use crate::parser::statement::{BlockStatement, Statement};
use crate::{token::TokenType, types::Numeric};

use super::{
    environment::{Environment, RcEnvironment},
    objects::{new_rc_object, Object, ResultObj},
};

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub enum Context {
    Global,
    If,
    Fn,
    Loop,
}

pub struct Evaluator {
    environment: RcEnvironment,
    buildins_internal_fn: HashMap<String, Box<dyn InternalFnPointer>>,
    stack_ctx: VecDeque<Context>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new(None))),
            buildins_internal_fn: HashMap::from([
                (
                    "longitud".to_owned(),
                    Box::new(longitud) as Box<dyn InternalFnPointer>,
                ),
                (
                    "tipo".to_owned(),
                    Box::new(tipo) as Box<dyn InternalFnPointer>,
                ),
                (
                    "imprimir".to_owned(),
                    Box::new(imprimir) as Box<dyn InternalFnPointer>,
                ),
                (
                    "leer".to_owned(),
                    Box::new(leer) as Box<dyn InternalFnPointer>,
                ),
                (
                    "cadena".to_owned(),
                    Box::new(cadena) as Box<dyn InternalFnPointer>,
                ),
            ]),
            stack_ctx: VecDeque::new(),
        }
    }

    pub fn eval_program(&mut self, statements: BlockStatement) -> ResultObj {
        self.stack_ctx.push_back(Context::Global);
        self.eval_block_statement(statements, &self.environment.clone())
    }

    // cambiarlo por un deqvec
    fn eval_block_statement(
        &mut self,
        mut program: BlockStatement,
        env: &RcEnvironment,
    ) -> ResultObj {
        // from https://github.com/Rydgel/monkey-rust/blob/master/lib/evaluator/mod.rs#L332
        match program.len() {
            // Optimizar con referencias
            0 => ResultObj::Copy(Object::Void),
            1 => self.eval_statement(program.remove(0), env),
            _ => {
                let res_obj = self.eval_statement(program.remove(0), env);
                match res_obj {
                    ResultObj::Copy(Object::Return(returned_obj)) => *returned_obj,
                    ResultObj::Copy(Object::Error(msg)) => ResultObj::Copy(Object::Error(msg)),
                    ResultObj::Copy(Object::Break) => res_obj,
                    ResultObj::Copy(Object::Continue) => ResultObj::Copy(Object::Void),
                    _ => self.eval_block_statement(program, env),
                }
            }
        }
    }

    fn eval_statement(&mut self, stmt: Statement, env: &RcEnvironment) -> ResultObj {
        match stmt {
            Statement::Var { name, value } => self.eval_var(&name, value, env),
            Statement::Return(expr, line, col) => {
                while let Some(ctx) = self.stack_ctx.pop_back() {
                    if let Context::Fn = ctx {
                        return ResultObj::Copy(Object::Return(Box::new(
                            self.eval_expression(expr, env),
                        )));
                    }
                }
                ResultObj::Copy(Object::Error(create_msg_err(
                    "Solo se puede retornar dentro de funciones".into(),
                    line,
                    col,
                )))
            }
            Statement::Continue(line, col) => {
                while let Some(ctx) = self.stack_ctx.back() {
                    if let Context::Loop = ctx {
                        return ResultObj::Copy(Object::Continue);
                    }
                    self.stack_ctx.pop_back();
                }
                ResultObj::Copy(Object::Error(create_msg_err(
                    "Solo se puede continuar en bucles".into(),
                    line,
                    col,
                )))
            }
            Statement::Break(line, col) => {
                while let Some(ctx) = self.stack_ctx.pop_back() {
                    match ctx {
                        Context::Loop => return ResultObj::Copy(Object::Break),
                        _ => continue,
                    }
                }
                ResultObj::Copy(Object::Error(create_msg_err(
                    "Solo se puede romper condicionales y bucles".into(),
                    line,
                    col,
                )))
            }
            Statement::Expression(expr) => self.eval_expression(expr, env),
            Statement::Fn {
                name,
                params,
                body,
                line,
                col,
            } => {
                let obj = ResultObj::Copy(Object::Fn {
                    name: name.clone(),
                    params,
                    body,
                    env: env.clone(),
                });
                match self.get_var_value(&name, env, line, col) {
                    Some(obj) => obj,
                    None => self.insert_obj(&name, obj, env),
                }
            }
        }
    }

    pub fn eval_expression(&mut self, expr: Expression, env: &RcEnvironment) -> ResultObj {
        match expr.r#type {
            ExprType::NumericLiteral(numeric) => ResultObj::Copy(Object::Numeric(numeric)),
            ExprType::BooleanLiteral(b) => ResultObj::Copy(Object::Boolean(b)),
            ExprType::Prefix { operator, right } => self.eval_prefix(operator, *right, env),
            ExprType::Infix {
                left,
                right,
                operator,
            } => self.eval_infix(operator, *left, *right, env),
            ExprType::If {
                condition,
                consequence,
                alternative,
            } => self.eval_if(*condition, consequence, alternative, env),
            ExprType::Identifier(ident) => self.eval_identifier(ident, env, expr.line, expr.col),
            ExprType::FnLiteral { params, body } => ResultObj::Copy(Object::FnExpr {
                params,
                body,
                env: env.clone(),
            }),
            ExprType::Call {
                function,
                arguments,
            } => self.eval_call(*function, arguments, env),
            ExprType::Assignment { left, right } => self.set_var(*left, *right, env),
            ExprType::StringLiteral(string) => {
                ResultObj::Ref(new_rc_object(Object::String(string)))
            }
            ExprType::ListLiteral { elements } => self.eval_list_literal(elements, env),
            ExprType::Index { left, index } => {
                self.eval_index_expression(*left, *index, None, env).clone()
            }
            ExprType::NullLiteral => ResultObj::Copy(Object::Null),
            ExprType::DictionaryLiteral { pairs } => self.eval_dictionary_expression(pairs, env),
            ExprType::While { condition, body } => self.eval_while_loop(*condition, body, env),
            ExprType::ForRange {
                ident,
                arguments,
                body,
            } => self.eval_for_range(ident, arguments, body, expr.line, expr.col, env),
        }
    }

    fn eval_if(
        &mut self,
        condition: Expression,
        consequence: BlockStatement,
        alternative: BlockStatement,
        env: &RcEnvironment,
    ) -> ResultObj {
        self.stack_ctx.push_back(Context::If);
        let condition = self.eval_expression(condition, env);
        let condition_res = {
            match condition {
                ResultObj::Copy(Object::Numeric(numeric)) => numeric != Numeric::Int(0),
                ResultObj::Copy(Object::Boolean(b)) => b,
                ResultObj::Copy(Object::Null) => false,
                obj => {
                    return obj;
                }
            }
        };
        let scope_env = Rc::new(RefCell::new(Environment::new(Some(env.clone()))));
        if condition_res {
            return self.eval_block_statement(consequence, &scope_env);
        }
        let obj = self.eval_block_statement(alternative, &scope_env);
        if let Some(Context::If) = self.stack_ctx.back() {
            self.stack_ctx.pop_back();
        }
        obj
    }

    fn eval_prefix(
        &mut self,
        operator: TokenType,
        right: Expression,
        env: &RcEnvironment,
    ) -> ResultObj {
        let right = self.eval_expression(right, env);
        match operator {
            TokenType::Plus => right,
            TokenType::Minus => match right {
                ResultObj::Copy(Object::Numeric(numeric)) => {
                    ResultObj::Copy(Object::Numeric(-numeric))
                }
                ResultObj::Copy(Object::Boolean(b)) => {
                    ResultObj::Copy(Object::Numeric(Numeric::Int(-(b as i64))))
                }
                _ => ResultObj::Copy(Object::Null),
            },
            TokenType::Bang => match right {
                ResultObj::Copy(Object::Numeric(int)) => {
                    ResultObj::Copy(Object::Boolean(int == Numeric::Int(0)))
                }
                ResultObj::Copy(Object::Boolean(b)) => ResultObj::Copy(Object::Boolean(!b)),
                ResultObj::Copy(Object::Null) => ResultObj::Copy(Object::Boolean(true)),
                _ => ResultObj::Copy(Object::Null),
            },
            _ => ResultObj::Copy(Object::Null),
        }
    }

    fn match_infix_ops(
        &mut self,
        left: ResultObj,
        right: ResultObj,
        operator: TokenType,
    ) -> ResultObj {
        match (left, right) {
            (ResultObj::Copy(Object::Numeric(a)), ResultObj::Copy(Object::Numeric(b))) => {
                self.eval_infix_numeric_operation(a, b, operator)
            }
            (ResultObj::Copy(Object::Numeric(a)), ResultObj::Copy(Object::Boolean(b))) => {
                self.eval_infix_numeric_operation(a, Numeric::Int(b as i64), operator)
            }
            (ResultObj::Copy(Object::Boolean(a)), ResultObj::Copy(Object::Numeric(b))) => {
                self.eval_infix_numeric_operation(Numeric::Int(a as i64), b, operator)
            }
            (ResultObj::Copy(Object::Boolean(a)), ResultObj::Copy(Object::Boolean(b))) => self
                .eval_infix_numeric_operation(
                    Numeric::Int(a as i64),
                    Numeric::Int(b as i64),
                    operator,
                ),
            // (ResultObj::Copy(Object::String(a)), ResultObj::Copy(Object::Numeric(b))) => {
            //     self.eval_infix_string_int_operation(&a, b, operator)
            // }
            // (ResultObj::Copy(Object::Numeric(a)), ResultObj::Copy(Object::String(b))) => {
            //     self.eval_infix_string_int_operation(&b, a, operator)
            // }
            (ResultObj::Ref(a), ResultObj::Ref(b)) => match (&*a.borrow(), &*b.borrow()) {
                (Object::String(a), Object::String(b)) => {
                    self.eval_infix_string_operation(a, b, operator)
                }
                (Object::List(ref a), Object::List(ref b)) => {
                    self.eval_infix_list_operation(a, b, operator)
                }
                _ => panic!("Ok, no se ocurre como llamar este error."),
            },
            (ResultObj::Copy(Object::Numeric(a)), ResultObj::Ref(b)) => match &*b.borrow() {
                Object::List(b) => self.eval_infix_list_int_operation(b, a, operator),
                Object::String(b) => self.eval_infix_string_int_operation(b, a, operator),
                _ => panic!("Ok, no se ocurre como llamar este error."),
            },
            (ResultObj::Ref(a), ResultObj::Copy(Object::Numeric(b))) => match &*a.borrow() {
                Object::List(a) => self.eval_infix_list_int_operation(a, b, operator),
                Object::String(a) => self.eval_infix_string_int_operation(a, b, operator),
                _ => panic!("Ok, no se ocurre como llamar este error."),
            },

            (ResultObj::Copy(Object::Return(a)), ResultObj::Ref(b)) => match (&*b.borrow(), *a) {
                (Object::List(b), ResultObj::Copy(Object::Numeric(a))) => {
                    self.eval_infix_list_int_operation(b, a, operator)
                }
                (Object::List(b), ResultObj::Ref(a)) => {
                    if let Object::List(a) = &*a.borrow() {
                        self.eval_infix_list_operation(a, b, operator)
                    } else {
                        panic!("Ok, no se ocurre como llamar este error.")
                    }
                }
                _ => panic!("Ok, no se ocurre como llamar este error."),
            },
            (ResultObj::Ref(b), ResultObj::Copy(Object::Return(a))) => match (&*b.borrow(), *a) {
                (Object::List(b), ResultObj::Copy(Object::Numeric(a))) => {
                    self.eval_infix_list_int_operation(b, a, operator)
                }
                (Object::List(b), ResultObj::Ref(a)) => {
                    if let Object::List(a) = &*a.borrow() {
                        self.eval_infix_list_operation(b, a, operator)
                    } else {
                        panic!("Ok, no se ocurre como llamar este error.")
                    }
                }
                _ => panic!("Ok, no se ocurre como llamar este error."),
            },
            (ResultObj::Copy(Object::Error(msg)), _) => {
                ResultObj::Copy(Object::Error("^".to_string() + &msg))
            }
            (_, ResultObj::Copy(Object::Error(msg))) => {
                ResultObj::Copy(Object::Error("^".to_string() + &msg))
            }
            (ResultObj::Copy(Object::Null), ResultObj::Copy(Object::Null)) => {
                self.eval_infix_null_operation(operator)
            }
            (ResultObj::Copy(Object::Null), _) => self.eval_infix_null_object_operation(operator),
            (_, ResultObj::Copy(Object::Null)) => self.eval_infix_null_object_operation(operator),
            (ResultObj::Copy(Object::Return(a)), b) => self.match_infix_ops(*a, b, operator),
            (a, ResultObj::Copy(Object::Return(b))) => self.match_infix_ops(a, *b, operator),
            (a, b) => ResultObj::Copy(Object::Error(format!(
                "No se soporta operaciones {} {} {}",
                self.get_type(&a),
                operator,
                self.get_type(&b)
            ))),
        }
    }

    fn eval_member_ops(
        &mut self,
        right: Expression,
        left: ResultObj,
        left_line: usize,
        left_col: usize,
        env: &RcEnvironment,
    ) -> ResultObj {
        match right.r#type {
            ExprType::Call {
                function,
                arguments,
            } => match function.r#type {
                ExprType::Identifier(identifier) => {
                    if self.is_error(&left) {
                        return left;
                    }
                    match_member_fn(self, identifier, arguments, left, left_line, left_col, env)
                }
                _ => ResultObj::Copy(Object::Error(create_msg_err(
                    "El operador de acceso de miembro espera un identicador o llamada".into(),
                    left_line,
                    left_col,
                ))),
            },
            _ => ResultObj::Copy(Object::Error(create_msg_err(
                format!(
                    "El operador de acceso de miembro no puede operar con una expresion {}",
                    right.r#type.get_type()
                ),
                right.line,
                right.col,
            ))),
        }
    }

    fn eval_infix(
        &mut self,
        operator: TokenType,
        left: Expression,
        right: Expression,
        env: &RcEnvironment,
    ) -> ResultObj {
        let line = left.line;
        let col = left.col;
        let left = self.eval_expression(left, env);

        if operator == TokenType::Dot {
            return self.eval_member_ops(right, left, line, col, env);
        }

        let right = self.eval_expression(right, env);

        // match err
        match self.match_infix_ops(left, right, operator) {
            ResultObj::Copy(Object::Error(err)) => {
                ResultObj::Copy(Object::Error(create_msg_err(err, line, col)))
            }
            obj => obj,
        }
    }

    fn get_type(&self, obj: &ResultObj) -> String {
        match obj {
            ResultObj::Copy(obj) => obj.get_type().to_string(),
            ResultObj::Ref(obj) => obj.borrow().get_type().to_string(),
        }
    }

    fn eval_infix_numeric_operation(&self, a: Numeric, b: Numeric, op: TokenType) -> ResultObj {
        // TODO: Hay que ver si hay overflow!!! y mandar un mensaje de error adecuado
        match op {
            TokenType::Plus => ResultObj::Copy(Object::Numeric(a + b)),
            TokenType::Minus => ResultObj::Copy(Object::Numeric(a - b)),
            TokenType::Slash => ResultObj::Copy(Object::Numeric(a / b)),
            TokenType::Asterisk => ResultObj::Copy(Object::Numeric(a * b)),
            TokenType::Percnt => ResultObj::Copy(Object::Numeric(a % b)),
            TokenType::Eq => ResultObj::Copy(Object::Boolean(a == b)),
            TokenType::NotEq => ResultObj::Copy(Object::Boolean(a != b)),
            TokenType::Lt => ResultObj::Copy(Object::Boolean(a < b)),
            TokenType::Gt => ResultObj::Copy(Object::Boolean(a > b)),
            TokenType::LtEq => ResultObj::Copy(Object::Boolean(a <= b)),
            TokenType::GtEq => ResultObj::Copy(Object::Boolean(a >= b)),
            _ => ResultObj::Copy(Object::Null),
        }
    }

    fn eval_infix_string_operation(&self, a: &String, b: &String, op: TokenType) -> ResultObj {
        match op {
            TokenType::Plus => ResultObj::Ref(new_rc_object(Object::String(format!("{}{}", a, b)))),
            TokenType::Eq => ResultObj::Copy(Object::Boolean(a == b)),
            TokenType::NotEq => ResultObj::Copy(Object::Boolean(a != b)),
            _ => ResultObj::Copy(Object::Null),
        }
    }

    fn eval_infix_string_int_operation(&self, a: &str, b: Numeric, op: TokenType) -> ResultObj {
        if let Numeric::Int(int) = b {
            return match op {
                TokenType::Asterisk => {
                    ResultObj::Ref(new_rc_object(Object::String(a.repeat(int as usize))))
                }
                _ => ResultObj::Copy(Object::Null),
            };
        }
        ResultObj::Copy(Object::Error(
            "No se puede hacer operaciones de indexacion con numeros flotantes".to_owned(),
        ))
    }

    fn eval_infix_list_operation(
        &self,
        a: &Vec<ResultObj>,
        b: &Vec<ResultObj>,
        op: TokenType,
    ) -> ResultObj {
        match op {
            TokenType::Plus => ResultObj::Ref(new_rc_object(Object::List(
                [a.as_slice(), b.as_slice()].concat(),
            ))),
            TokenType::Eq => ResultObj::Copy(Object::Boolean(a == b)),
            TokenType::NotEq => ResultObj::Copy(Object::Boolean(a != b)),
            TokenType::Lt => ResultObj::Copy(Object::Boolean(a.len() < b.len())),
            TokenType::Gt => ResultObj::Copy(Object::Boolean(a.len() > b.len())),
            TokenType::LtEq => ResultObj::Copy(Object::Boolean(a.len() <= b.len())),
            TokenType::GtEq => ResultObj::Copy(Object::Boolean(a.len() >= b.len())),
            _ => ResultObj::Copy(Object::Null),
        }
    }

    fn eval_infix_list_int_operation(
        &self,
        a: &Vec<ResultObj>,
        b: Numeric,
        op: TokenType,
    ) -> ResultObj {
        if let Numeric::Int(int) = b {
            match op {
                TokenType::Asterisk => {
                    let mut objs = Vec::new();
                    objs.reserve(int as usize);
                    for _ in 0..int {
                        objs.extend(a.to_owned());
                    }
                    return ResultObj::Ref(new_rc_object(Object::List(objs)));
                }
                _ => return ResultObj::Copy(Object::Null),
            };
        }
        ResultObj::Copy(Object::Error(
            "No se puede hacer operaciones con numeros flotantes en listas".to_owned(),
        ))
    }

    fn eval_infix_null_operation(&self, operator: TokenType) -> ResultObj {
        match operator {
            TokenType::Eq => ResultObj::Copy(Object::Boolean(true)),
            TokenType::NotEq => ResultObj::Copy(Object::Boolean(false)),
            _ => ResultObj::Copy(Object::Error(
                "El objeto nulo solo puede hacer operacciones logicas de igualdad".to_owned(),
            )),
        }
    }

    fn eval_infix_null_object_operation(&self, operator: TokenType) -> ResultObj {
        match operator {
            TokenType::Eq => ResultObj::Copy(Object::Boolean(false)),
            TokenType::NotEq => ResultObj::Copy(Object::Boolean(true)),
            _ => ResultObj::Copy(Object::Error(
                "El objeto nulo solo puede hacer operacciones logicas de igualdad".to_owned(),
            )),
        }
    }

    fn eval_var(&mut self, name: &String, value: Expression, env: &RcEnvironment) -> ResultObj {
        if let Some(obj) = self.get_var_value(name, env, value.line, value.col) {
            return obj;
        }

        self.insert_var(name, value, env)
    }

    fn set_var(&mut self, left: Expression, right: Expression, env: &RcEnvironment) -> ResultObj {
        return match &left.r#type {
            ExprType::Identifier(ident) => {
                if !self.exist_var(ident, env) {
                    return ResultObj::Copy(Object::Error(create_msg_err(
                        format!("El no existe referencias hacia `{}`", ident),
                        left.line,
                        left.col,
                    )));
                }

                let obj = self.eval_expression(right, env);
                let mut env_ref = RefCell::borrow_mut(env);

                env_ref.update(ident, obj.clone());
                obj
            }
            ExprType::Index { left, index } => {
                let right_obj = self.eval_expression(right, env);
                if self.is_error(&right_obj) {
                    return right_obj;
                }

                self.eval_index_expression(
                    *left.to_owned(),
                    *index.to_owned(),
                    Some(right_obj),
                    env,
                )
            }
            _ => ResultObj::Copy(Object::Error(create_msg_err(
                format!(
                    "No se puede realizar operaciones de asignacion a {}",
                    left.r#type
                ),
                left.line,
                left.col,
            ))),
        };
    }

    fn get_var_value(
        &self,
        name: &String,
        env: &RcEnvironment,
        line: usize,
        col: usize,
    ) -> Option<ResultObj> {
        let env_ref = RefCell::borrow(env);
        env_ref.get(name).map(|_| -> ResultObj {
            ResultObj::Copy(Object::Error(create_msg_err(
                format!("El identificador `{}` ya habia sido declarado", name),
                line,
                col,
            )))
        })
    }

    fn exist_var(&self, name: &String, env: &RcEnvironment) -> bool {
        let env_ref = RefCell::borrow(env);
        env_ref.exist(name)
    }

    fn insert_var(&mut self, name: &str, value: Expression, env: &RcEnvironment) -> ResultObj {
        let line = value.line;
        let col = value.col;
        let mut value_obj = self.eval_expression(value, env);
        if let ResultObj::Copy(ref obj) = value_obj {
            match obj {
                Object::Error(_) => return value_obj,
                Object::Return(ref returned_obj) => value_obj = *returned_obj.clone(),
                Object::Void => {
                    return ResultObj::Copy(Object::Error(create_msg_err(
                        "No se puede asignar el tipo de dato vacio a una variable".to_owned(),
                        line,
                        col,
                    )));
                }
                _ => {}
            }
        }
        self.insert_obj(name, value_obj, env)
    }

    fn insert_obj(&mut self, name: &str, obj: ResultObj, env: &RcEnvironment) -> ResultObj {
        let mut env_ref = RefCell::borrow_mut(env);
        env_ref.set(name.to_owned(), obj.clone());
        obj
    }

    fn eval_identifier(
        &mut self,
        ident: String,
        env: &RcEnvironment,
        line: usize,
        col: usize,
    ) -> ResultObj {
        match env.borrow().get(&ident) {
            Some(obj) => obj,
            None => {
                if let Some(func) = self.buildins_internal_fn.get(&ident) {
                    return ResultObj::Copy(Object::BuildinFn {
                        name: ident,
                        func: func.clone_box(),
                    });
                }
                ResultObj::Copy(Object::Error(create_msg_err(
                    format!("El identicador `{}` no existe.", ident),
                    line,
                    col,
                )))
            }
        }
    }

    pub fn eval_call(
        &mut self,
        function: Expression,
        arguments: Vec<Expression>,
        env: &RcEnvironment,
    ) -> ResultObj {
        let line = function.line;
        let col = function.col;
        let obj = self.eval_expression(function, env);
        match obj {
            ResultObj::Copy(Object::FnExpr { params, body, env }) => {
                self.eval_fn_expr(arguments, params, body, &env, line, col)
            }
            ResultObj::Copy(Object::Fn {
                params, body, env, ..
            }) => self.eval_fn_expr(arguments, params, body, &env, line, col),
            ResultObj::Copy(Object::BuildinFn { func, .. }) => func(self, arguments, env),
            // TODO(Retornar errores previo)
            _ => ResultObj::Copy(Object::Error(create_msg_err(
                "La operacion de llamada solo puede ser aplicada a objetos que sean funciones"
                    .to_owned(),
                line,
                col,
            ))),
        }
    }

    pub fn eval_fn_expr(
        &mut self,
        arguments: FnParams,
        params: FnParams,
        body: BlockStatement,
        env: &RcEnvironment,
        line: usize,
        col: usize,
    ) -> ResultObj {
        self.stack_ctx.push_back(Context::Fn);
        let scope_env = Rc::new(RefCell::new(Environment::new(Some(env.clone()))));
        if arguments.len() != params.len() {
            return ResultObj::Copy(Object::Error(create_msg_err(
                format!(
                    "Se encontro {} argumentos, de {}.",
                    arguments.len(),
                    params.len()
                ),
                line,
                col,
            )));
        }
        for (arg, param) in arguments.iter().zip(params) {
            if let ExprType::Identifier(param_name) = param.r#type {
                self.insert_var(&param_name, arg.clone(), &scope_env);
            }
        }
        let res_obj = self.eval_block_statement(body, &scope_env);
        if let Some(Context::Fn) = self.stack_ctx.back() {
            self.stack_ctx.pop_back();
        }
        res_obj
    }

    fn eval_list_literal(&mut self, elements: Vec<Expression>, env: &RcEnvironment) -> ResultObj {
        let mut objs = Vec::new();
        for expr in elements {
            let obj = self.eval_expression(expr, env);
            if self.is_error(&obj) {
                return obj;
            }
            objs.push(obj);
        }
        ResultObj::Ref(new_rc_object(Object::List(objs)))
    }

    fn eval_index_expression(
        &mut self,
        left: Expression,
        index: Expression,
        new_value: Option<ResultObj>,
        env: &RcEnvironment,
    ) -> ResultObj {
        let line = left.line;
        let col = left.col;
        let index_line = index.line;
        let index_col = index.col;
        let left_obj = self.eval_expression(left, env);
        let index_obj = self.eval_expression(index, env);
        if self.is_error(&index_obj) {
            return index_obj;
        }
        match left_obj {
            ResultObj::Copy(obj) => match obj {
                Object::Error(msg) => ResultObj::Copy(Object::Error(msg)),
                _ => ResultObj::Copy(Object::Error(create_msg_err(
                    "Solo se puede usar el operador de indexar en listas y dicccionarios"
                        .to_owned(),
                    line,
                    col,
                ))),
            },
            ResultObj::Ref(obj) => match *obj.borrow_mut() {
                Object::List(ref mut objs) => {
                    if let ResultObj::Copy(Object::Numeric(Numeric::Int(index))) = index_obj {
                        if let Some(new_value) = new_value {
                            if (index as usize) < objs.len() {
                                objs[index as usize] = new_value.clone();
                                return new_value;
                            }
                            return ResultObj::Copy(Object::Null);
                        }
                        return match objs.get(index as usize) {
                            Some(obj) => obj.clone(),
                            None => ResultObj::Copy(Object::Null),
                        };
                    }
                    ResultObj::Copy(Object::Error(create_msg_err(
                        "El operador de indexar solo opera con enteros".to_owned(),
                        index_line,
                        index_col,
                    )))
                }
                Object::Dictionary(ref pairs) => match pairs.get(&index_obj) {
                    Some(obj) => obj.clone(),
                    None => ResultObj::Copy(Object::Error(create_msg_err(
                        format!("Llave invalida {}", index_obj.get_type()),
                        index_line,
                        index_col,
                    ))),
                },
                _ => ResultObj::Copy(Object::Error(create_msg_err(
                    "Solo se puede usar el operador de indexar en listas y dicccionarios"
                        .to_owned(),
                    line,
                    col,
                ))),
            },
        }
    }

    fn eval_dictionary_expression(
        &mut self,
        expr_pairs: HashMap<Expression, Expression>,
        env: &RcEnvironment,
    ) -> ResultObj {
        let mut pairs = HashMap::new();
        for (k, v) in expr_pairs {
            let obj_key = self.eval_expression(k, env);
            if self.is_error(&obj_key) {
                return obj_key;
            }
            let obj_value = self.eval_expression(v, env);
            if self.is_error(&obj_value) {
                return obj_value;
            }
            pairs.insert(obj_key, obj_value);
        }
        ResultObj::Ref(new_rc_object(Object::Dictionary(pairs)))
    }

    pub fn is_error(&mut self, obj: &ResultObj) -> bool {
        if let ResultObj::Copy(Object::Error(_)) = obj {
            return true;
        }
        false
    }

    fn eval_while_loop(
        &mut self,
        condition: Expression,
        body: BlockStatement,
        env: &RcEnvironment,
    ) -> ResultObj {
        self.stack_ctx.push_back(Context::Loop);
        let condition_ref = Rc::new(RefCell::new(condition));
        let condition_obj = self.eval_expression(condition_ref.borrow().clone(), env);
        let mut condition_res = {
            match condition_obj {
                ResultObj::Copy(Object::Numeric(numeric)) => numeric != Numeric::Int(0),
                ResultObj::Copy(Object::Boolean(b)) => b,
                ResultObj::Copy(Object::Null) => false,
                obj => {
                    return obj;
                }
            }
        };
        let body = Box::new(body);
        while condition_res {
            let scope_env = Rc::new(RefCell::new(Environment::new(Some(env.clone()))));
            let res_obj = self.eval_block_statement(*body.clone(), &scope_env);
            if self.is_error(&res_obj) {
                return res_obj;
            }
            if let ResultObj::Copy(obj) = res_obj {
                match obj {
                    Object::Break => break,
                    Object::Return(_) => return ResultObj::Copy(obj),
                    _ => {}
                }
            }
            let condition_obj = self.eval_expression(condition_ref.borrow().clone(), env);
            condition_res = {
                match condition_obj {
                    ResultObj::Copy(Object::Numeric(numeric)) => numeric != Numeric::Int(0),
                    ResultObj::Copy(Object::Boolean(b)) => b,
                    ResultObj::Copy(Object::Null) => false,
                    obj => {
                        return obj;
                    }
                }
            };
        }
        if let Some(Context::Loop) = self.stack_ctx.back() {
            self.stack_ctx.pop_back();
        }
        ResultObj::Copy(Object::Void)
    }

    fn extract_numeric_int(
        &mut self,
        expr: Expression,
        env: &RcEnvironment,
    ) -> Result<i64, ResultObj> {
        let line = expr.line;
        let col = expr.col;
        match self.eval_expression(expr, env) {
            ResultObj::Copy(obj) => match obj {
                Object::Numeric(Numeric::Int(int)) => Ok(int),
                Object::Error(err) => Err(ResultObj::Copy(Object::Error(err))),
                obj => Err(ResultObj::Copy(Object::Error(create_msg_err(
                    format!(
                        "No se soporta operaciones de rango con el tipo de dato `{}`",
                        obj.get_type()
                    ),
                    line,
                    col,
                )))),
            },
            ResultObj::Ref(obj) => {
                let obj = obj.borrow();
                Err(ResultObj::Copy(Object::Error(create_msg_err(
                    format!(
                        "No se soporta operaciones de rango con el tipo de dato `{}`",
                        obj.get_type()
                    ),
                    line,
                    col,
                ))))
            }
        }
    }

    fn eval_for_range(
        &mut self,
        ident: String,
        mut arguments: Vec<Expression>,
        body: Vec<Statement>,
        mut line: usize,
        mut col: usize,
        env: &RcEnvironment,
    ) -> ResultObj {
        if self.exist_var(&ident, env) {
            return ResultObj::Copy(Object::Error(create_msg_err(
                format!("Ya existe referencias hacia la variable `{}`", ident),
                line,
                col,
            )));
        }
        self.stack_ctx.push_back(Context::Loop);

        let iter_obj: ResultObj;
        let mut end: i64 = 0;
        let mut steps: i64 = 1;
        match arguments.len() {
            1 => {
                let expr = arguments.remove(0);
                line = expr.line;
                col = expr.col;
                iter_obj = self.eval_expression(expr, env);
                if self.is_error(&iter_obj) {
                    return iter_obj;
                }
            }
            2 => {
                let expr = arguments.remove(0);
                line = expr.line;
                col = expr.col;
                iter_obj = self.eval_expression(expr, env);
                if self.is_error(&iter_obj) {
                    return iter_obj;
                }

                let expr = arguments.remove(0);
                match self.extract_numeric_int(expr, env) {
                    Ok(int) => end = int,
                    Err(res_obj) => return res_obj,
                }
            }
            3 => {
                let expr = arguments.remove(0);
                line = expr.line;
                col = expr.col;
                iter_obj = self.eval_expression(expr, env);
                if self.is_error(&iter_obj) {
                    return iter_obj;
                }

                let expr = arguments.remove(0);
                match self.extract_numeric_int(expr, env) {
                    Ok(int) => end = int,
                    Err(res_obj) => return res_obj,
                }

                let expr = arguments.remove(0);
                match self.extract_numeric_int(expr, env) {
                    Ok(int) => steps = int,
                    Err(res_obj) => return res_obj,
                }
            }
            _ => {
                return ResultObj::Copy(Object::Error(format!(
                    "Se encontro {} argumentos de 1..3",
                    arguments.len()
                )));
            }
        }

        if end < 0 || steps < 0 {
            return ResultObj::Copy(Object::Error(create_msg_err(
                "No se puede hacer operaciones de rango con numeros negativos".into(),
                line,
                col,
            )));
        } else if steps == 0 {
            return ResultObj::Copy(Object::Error(create_msg_err(
                "Los pasos del rango debe ser mayor que 0".into(),
                line,
                col,
            )));
        }

        // let body = Rc::new(RefCell::new(body));
        let body = Box::new(body);
        match iter_obj {
            ResultObj::Copy(obj) => match obj {
                Object::Numeric(Numeric::Int(begin)) => {
                    if begin < 0 {
                        return ResultObj::Copy(Object::Error(create_msg_err(
                            "No se puede hacer operaciones de rango con numeros negativos".into(),
                            line,
                            col,
                        )));
                    }
                    let range = if end != 0 {
                        0..end as usize
                    } else {
                        0..begin as usize
                    };
                    for i in range.step_by(steps as usize) {
                        let scope_env = Rc::new(RefCell::new(Environment::new(Some(env.clone()))));
                        let elem_obj = self.eval_var(
                            &ident,
                            Expression::new(
                                ExprType::NumericLiteral(Numeric::Int(i as i64)),
                                line,
                                col + 1,
                            ),
                            &scope_env,
                        );
                        if self.is_error(&elem_obj) {
                            return elem_obj;
                        }
                        let res_obj = self.eval_block_statement(*body.clone(), &scope_env);
                        if self.is_error(&res_obj) {
                            return res_obj;
                        }
                        if let ResultObj::Copy(obj) = res_obj {
                            match obj {
                                Object::Break => break,
                                Object::Return(_) => return ResultObj::Copy(obj),
                                _ => {}
                            }
                        }
                    }
                }
                obj => {
                    return ResultObj::Copy(Object::Error(create_msg_err(
                        format!(
                            "No se soporta operaciones de rango con el tipo de dato `{}`",
                            obj.get_type()
                        ),
                        line,
                        col,
                    )))
                }
            },
            ResultObj::Ref(obj) => {
                let obj = obj.borrow_mut().to_owned();
                return ResultObj::Copy(Object::Error(create_msg_err(
                    format!(
                        "No se soporta operaciones de rango con el tipo de dato `{}`",
                        obj.get_type()
                    ),
                    line,
                    col,
                )));
            }
        }

        if let Some(Context::Loop) = self.stack_ctx.back() {
            self.stack_ctx.pop_back();
        }
        ResultObj::Copy(Object::Void)
    }
}

pub fn create_msg_err(msg: String, line: usize, col: usize) -> String {
    if msg.chars().nth(0).unwrap() == '^' {
        return msg[1..].to_string();
    }
    format!(
        "Error de ejecución. {}. Linea {}, columna {}.",
        msg, line, col
    )
}
