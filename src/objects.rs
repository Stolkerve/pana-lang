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
