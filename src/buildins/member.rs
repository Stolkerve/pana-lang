use crate::{
    eval::{
        environment::RcEnvironment,
        evaluator::Evaluator,
        objects::{Object, ResultObj},
    },
    parser::expression::FnParams,
};

pub fn match_member_fn(
    eval: &mut Evaluator,
    identifier: String,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    match identifier.as_ref() {
        // Mixto
        "eliminar" => todo!(),
        "limpiar" => todo!(),
        "concatenar" => todo!(),
        "buscar" => todo!(),
        "insertar" => todo!(),
        "separar" => todo!(),
        "vacio" => todo!(),

        // Funciones miembro de las listas
        "agregar" => agregar(eval, args, target, target_line, target_col, env),
        "indice" => todo!(),
        "ordenar" => todo!(),

        // Funciones miembro de los dicccionarios
        "llaves" => todo!(),
        "valores" => todo!(),

        // Funciones miembro de las cadenas
        "caracteres" => todo!(),
        "es_alfa" => todo!(),
        "es_numerico" => todo!(),
        "inicia_con" => todo!(),
        "invertir" => todo!(),
        "mayusculas" => todo!(),
        "minusculas" => todo!(),
        "reemplazar" => todo!(),
        "recortar" => todo!(),
        "subcadena" => todo!(),
        "a_numerico" => todo!(),
        _ => ResultObj::Copy(Object::Error(eval.create_msg_err(
            format!(
                "El tipo de dato {} no posee el miembro {}",
                target.get_type(),
                identifier
            ),
            target_line,
            target_col,
        ))),
    }
}

pub fn agregar(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    match target {
        ResultObj::Copy(obj) => ResultObj::Copy(Object::Error(eval.create_msg_err(
            format!(
                "El tipo de dato {} no posee el miembro `agregar`",
                obj.get_type()
            ),
            target_line,
            target_col,
        ))),
        ResultObj::Ref(ref_obj) => match *ref_obj.borrow_mut() {
            Object::List(ref mut list) => {
                if args.len() != 1 {
                    return ResultObj::Copy(Object::Error(eval.create_msg_err(
                        format!("Se encontro {} argumentos de 1", args.len()),
                        target_line,
                        target_col,
                    )));
                }
                let new_obj = eval.eval_expression(args.remove(0), env);
                if eval.is_error(&new_obj) {
                    return new_obj;
                }
                list.push(new_obj);
                ResultObj::Copy(Object::Void)
            }
            ref obj => ResultObj::Copy(Object::Error(eval.create_msg_err(
                format!(
                    "El tipo de dato {} no posee el miembro `agregar`",
                    obj.get_type()
                ),
                target_line,
                target_col,
            ))),
        },
    }
}
