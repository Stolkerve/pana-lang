use std::io::Write;

use crate::eval::{
    environment::RcEnvironment,
    evaluator::Evaluator,
    objects::{Object, ResultObj},
};
use crate::{parser::expression::FnParams, types::Numeric};

pub trait InternalFnPointer: Fn(&mut Evaluator, FnParams, &RcEnvironment) -> ResultObj {
    fn clone_box<'a>(&self) -> Box<dyn 'a + InternalFnPointer>
    where
        Self: 'a;
}

impl<F> InternalFnPointer for F
where
    F: Fn(&mut Evaluator, FnParams, &RcEnvironment) -> ResultObj + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + InternalFnPointer>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn 'a + InternalFnPointer> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

// Funcion que retorna la longitud de un string o array
pub fn longitud(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() != 1 {
        return ResultObj::Copy(Object::Error(format!(
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
        ResultObj::Copy(obj) => match obj {
            Object::String(string) => {
                ResultObj::Copy(Object::Numeric(Numeric::Int(string.len() as i64)))
            }
            obj => ResultObj::Copy(Object::Error(format!(
                "Se espera un tipo de dato cadena, no {}",
                obj.get_type()
            ))),
        },
        ResultObj::Ref(obj) => match &*obj.borrow() {
            Object::List(objs) => ResultObj::Copy(Object::Numeric(Numeric::Int(objs.len() as i64))),
            Object::Dictionary(pairs) => {
                ResultObj::Copy(Object::Numeric(Numeric::Int(pairs.len() as i64)))
            }
            obj => ResultObj::Copy(Object::Error(format!(
                "Se espera un tipo de dato cadena, no {}",
                obj.get_type()
            ))),
        },
    }
}

// Funcion que imprime en una linea objetos en pantalla
pub fn imprimir(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
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
        return ResultObj::Copy(Object::Void);
    }
    println!();
    ResultObj::Copy(Object::Void)
}

// Funcion que retorna el tipo de dato del objeto
pub fn tipo(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() != 1 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 1",
            args.len()
        )));
    }
    let arg_obj = eval.eval_expression(args.get(0).unwrap().clone(), env);
    match arg_obj {
        ResultObj::Copy(obj) => ResultObj::Copy(Object::String(obj.get_type().to_owned())),
        ResultObj::Ref(obj) => ResultObj::Copy(Object::String(obj.borrow().get_type().to_owned())),
    }
}

// Funcion que permite un input desde el terminal
pub fn leer(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    match args.len() {
        0 => {
            let mut output = String::new();
            std::io::stdin().read_line(&mut output).unwrap();
            ResultObj::Copy(Object::String(output))
        }
        1 => {
            let arg_obj = eval.eval_expression(args.get(0).unwrap().clone(), env);
            return match arg_obj {
                ResultObj::Copy(obj) => match obj {
                    Object::String(promp) => {
                        let mut output = String::new();
                        print!("{}", promp);
                        std::io::stdout().flush().unwrap();
                        std::io::stdin().read_line(&mut output).unwrap();
                        return ResultObj::Copy(Object::String(output.trim_end().to_owned()));
                    }
                    _ => ResultObj::Copy(Object::Error(format!(
                        "Se espera un tipo de dato cadena, no {}",
                        obj.get_type()
                    ))),
                },
                ResultObj::Ref(obj) => ResultObj::Copy(Object::Error(format!(
                    "Se espera un tipo de dato cadena, no {}",
                    obj.borrow().get_type()
                ))),
            };
        }
        _ => ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 1",
            args.len()
        ))),
    }
}

pub fn cadena(eval: &mut Evaluator, args: FnParams, env: &RcEnvironment) -> ResultObj {
    if args.len() != 1 {
        return ResultObj::Copy(Object::Error(format!(
            "Se encontro {} argumentos de 1",
            args.len()
        )));
    }
    let arg_obj = eval.eval_expression(args.get(0).unwrap().clone(), env);
    match arg_obj {
        ResultObj::Copy(obj) => ResultObj::Copy(Object::String(obj.to_string())),
        ResultObj::Ref(obj) => ResultObj::Copy(Object::String(obj.borrow().to_string())),
    }
}
