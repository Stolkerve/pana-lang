use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::objects::Object;

pub type StackObject = HashMap<String, Object>;
pub type RcEnvironment = Rc<RefCell<Environment>>;

pub struct Environment {
    stack: StackObject,
    parent: Option<RcEnvironment>,
}

impl Environment {
    pub fn new(parent: Option<RcEnvironment>) -> Self {
        Self {
            stack: StackObject::new(),
            parent,
        }
    }

    pub fn get(&self, name: &String) -> Option<Object> {
        match self.stack.get(name) {
            Some(obj) => Some(obj.clone()),
            None => match self.parent {
                Some(ref env) => {
                    let env = env.borrow();
                    env.get(name)
                }
                None => None,
            },
        }
    }

    // Esta funcion sirve para guardar una variable o fn en el
    // stack de environment del contexto
    pub fn set(&mut self, name: String, value: Object) -> Option<Object> {
        self.stack.insert(name.clone(), value.clone())
    }

    // Va a visitar todos los stacks hasta encontrar la variable o fn
    // y actulizarlo
    pub fn update(&mut self, name: String, value: Object) -> Option<Object> {
        match self.stack.insert(name.clone(), value.clone()) {
            Some(obj) => Some(obj.clone()),
            None => match self.parent {
                Some(ref env) => {
                    let mut env = env.borrow_mut();
                    env.update(name, value)
                }
                None => None,
            },
        }
    }
}
