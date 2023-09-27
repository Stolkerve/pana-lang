use std::io::Write;

use crate::eval::evaluator::Evaluator;
use crate::{
    types::Numeric, parser::expression::FnParams,
};
use super::{environment::RcEnvironment, objects::{ResultObj, Object}};


pub trait BuildinFnPointer:
    Fn(&mut Evaluator, FnParams, &RcEnvironment) -> ResultObj
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + BuildinFnPointer>
    where
        Self: 'a;
}

impl<F> BuildinFnPointer for F
where
    F: Fn(&mut Evaluator, FnParams, &RcEnvironment) -> ResultObj + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + BuildinFnPointer>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn 'a + BuildinFnPointer> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

// Funcion que retorna la longitud de un string o array
pub fn buildin_longitud_fn(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return ResultObj::Borrow(Object::Error(format!(
            "Se encontro {} argumentos de 1",
            args.len()
        )));
    }
    let arg_obj = eval.eval_expression(args.get(0).unwrap().clone(), env);
    match arg_obj {
        // obj => Object::Error(format!(
        //     "Se espera un tipo de dato cadena, no {}",
        //     obj.get_type()
        // )),
        ResultObj::Borrow(obj) => match obj {
            Object::String(string) => {
                ResultObj::Borrow(Object::Numeric(Numeric::Int(string.len() as i128)))
            }
            obj => ResultObj::Borrow(Object::Error(format!(
                "Se espera un tipo de dato cadena, no {}",
                obj.get_type()
            ))),
        },
        ResultObj::Ref(obj) => match &*obj.borrow() {
            Object::List(objs) => {
                ResultObj::Borrow(Object::Numeric(Numeric::Int(objs.len() as i128)))
            }
            Object::Dictionary(pairs) => {
                ResultObj::Borrow(Object::Numeric(Numeric::Int(pairs.len() as i128)))
            }
            obj => ResultObj::Borrow(Object::Error(format!(
                "Se espera un tipo de dato cadena, no {}",
                obj.get_type()
            ))),
        },
    }
}

// Funcion que imprime en una linea objetos en pantalla
pub fn buildin_imprimir_fn(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
) -> ResultObj {
    if !args.is_empty() {
        let objs = args
            .iter()
            .map(|arg| eval.eval_expression(arg.clone(), env))
            .collect::<Vec<_>>();
        let string = objs
            .iter()
            .map(|obj| obj.to_string())
            .collect::<Vec<_>>()
            .join("");
        println!("{}", string);
        return ResultObj::Borrow(Object::Void);
    }
    println!();
    ResultObj::Borrow(Object::Void)
}

// Funcion que retorna el tipo de dato del objeto
pub fn buildin_tipo_fn(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return ResultObj::Borrow(Object::Error(format!(
            "Se encontro {} argumentos de 1",
            args.len()
        )));
    }
    let arg_obj = eval.eval_expression(args.get(0).unwrap().clone(), env);
    match arg_obj {
        ResultObj::Borrow(obj) => ResultObj::Borrow(Object::String(obj.get_type().to_owned())),
        ResultObj::Ref(obj) => {
            ResultObj::Borrow(Object::String(obj.borrow().get_type().to_owned()))
        }
    }
}

// Funcion que permite un input desde el terminal
pub fn buildin_leer_fn(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
) -> ResultObj {
    match args.len() {
        0 => {
            let mut output = String::new();
            std::io::stdin().read_line(&mut output).unwrap();
            ResultObj::Borrow(Object::String(output))
        }
        1 => {
            let arg_obj = eval.eval_expression(args.get(0).unwrap().clone(), env);
            return match arg_obj {
                ResultObj::Borrow(obj) => match obj {
                    Object::String(promp) => {
                        let mut output = String::new();
                        print!("{}", promp);
                        std::io::stdout().flush().unwrap();
                        std::io::stdin().read_line(&mut output).unwrap();
                        return ResultObj::Borrow(Object::String(output.trim_end().to_owned()));
                    }
                    _ => ResultObj::Borrow(Object::Error(format!(
                        "Se espera un tipo de dato cadena, no {}",
                        obj.get_type()
                    ))),
                },
                ResultObj::Ref(obj) => ResultObj::Borrow(Object::Error(format!(
                    "Se espera un tipo de dato cadena, no {}",
                    obj.borrow().get_type()
                ))),
            };
        }
        _ => ResultObj::Borrow(Object::Error(format!(
            "Se encontro {} argumentos de 1",
            args.len()
        ))),
    }
}

pub fn buildin_cadena_fn(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return ResultObj::Borrow(Object::Error(format!(
            "Se encontro {} argumentos de 1",
            args.len()
        )));
    }
    let arg_obj = eval.eval_expression(args.get(0).unwrap().clone(), env);
    match arg_obj {
        ResultObj::Borrow(obj) => ResultObj::Borrow(Object::String(obj.to_string())),
        ResultObj::Ref(obj) => ResultObj::Borrow(Object::String(obj.borrow().to_string())),
    }
}
