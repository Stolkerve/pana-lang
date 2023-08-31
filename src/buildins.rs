use crate::{
    ast::expressions::FnParams,
    environment::RcEnvironment,
    evaluator::{Context, Evaluator},
    objects::Object,
};

pub trait BuildinFnPointer:
    Fn(&mut Evaluator, FnParams, &RcEnvironment, &Context) -> Object
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + BuildinFnPointer>
    where
        Self: 'a;
}

impl<F> BuildinFnPointer for F
where
    F: Fn(&mut Evaluator, FnParams, &RcEnvironment, &Context) -> Object + Clone,
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
    root_context: &Context,
) -> Object {
    if args.len() != 1 {
        return Object::Error(format!("Se encontro {} argumentos de 1", args.len()));
    }
    let arg_obj = eval.eval_expression(args.get(0).unwrap().clone(), env, root_context);
    match arg_obj {
        Object::String(string) => Object::Int(string.len() as i64),
        Object::Array(objs) => Object::Int(objs.len() as i64),
        obj => Object::Error(format!(
            "Se espera un tipo de dato cadena, no {}",
            obj.get_type()
        )),
    }
}

// Funcion que imprime en una linea objetos en pantalla
pub fn buildin_imprimir_fn(
    eval: &mut Evaluator,
    args: FnParams,
    env: &RcEnvironment,
    root_context: &Context,
) -> Object {
    if !args.is_empty() {
        let objs = args.iter().map(|arg| eval.eval_expression(arg.clone(), env, root_context)).collect::<Vec<_>>();
        let string = objs.iter().map(|obj| obj.to_string()).collect::<Vec<_>>().join("");
        println!("{}", string);
        return Object::Void;
    }
    println!("");
    Object::Void
}