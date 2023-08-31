use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    ast::{
        expressions::{format_arguments, FnParams},
        statements::BlockStatement,
    },
    environment::Environment, buildins::BuildinFnPointer,
};


#[derive(Clone)]
pub enum Object {
    Int(i64),
    Boolean(bool),
    Error(String),
    String(String),
    Return(Box<Object>),
    Array(Vec<Object>),
    FnExpr {
        params: FnParams,
        body: BlockStatement,
        env: Rc<RefCell<Environment>>,
    },
    Fn {
        name: String,
        params: FnParams,
        body: BlockStatement,
        env: Rc<RefCell<Environment>>,
    },
    BuildinFn {
        name: String,
        func: Box<dyn BuildinFnPointer>
    },
    Void,
    Null,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Error(l0), Self::Error(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Return(l0), Self::Return(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::FnExpr { params: l_params, body: l_body, env: l_env }, Self::FnExpr { params: r_params, body: r_body, env: r_env }) => false,
            (Self::Fn { name: l_name, params: l_params, body: l_body, env: l_env }, Self::Fn { name: r_name, params: r_params, body: r_body, env: r_env }) => l_name == r_name,
            (Self::BuildinFn { name: l_name, func: l_func }, Self::BuildinFn { name: r_name, func: r_func }) => l_name == r_name,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Object {
    pub fn get_type(&self) -> &str {
        match self {
            Object::Int(_) => "numerico",
            Object::Boolean(_) => "logico",
            Object::Error(_) => "error",
            Object::String(_) => "cadena",
            Object::Return(obj) => obj.get_type(),
            Object::FnExpr { .. } => "funcion",
            Object::Fn { .. } => "funcion",
            Object::BuildinFn { .. } => "funcion",
            Object::Null => "nulo",
            Object::Void => "vacio",
            Object::Array(_) => "lista",
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(int) => write!(f, "{}", int),
            Object::Boolean(b) => write!(f, "{}", bool_to_spanish(*b)),
            Object::Null => write!(f, "nulo"),
            Object::Error(msg) => write!(f, "{}", msg),
            Object::Return(obj) => write!(f, "{}", obj),
            Object::Fn { params, name, .. } => {
                write!(f, "fn {}({}) {{...}}", name, format_arguments(params))
            }
            Object::FnExpr { params, .. } => write!(f, "fn({}) {{...}}", format_arguments(params)),
            Object::BuildinFn { name, .. } => write!(f, "fn {}(...) {{...}}", name),
            Object::String(string) => write!(f, "{}", string),
            Object::Void => write!(f, ""),
            Object::Array(objs) => write!(f, "[{}]", objs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
        }
    }
}

fn bool_to_spanish(b: bool) -> String {
    if b {
        return "verdad".to_owned();
    }
    "falso".to_owned()
}
