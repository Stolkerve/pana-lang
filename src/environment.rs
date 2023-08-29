use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::objects::Object;

pub type StackObject = HashMap<String, Object>;
pub type RcEnvironment = Rc<RefCell<Environment>>;

#[derive(Debug)]
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

    pub fn set(&mut self, name: String, value: Object) -> Option<Object> {
        self.stack.insert(name, value)
    }
}
