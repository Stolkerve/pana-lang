use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        expressions::{Expression, FnParams},
        statements::{BlockStatement, Statement},
        Program,
    },
    buildins::{
        buildin_cadena_fn, buildin_imprimir_fn, buildin_leer_fn, buildin_longitud_fn,
        buildin_tipo_fn, BuildinFnPointer,
    },
    environment::{Environment, RcEnvironment},
    objects::{new_rc_object, Object, ResultObj},
    token::Token,
    types::Numeric,
};

#[derive(PartialEq, Clone, Debug)]
pub enum Context {
    Global,
    If,
    Fn,
}

pub struct Evaluator {
    environment: RcEnvironment,
    buildins_fn: HashMap<String, Box<dyn BuildinFnPointer>>,
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
            buildins_fn: HashMap::from([
                (
                    "longitud".to_owned(),
                    Box::new(buildin_longitud_fn) as Box<dyn BuildinFnPointer>,
                ),
                (
                    "tipo".to_owned(),
                    Box::new(buildin_tipo_fn) as Box<dyn BuildinFnPointer>,
                ),
                (
                    "imprimir".to_owned(),
                    Box::new(buildin_imprimir_fn) as Box<dyn BuildinFnPointer>,
                ),
                (
                    "leer".to_owned(),
                    Box::new(buildin_leer_fn) as Box<dyn BuildinFnPointer>,
                ),
                (
                    "cadena".to_owned(),
                    Box::new(buildin_cadena_fn) as Box<dyn BuildinFnPointer>,
                ),
            ]),
        }
    }

    pub fn eval_program(&mut self, program: Program) -> ResultObj {
        self.eval_block_statement(
            program.statements,
            &self.environment.clone(),
            &Context::Global,
        )
    }

    fn eval_block_statement(
        &mut self,
        mut program: BlockStatement,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        // from https://github.com/Rydgel/monkey-rust/blob/master/lib/evaluator/mod.rs#L332
        // TODO: Arregla eso del context
        let context: Context;
        match program.len() {
            0 => ResultObj::Borrow(Object::Void),
            1 => {
                let stmt = program.remove(0);
                context = if root_context == &Context::Global {
                    self.check_context(&stmt)
                } else {
                    root_context.clone()
                };
                self.eval_statement(stmt, env, &context)
            }
            _ => {
                let stmt = program.remove(0);
                context = if root_context == &Context::Global {
                    self.check_context(&stmt)
                } else {
                    root_context.clone()
                };
                match self.eval_statement(stmt, env, &context) {
                    ResultObj::Borrow(Object::Return(obj)) => *obj,
                    _ => self.eval_block_statement(program, env, &context),
                }
            }
        }
    }

    fn check_context(&self, stmt: &Statement) -> Context {
        match stmt {
            Statement::Expression(expr) => match expr {
                Expression::FnLiteral { .. } => Context::Fn,
                Expression::If { .. } => Context::If,
                _ => Context::Global,
            },
            Statement::Fn { .. } => Context::Fn,
            _ => Context::Global,
        }
    }

    fn eval_statement(
        &mut self,
        stmt: Statement,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        match stmt {
            Statement::Var { name, value } => self.eval_var(name, value, env, root_context),
            Statement::Return(expr) => ResultObj::Borrow(Object::Return(Box::new(
                self.eval_expression(expr, env, root_context),
            ))),
            Statement::Expression(expr) => self.eval_expression(expr, env, root_context),
            Statement::Fn { name, params, body } => {
                let obj = ResultObj::Borrow(Object::Fn {
                    name: name.clone(),
                    params,
                    body,
                    env: env.clone(),
                });
                match self.get_var_value(&name, env) {
                    Some(obj) => obj,
                    None => self.insert_obj(name, obj, env),
                }
            }
        }
    }

    pub fn eval_expression(
        &mut self,
        expr: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        match expr {
            Expression::NumericLiteral(numeric) => ResultObj::Borrow(Object::Numeric(numeric)),
            Expression::BooleanLiteral(b) => ResultObj::Borrow(Object::Boolean(b)),
            Expression::Prefix { operator, right } => {
                self.eval_prefix(operator, *right, env, root_context)
            }
            Expression::Infix {
                left,
                right,
                operator,
            } => self.eval_infix(operator, *left, *right, env, root_context),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => self.eval_if(*condition, consequence, alternative, env, root_context),
            Expression::Identifier(ident) => self.eval_identifier(ident, env),
            Expression::FnLiteral { params, body } => ResultObj::Borrow(Object::FnExpr {
                params,
                body,
                env: env.clone(),
            }),
            Expression::Call {
                function,
                arguments,
            } => self.eval_call(*function, arguments, env, root_context),
            Expression::Assignment { left, right } => {
                self.set_var(*left, *right, env, root_context)
            }
            Expression::StringLiteral(string) => ResultObj::Borrow(Object::String(string)),
            Expression::ListLiteral { elements } => {
                self.eval_list_literal(elements, env, root_context)
            }
            Expression::Index { left, index } => self
                .eval_index_expression(*left, *index, None, env, root_context)
                .clone(),
            Expression::NullLiteral => ResultObj::Borrow(Object::Null),
            Expression::DictionaryLiteral { pairs } => {
                self.eval_dictionary_expression(pairs, env, root_context)
            }
        }
    }

    fn eval_if(
        &mut self,
        condition: Expression,
        consequence: BlockStatement,
        alternative: BlockStatement,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        let condition = self.eval_expression(condition, env, root_context);
        let condition_res = {
            match condition {
                ResultObj::Borrow(Object::Numeric(numeric)) => numeric != Numeric::Int(0),
                ResultObj::Borrow(Object::Boolean(b)) => b,
                ResultObj::Borrow(Object::Null) => false,
                obj => {
                    return obj;
                }
            }
        };
        let scope_env = Rc::new(RefCell::new(Environment::new(Some(env.clone()))));
        if condition_res {
            return self.eval_block_statement(consequence, &scope_env, root_context);
        }
        self.eval_block_statement(alternative, &scope_env, root_context)
    }

    fn eval_prefix(
        &mut self,
        operator: Token,
        right: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        let right = self.eval_expression(right, env, root_context);
        match operator {
            Token::Plus => right,
            Token::Sub => match right {
                ResultObj::Borrow(Object::Numeric(numeric)) => {
                    ResultObj::Borrow(Object::Numeric(-numeric))
                }
                ResultObj::Borrow(Object::Boolean(b)) => {
                    ResultObj::Borrow(Object::Numeric(Numeric::Int(-(b as i64))))
                }
                _ => ResultObj::Borrow(Object::Null),
            },
            Token::Not => match right {
                ResultObj::Borrow(Object::Numeric(int)) => {
                    ResultObj::Borrow(Object::Boolean(int == Numeric::Int(0)))
                }
                ResultObj::Borrow(Object::Boolean(b)) => ResultObj::Borrow(Object::Boolean(!b)),
                ResultObj::Borrow(Object::Null) => ResultObj::Borrow(Object::Boolean(true)),
                _ => ResultObj::Borrow(Object::Null),
            },
            _ => ResultObj::Borrow(Object::Null),
        }
    }

    fn eval_infix(
        &mut self,
        operator: Token,
        left: Expression,
        right: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        let left = self.eval_expression(left, env, root_context);
        let right = self.eval_expression(right, env, root_context);

        match (left, right) {
            (ResultObj::Borrow(Object::Numeric(a)), ResultObj::Borrow(Object::Numeric(b))) => {
                self.eval_infix_numeric_operation(a, b, operator)
            }
            (ResultObj::Borrow(Object::Numeric(a)), ResultObj::Borrow(Object::Boolean(b))) => {
                self.eval_infix_numeric_operation(a, Numeric::Int(b as i64), operator)
            }
            (ResultObj::Borrow(Object::Boolean(a)), ResultObj::Borrow(Object::Numeric(b))) => {
                self.eval_infix_numeric_operation(Numeric::Int(a as i64), b, operator)
            }
            (ResultObj::Borrow(Object::Boolean(a)), ResultObj::Borrow(Object::Boolean(b))) => self
                .eval_infix_numeric_operation(
                    Numeric::Int(a as i64),
                    Numeric::Int(b as i64),
                    operator,
                ),
            (ResultObj::Borrow(Object::String(a)), ResultObj::Borrow(Object::String(b))) => {
                self.eval_infix_string_operation(&a, &b, operator)
            }
            (ResultObj::Borrow(Object::String(a)), ResultObj::Borrow(Object::Numeric(b))) => {
                self.eval_infix_string_int_operation(&a, b, operator)
            }
            (ResultObj::Borrow(Object::Numeric(a)), ResultObj::Borrow(Object::String(b))) => {
                self.eval_infix_string_int_operation(&b, a, operator)
            }
            (ResultObj::Ref(a), ResultObj::Ref(b)) => match (&*a.borrow(), &*b.borrow()) {
                (Object::List(ref a), Object::List(ref b)) => {
                    self.eval_infix_list_operation(a, b, operator)
                }
                _ => panic!(""),
            },
            (ResultObj::Borrow(Object::Numeric(a)), ResultObj::Ref(b)) => match &*b.borrow() {
                Object::List(b) => self.eval_infix_list_int_operation(b, a, operator),
                _ => panic!(""),
            },
            (ResultObj::Ref(a), ResultObj::Borrow(Object::Numeric(b))) => match &*a.borrow() {
                Object::List(a) => self.eval_infix_list_int_operation(a, b, operator),
                _ => panic!(""),
            },
            (ResultObj::Borrow(Object::Null), ResultObj::Borrow(Object::Null)) => {
                self.eval_infix_null_operation(operator)
            }
            (ResultObj::Borrow(Object::Null), _) => self.eval_infix_null_object_operation(operator),
            (_, ResultObj::Borrow(Object::Null)) => self.eval_infix_null_object_operation(operator),
            (ResultObj::Borrow(Object::Error(msg)), _) => ResultObj::Borrow(Object::Error(msg)),
            (_, ResultObj::Borrow(Object::Error(msg))) => ResultObj::Borrow(Object::Error(msg)),
            (a, b) => ResultObj::Borrow(Object::Error(format!(
                "No se soporta operaciones {} {} {}",
                self.get_type(&a),
                operator,
                self.get_type(&b)
            ))),
        }
    }

    fn get_type(&self, obj: &ResultObj) -> String {
        match obj {
            ResultObj::Borrow(obj) => obj.get_type().to_string(),
            ResultObj::Ref(obj) => obj.borrow().get_type().to_string(),
        }
    }

    fn eval_infix_numeric_operation(&self, a: Numeric, b: Numeric, op: Token) -> ResultObj {
        match op {
            Token::Plus => ResultObj::Borrow(Object::Numeric(a + b)),
            Token::Sub => ResultObj::Borrow(Object::Numeric(a - b)),
            Token::Div => ResultObj::Borrow(Object::Numeric(a / b)),
            Token::Mul => ResultObj::Borrow(Object::Numeric(a * b)),
            Token::Eq => ResultObj::Borrow(Object::Boolean(a == b)),
            Token::NotEq => ResultObj::Borrow(Object::Boolean(a != b)),
            Token::Lt => ResultObj::Borrow(Object::Boolean(a < b)),
            Token::Gt => ResultObj::Borrow(Object::Boolean(a > b)),
            Token::LtEq => ResultObj::Borrow(Object::Boolean(a <= b)),
            Token::GtEq => ResultObj::Borrow(Object::Boolean(a >= b)),
            _ => ResultObj::Borrow(Object::Null),
        }
    }

    fn eval_infix_string_operation(&self, a: &String, b: &String, op: Token) -> ResultObj {
        match op {
            Token::Plus => ResultObj::Borrow(Object::String(format!("{}{}", a, b))),
            Token::Eq => ResultObj::Borrow(Object::Boolean(a == b)),
            Token::NotEq => ResultObj::Borrow(Object::Boolean(a != b)),
            _ => ResultObj::Borrow(Object::Null),
        }
    }

    fn eval_infix_string_int_operation(&self, a: &str, b: Numeric, op: Token) -> ResultObj {
        if let Numeric::Int(int) = b {
            return match op {
                Token::Mul => ResultObj::Borrow(Object::String(a.repeat(int as usize))),
                _ => ResultObj::Borrow(Object::Null),
            };
        }
        ResultObj::Borrow(Object::Error(
            "No se puede hacer operaciones de indexacion con numeros flotantes".to_owned(),
        ))
    }

    fn eval_infix_list_operation(
        &self,
        a: &Vec<ResultObj>,
        b: &Vec<ResultObj>,
        op: Token,
    ) -> ResultObj {
        match op {
            Token::Plus => ResultObj::Ref(new_rc_object(Object::List(
                [a.as_slice(), b.as_slice()].concat(),
            ))),
            Token::Eq => ResultObj::Borrow(Object::Boolean(a == b)),
            Token::NotEq => ResultObj::Borrow(Object::Boolean(a != b)),
            Token::Lt => ResultObj::Borrow(Object::Boolean(a.len() < b.len())),
            Token::Gt => ResultObj::Borrow(Object::Boolean(a.len() > b.len())),
            Token::LtEq => ResultObj::Borrow(Object::Boolean(a.len() <= b.len())),
            Token::GtEq => ResultObj::Borrow(Object::Boolean(a.len() >= b.len())),
            _ => ResultObj::Borrow(Object::Null),
        }
    }

    fn eval_infix_list_int_operation(
        &self,
        a: &Vec<ResultObj>,
        b: Numeric,
        op: Token,
    ) -> ResultObj {
        if let Numeric::Int(int) = b {
            match op {
                Token::Mul => {
                    let mut objs = Vec::new();
                    objs.reserve(int as usize);
                    for _ in 0..int {
                        objs.extend(a.to_owned());
                    }
                    ResultObj::Ref(new_rc_object(Object::List(objs)))
                }
                _ => ResultObj::Borrow(Object::Null),
            };
        }
        ResultObj::Borrow(Object::Error(
            "No se puede hacer operaciones con numeros flotantes en listas".to_owned(),
        ))
    }

    fn eval_infix_null_operation(&self, operator: Token) -> ResultObj {
        match operator {
            Token::Eq => ResultObj::Borrow(Object::Boolean(true)),
            Token::NotEq => ResultObj::Borrow(Object::Boolean(false)),
            _ => ResultObj::Borrow(Object::Error("XD".to_owned())),
        }
    }

    fn eval_infix_null_object_operation(&self, operator: Token) -> ResultObj {
        match operator {
            Token::Eq => ResultObj::Borrow(Object::Boolean(false)),
            Token::NotEq => ResultObj::Borrow(Object::Boolean(true)),
            _ => ResultObj::Borrow(Object::Error("XD".to_owned())),
        }
    }

    fn eval_var(
        &mut self,
        name: String,
        value: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        if let Some(obj) = self.get_var_value(&name, env) {
            return obj;
        }

        self.insert_var(name, value, env, root_context)
    }

    fn set_var(
        &mut self,
        left: Expression,
        right: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        return match left {
            Expression::Identifier(ident) => {
                if !self.exist_var(&ident, env) {
                    return ResultObj::Borrow(Object::Error(format!(
                        "El no existe referencias hacia `{}`",
                        ident
                    )));
                }

                let obj = self.eval_expression(right, env, root_context);
                let mut env_ref = RefCell::borrow_mut(env);

                env_ref.update(ident, obj.clone());
                obj
            }
            Expression::Index { left, index } => {
                let right_obj = self.eval_expression(right, env, root_context);
                if self.is_error(&right_obj) {
                    return right_obj;
                }

                self.eval_index_expression(*left, *index, Some(right_obj), env, root_context)
            }
            _ => ResultObj::Borrow(Object::Error(format!(
                "No se puede realizar operaciones de asignacion a {}",
                left
            ))),
        };
    }

    fn get_var_value(&self, name: &String, env: &RcEnvironment) -> Option<ResultObj> {
        let env_ref = RefCell::borrow(env);
        env_ref.get(name).map(|_| -> ResultObj {
            ResultObj::Borrow(Object::Error(format!(
                "El identificador `{}` ya habia sido declarado",
                name
            )))
        })
    }

    fn exist_var(&self, name: &String, env: &RcEnvironment) -> bool {
        let env_ref = RefCell::borrow(env);
        env_ref.exist(name)
    }

    fn insert_var(
        &mut self,
        name: String,
        value: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        let obj = self.eval_expression(value, env, root_context);
        match obj {
            ResultObj::Borrow(Object::Error(msg)) => {
                return ResultObj::Borrow(Object::Error(msg)); // Cosas de borrow...
            }
            ResultObj::Borrow(Object::Void) => {
                return ResultObj::Borrow(Object::Error(
                    "No se puede asignar el tipo de dato vacio a una variable".to_owned(),
                ));
            }
            _ => {}
        }
        self.insert_obj(name, obj, env)
    }

    fn insert_obj(&mut self, name: String, obj: ResultObj, env: &RcEnvironment) -> ResultObj {
        let mut env_ref = RefCell::borrow_mut(env);
        env_ref.set(name, obj.clone());
        obj
    }

    fn eval_identifier(&mut self, ident: String, env: &RcEnvironment) -> ResultObj {
        match env.borrow().get(&ident) {
            Some(obj) => obj,
            None => {
                if let Some(func) = self.buildins_fn.get(&ident) {
                    return ResultObj::Borrow(Object::BuildinFn {
                        name: ident,
                        func: func.clone_box(),
                    });
                }
                ResultObj::Borrow(Object::Error(format!(
                    "El identicador `{}` no existe.",
                    ident
                )))
            }
        }
    }

    fn eval_call(
        &mut self,
        function: Expression,
        arguments: Vec<Expression>,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        let obj = self.eval_expression(function, env, root_context);
        match obj {
            ResultObj::Borrow(Object::FnExpr { params, body, env }) => {
                self.eval_fn_expr(arguments, params, body, &env, root_context)
            }
            ResultObj::Borrow(Object::Fn {
                params, body, env, ..
            }) => self.eval_fn_expr(arguments, params, body, &env, root_context),
            ResultObj::Borrow(Object::BuildinFn { func, .. }) => {
                func(self, arguments, env, root_context)
            }
            _ => ResultObj::Borrow(Object::Error("XD".to_owned())),
        }

        // match returned_obj {
        //     Object::Return(_) => returned_obj,
        //     Object::Error(_) => returned_obj,
        //     _ => Object::Void
        // }
    }

    fn eval_fn_expr(
        &mut self,
        arguments: FnParams,
        params: FnParams,
        body: BlockStatement,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        let scope_env = Rc::new(RefCell::new(Environment::new(Some(env.clone()))));
        if arguments.len() != params.len() {
            return ResultObj::Borrow(Object::Error(format!(
                "Se encontro {} argumentos, de los {}.",
                arguments.len(),
                params.len()
            )));
        }
        for (arg, param) in arguments.iter().zip(params) {
            if let Expression::Identifier(param_name) = param {
                self.insert_var(param_name, arg.to_owned(), &scope_env, root_context);
            }
        }
        self.eval_block_statement(body, &scope_env, root_context)
    }

    fn eval_list_literal(
        &mut self,
        elements: Vec<Expression>,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        let mut objs = Vec::new();
        for expr in elements {
            let obj = self.eval_expression(expr, env, root_context);
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
        root_context: &Context,
    ) -> ResultObj {
        let left_obj = self.eval_expression(left, env, root_context);
        match left_obj {
            ResultObj::Borrow(obj) => match obj {
                Object::Error(msg) => ResultObj::Borrow(Object::Error(msg)),
                _ => ResultObj::Borrow(Object::Error(
                    "Solo se puede usar el operador de indexar en listas".to_owned(),
                )),
            },
            ResultObj::Ref(obj) => match *obj.borrow_mut() {
                Object::List(ref mut objs) => {
                    if let Expression::NumericLiteral(Numeric::Int(index)) = index {
                        if let Some(new_value) = new_value {
                            if (index as usize) < objs.len() {
                                objs[index as usize] = new_value.clone();
                                return new_value;
                            }
                            return ResultObj::Borrow(Object::Null);
                        }
                        return match objs.get(index as usize) {
                            Some(obj) => obj.clone(),
                            None => ResultObj::Borrow(Object::Null),
                        };
                    }
                    ResultObj::Borrow(Object::Error(
                        "El operador de indexar solo opera con enteros".to_owned(),
                    ))
                }
                Object::Dictionary(ref pairs) => {
                    match pairs.get(&self.eval_expression(index.clone(), env, root_context)) {
                        Some(obj) => obj.clone(),
                        None => {
                            ResultObj::Borrow(Object::Error(format!("Llave invalida {}", index)))
                        }
                    }
                }
                _ => ResultObj::Borrow(Object::Error(
                    "Solo se puede usar el operador de indexar en listas".to_owned(),
                )),
            },
        }
    }

    fn eval_dictionary_expression(
        &mut self,
        expr_pairs: HashMap<Expression, Expression>,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> ResultObj {
        let mut pairs = HashMap::new();
        for (k, v) in expr_pairs {
            let obj_key = self.eval_expression(k, env, root_context);
            if self.is_error(&obj_key) {
                return obj_key;
            }
            let obj_value = self.eval_expression(v, env, root_context);
            if self.is_error(&obj_value) {
                return obj_value;
            }
            pairs.insert(obj_key, obj_value);
        }
        ResultObj::Ref(new_rc_object(Object::Dictionary(pairs)))
    }

    fn is_error(&mut self, obj: &ResultObj) -> bool {
        if let ResultObj::Borrow(Object::Error(_)) = obj {
            return true;
        }
        false
    }
}
