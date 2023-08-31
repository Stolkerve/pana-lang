use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        expressions::{Expression, FnParams},
        statements::{BlockStatement, Statement},
        Program,
    },
    buildins::{buildin_imprimir_fn, buildin_longitud_fn, BuildinFnPointer},
    environment::{Environment, RcEnvironment},
    objects::Object,
    token::Token,
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
                    "imprimir".to_owned(),
                    Box::new(buildin_imprimir_fn) as Box<dyn BuildinFnPointer>,
                ),
            ]),
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Object {
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
    ) -> Object {
        // from https://github.com/Rydgel/monkey-rust/blob/master/lib/evaluator/mod.rs#L332
        // TODO: Arregla eso del context
        let context: Context;
        match program.len() {
            0 => Object::Void,
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
                    Object::Return(obj) => *obj,
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
    ) -> Object {
        match stmt {
            Statement::Var { name, value } => self.eval_var(name, value, env, root_context),
            Statement::Return(expr) => {
                Object::Return(Box::new(self.eval_expression(expr, env, root_context)))
            }
            Statement::Expression(expr) => self.eval_expression(expr, env, root_context),
            Statement::Fn { name, params, body } => {
                let obj = Object::Fn {
                    name: name.clone(),
                    params,
                    body,
                    env: env.clone(),
                };

                match self.check_ident(&name, env) {
                    Some(obj) => obj,
                    None => self.push_obj(name, obj, env),
                }
            }
        }
    }

    pub fn eval_expression(
        &mut self,
        expr: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> Object {
        match expr {
            Expression::IntLiteral(int) => Object::Int(int),
            Expression::BooleanLiteral(b) => Object::Boolean(b),
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
            Expression::FnLiteral { params, body } => Object::FnExpr {
                params,
                body,
                env: env.clone(),
            },
            Expression::Call {
                function,
                arguments,
            } => self.eval_call(*function, arguments, env, root_context),
            Expression::Assignment { name, value } => self.set_var(name, *value, env, root_context),
            Expression::StringLiteral(string) => Object::String(string),
            Expression::ArrayLiteral { elements } => self.eval_array_literal(elements, env, root_context),
            Expression::Index { left, index } => self.eval_index_expression(*left, *index, env, root_context),
        }
    }

    fn eval_if(
        &mut self,
        condition: Expression,
        consequence: BlockStatement,
        alternative: BlockStatement,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> Object {
        let condition = self.eval_expression(condition, env, root_context);
        let condition_res = {
            match condition {
                Object::Int(int) => int != 0,
                Object::Boolean(b) => b,
                Object::Null => false,
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
    ) -> Object {
        let right = self.eval_expression(right, env, root_context);
        match operator {
            Token::Plus => right,
            Token::Sub => match right {
                Object::Int(int) => Object::Int(-int),
                Object::Boolean(b) => Object::Int(-(b as i64)),
                _ => Object::Null,
            },
            Token::Not => match right {
                Object::Int(int) => Object::Boolean(int == 0),
                Object::Boolean(b) => Object::Boolean(!b),
                Object::Null => Object::Boolean(true),
                _ => Object::Null,
            },
            _ => Object::Null,
        }
    }

    fn eval_infix(
        &mut self,
        operator: Token,
        left: Expression,
        right: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> Object {
        let left = self.eval_expression(left, env, root_context);
        let right = self.eval_expression(right, env, root_context);

        match (left, right) {
            (Object::Int(a), Object::Int(b)) => self.eval_infix_operation(a, b, operator),
            (Object::Int(a), Object::Boolean(b)) => {
                self.eval_infix_operation(a, b as i64, operator)
            }
            (Object::Boolean(a), Object::Int(b)) => {
                self.eval_infix_operation(a as i64, b, operator)
            }
            (Object::Boolean(a), Object::Boolean(b)) => {
                self.eval_infix_operation(a as i64, b as i64, operator)
            }
            (Object::String(a), Object::String(b)) => {
                self.eval_infix_string_operation(&a, &b, operator)
            }
            ( Object::Error(msg), _) => Object::Error(msg),
            (_, Object::Error(msg)) => Object::Error(msg),
            _ => Object::Null,
        }
    }

    fn eval_infix_operation(&self, a: i64, b: i64, op: Token) -> Object {
        match op {
            Token::Plus => Object::Int(a + b),
            Token::Sub => Object::Int(a - b),
            Token::Div => Object::Int(a / b),
            Token::Mul => Object::Int(a * b),
            Token::Eq => Object::Boolean(a == b),
            Token::NotEq => Object::Boolean(a != b),
            Token::Lt => Object::Boolean(a < b),
            Token::Gt => Object::Boolean(a > b),
            Token::LtEq => Object::Boolean(a <= b),
            Token::GtEq => Object::Boolean(a >= b),
            _ => Object::Null,
        }
    }

    fn eval_infix_string_operation(&self, a: &String, b: &String, op: Token) -> Object {
        match op {
            Token::Plus => Object::String(format!("{}{}", a, b)),
            Token::Eq => Object::Boolean(a == b),
            Token::NotEq => Object::Boolean(a != b),
            _ => Object::Null,
        }
    }

    fn eval_var(
        &mut self,
        name: String,
        value: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> Object {
        if let Some(obj) = self.check_ident(&name, env) {
            return obj;
        }

        self.push_var(name, value, env, root_context)
    }

    fn set_var(
        &mut self,
        name: String,
        value: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> Object {
        if self.check_ident(&name, env).is_none() {
            return Object::Error(format!("El no existe referencias hacia `{}`", name));
        }

        let obj = self.eval_expression(value, env, root_context);
        let mut env_ref = RefCell::borrow_mut(env);

        env_ref.update(name, obj.clone());
        obj
    }

    fn check_ident(&self, name: &String, env: &RcEnvironment) -> Option<Object> {
        let env_ref = RefCell::borrow(env);

        env_ref.get(name).map(|_| {
            Object::Error(format!(
                "El identificador `{}` ya habia sido declarado",
                name
            ))
        })
    }

    fn push_var(
        &mut self,
        name: String,
        value: Expression,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> Object {
        let obj = self.eval_expression(value, env, root_context);
        match obj {
            Object::Error(msg) => {
                return Object::Error(msg);
            },
            Object::Void => {
                return Object::Error("No se puede asignar el tipo de dato vacio a una variable".to_owned());
            },
            _ => {}
        }
        self.push_obj(name, obj, env)
    }

    fn push_obj(&mut self, name: String, obj: Object, env: &RcEnvironment) -> Object {
        let mut env_ref = RefCell::borrow_mut(env);
        env_ref.set(name, obj.clone());
        obj
    }

    fn eval_identifier(&mut self, ident: String, env: &RcEnvironment) -> Object {
        match env.borrow().get(&ident) {
            Some(obj) => obj.clone(),
            None => {
                if let Some(func) = self.buildins_fn.get(&ident) {
                    return Object::BuildinFn {
                        name: ident.clone(),
                        func: Box::new(func.clone()),
                    };
                }
                Object::Error(format!("El identicador `{}` no existe.", ident))
            }
        }
    }

    fn eval_call(
        &mut self,
        function: Expression,
        arguments: Vec<Expression>,
        env: &RcEnvironment,
        root_context: &Context,
    ) -> Object {
        let obj = self.eval_expression(function, env, root_context);
        match obj {
            Object::FnExpr { params, body, env } => {
                self.eval_fn_expr(arguments, params, body, &env, root_context)
            }
            Object::Fn {
                params, body, env, ..
            } => self.eval_fn_expr(arguments, params, body, &env, root_context),
            Object::BuildinFn { func, .. } => func(self, arguments, env, root_context),
            _ => Object::Error("XD".to_owned()),
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
    ) -> Object {
        let scope_env = Rc::new(RefCell::new(Environment::new(Some(env.clone()))));
        if arguments.len() != params.len() {
            return Object::Error(format!(
                "Se encontro {} argumentos, de los {}.",
                arguments.len(),
                params.len()
            ));
        }
        for (arg, param) in arguments.iter().zip(params) {
            if let Expression::Identifier(param_name) = param {
                self.push_var(param_name, arg.to_owned(), &scope_env, root_context);
            }
        }
        self.eval_block_statement(body, &scope_env, root_context)
    }

    fn eval_array_literal(&mut self, elements: Vec<Expression>, env: &Rc<RefCell<Environment>>, root_context: &Context) -> Object {
        let mut objs = Vec::new();
        for expr in elements {
           let obj = self.eval_expression(expr, env, root_context);
           match obj {
                Object::Error(_) => return obj,
                _ => objs.push(obj)
            }
        }
        Object::Array(objs)
    }

    fn eval_index_expression(&mut self, left: Expression, index: Expression, env: &Rc<RefCell<Environment>>, root_context: &Context) -> Object {
        let left_obj = self.eval_expression(left, env, root_context);
        match left_obj {
            Object::Error(_) => left_obj,
            Object::Array(objs) => {
                if let Expression::IntLiteral(index) = index {
                    return match objs.get(index as usize) {
                        Some(obj) => obj.clone(),
                        None => Object::Null,
                    };
                }
                Object::Error("El operador de indexar solo opera con enteros".to_owned())
            },
            _ => {
                Object::Error("Solo se puede usar el operador de indexar en listas".to_owned())
            }
        }
    }
}
