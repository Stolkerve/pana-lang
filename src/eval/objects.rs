use std::{cell::RefCell, collections::HashMap, fmt::Display, hash::Hash, rc::Rc};

use crate::{
    buildins::internal::InternalFnPointer,
    parser::{
        expression::{format_arguments, FnParams},
        statement::BlockStatement,
    },
    types::Numeric,
};

use super::environment::Environment;

pub type RcObject = Rc<RefCell<Object>>;
pub fn new_rc_object(obj: Object) -> RcObject {
    RcObject::new(RefCell::new(obj))
}

#[derive(Clone)]
pub struct FnExprObj {
    pub params: FnParams,
    pub body: BlockStatement,
    pub env: Rc<RefCell<Environment>>,
}

#[derive(Clone)]
pub struct FnObj {
    pub name: String,
    pub params: FnParams,
    pub body: BlockStatement,
    pub env: Rc<RefCell<Environment>>,
}

#[derive(Clone)]
pub struct BuildinFnObj {
    pub name: String,
    pub func: Box<dyn InternalFnPointer>,
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
    FnExpr(Box<FnExprObj>),
    Fn(Box<FnObj>),
    BuildinFn(Box<BuildinFnObj>),
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
            (Self::Fn(l_obj), Self::Fn(r_obj)) => l_obj.name == r_obj.name,
            (Self::BuildinFn(l_obj), Self::BuildinFn(r_obj)) => l_obj.name == r_obj.name,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::Numeric(n) => {
                format!("numerico {}", n.get_type())
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
            Object::Fn(obj) => {
                write!(
                    f,
                    "fn {}({}) {{...}}",
                    obj.name,
                    format_arguments(&obj.params)
                )
            }
            Object::FnExpr(obj) => write!(f, "fn({}) {{...}}", format_arguments(&obj.params)),
            Object::BuildinFn(obj) => write!(f, "fn {}(...) {{...}}", obj.name),
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

impl PartialOrd for ResultObj {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ResultObj::Ref(obj), ResultObj::Ref(obj2)) => {
                match (&*obj.borrow(), &*obj2.borrow()) {
                    (Object::String(str), Object::String(str2)) => str.partial_cmp(str2),
                    (_, _) => None,
                }
            }
            (ResultObj::Copy(obj), ResultObj::Copy(obj2)) => match (obj, obj2) {
                (Object::Numeric(num), Object::Numeric(num2)) => num.partial_cmp(num2),
                (_, _) => None,
            },
            (_, _) => None,
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
