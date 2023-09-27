use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::objects::{ResultObj, RcObject};

pub type StackObject = HashMap<String, ResultObj>;
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

    pub fn get(&self, name: &String) -> Option<ResultObj> {
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

    pub fn get_ref(&self, name: &String) -> Option<RcObject> {
        match self.stack.get(name) {
            Some(obj) => Some(match obj {
                ResultObj::Borrow(_) => panic!("No se puede referenciar a un objeto copia"),
                ResultObj::Ref(xd) => xd.clone(),
            }),
            None => match self.parent {
                Some(ref env) => {
                    let env = env.borrow();
                    env.get_ref(name)
                }
                None => None,
            },
        }
    }

    pub fn exist(&self, name: &String) -> bool {
        match self.stack.get(name) {
            Some(_) => true,
            None => match self.parent {
                Some(ref env) => {
                    let env = env.borrow();
                    env.exist(name)
                }
                None => false,
            },
        }
    }

    // Esta funcion sirve para guardar una variable o fn en el
    // stack de environment del contexto
    pub fn set(&mut self, name: String, value: ResultObj) -> Option<ResultObj> {
        self.stack.insert(name.clone(), value)
    }

    // Va a visitar todos los stacks hasta encontrar la variable o fn
    // y actulizarlo
    pub fn update(&mut self, name: &String, value: ResultObj) -> Option<ResultObj> {
        match self.stack.get(name) {
            Some(_) => self.stack.insert(name.clone(), value),
            None => match self.parent {
                Some(ref env) => {
                    let mut env = env.borrow_mut();
                    env.update(name, value)
                }
                None => None,
            },
        }
        // match self.stack.insert(name.clone(), RcObject::new(RefCell::new(value))) {
        //     Some(obj) => Some(obj),
        //     None => match self.parent {
        //         Some(ref env) => {
        //         }
        //         None => None,
        //     },
        // }
    }
}
