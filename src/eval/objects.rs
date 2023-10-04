use std::{cell::RefCell, collections::HashMap, fmt::Display, hash::Hash, rc::Rc};

use crate::{
    parser::{
        expression::{format_arguments, FnParams},
        statement::BlockStatement,
    },
    types::Numeric,
};

use super::{builtins::BuildinFnPointer, environment::Environment};

pub type RcObject = Rc<RefCell<Object>>;
pub fn new_rc_object(obj: Object) -> RcObject {
    RcObject::new(RefCell::new(obj))
}

#[derive(Clone)]
pub enum Object {
    Numeric(Numeric),
    Boolean(bool),
    Error(String),
    String(String),
    Return(Box<ResultObj>),
    List(Vec<ResultObj>),
    Dictionary(HashMap<ResultObj, ResultObj>),
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
        func: Box<dyn BuildinFnPointer>,
    },
    Void,
    Break,
    Continue,
    Null,
}

impl Eq for Object {}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Numeric(l0), Self::Numeric(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Error(l0), Self::Error(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Return(_), Self::Return(_)) => panic!("No se peude comparar un return"),
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::FnExpr { .. }, Self::FnExpr { .. }) => panic!("No se puede comparar funciones"),
            (Self::Fn { name: l_name, .. }, Self::Fn { name: r_name, .. }) => l_name == r_name,
            (Self::BuildinFn { name: l_name, .. }, Self::BuildinFn { name: r_name, .. }) => {
                l_name == r_name
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::Numeric(n) => {
                format!(
                    "numerico {}",
                    match n {
                        Numeric::Int(_) => String::from("entero"),
                        Numeric::Float(_) => String::from("flotante"),
                    }
                )
            }
            Object::Boolean(_) => "logico".to_owned(),
            Object::Error(_) => "error".to_owned(),
            Object::String(_) => "cadena".to_owned(),
            Object::Return(obj) => {
                match obj.as_ref() {
                    ResultObj::Copy(obj) => obj.get_type(),
                    ResultObj::Ref(_) => todo!(),
                    // let a = obj.borrow();
                    // a.get_type()
                }
            }
            Object::FnExpr { .. } => "funcion".to_owned(),
            Object::Fn { .. } => "funcion".to_owned(),
            Object::BuildinFn { .. } => "funcion".to_owned(),
            Object::Null => "nulo".to_owned(),
            Object::Void => "vacio".to_owned(),
            Object::List(_) => "lista".to_owned(),
            Object::Dictionary { .. } => "diccionario".to_owned(),
            Object::Break => unreachable!(),
            Object::Continue => unreachable!(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Numeric(int) => write!(f, "{}", int),
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
            Object::List(objs) => write!(
                f,
                "[{}]",
                objs.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Object::Dictionary(pairs) => write!(
                f,
                "{{{}}}",
                pairs
                    .iter()
                    .map(|(x, y)| format!("{}: {}", x, y))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Object::Break => unreachable!(),
            Object::Continue => unreachable!(),
        }
    }
}

fn bool_to_spanish(b: bool) -> String {
    if b {
        return "verdad".to_owned();
    }
    "falso".to_owned()
}

/*
ResultObj, como su nombre dice es el resultado de del Evaluator,
este puede retornar una copia de un objeto como son los:
Int, Bool, Null, String, Error, Return y Void. O retornar una referencia
a un objeto como: List, Dictionary.
*/
#[derive(Clone)]
pub enum ResultObj {
    Copy(Object),
    Ref(RcObject),
}

impl ResultObj {
    pub fn get_type(&self) -> String {
        match self {
            ResultObj::Copy(obj) => obj.get_type(),
            ResultObj::Ref(obj) => obj.borrow().get_type(),
        }
    }
}

impl Hash for ResultObj {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Eq for ResultObj {}

impl PartialEq for ResultObj {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Copy(l0), Self::Copy(r0)) => l0 == r0,
            (Self::Ref(l0), Self::Ref(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Display for ResultObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultObj::Copy(obj) => write!(f, "{}", obj),
            ResultObj::Ref(obj) => write!(f, "{}", obj.borrow()),
        }
    }
}
