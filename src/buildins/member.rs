use crate::{
    eval::{
        environment::RcEnvironment,
        evaluator::Evaluator,
        objects::{new_rc_object, Object, ResultObj},
    },
    lexer::Lexer,
    parser::expression::FnParams,
    token::TokenType,
    types::Numeric,
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
        "eliminar" => eliminar(eval, args, target, target_line, target_col, env),
        "limpiar" => limpiar(eval, args, target, target_line, target_col),
        "buscar" => buscar(eval, args, target, target_line, target_col, env),
        "insertar" => insertar(eval, args, target, target_line, target_col, env),
        "vacio" => vacio(eval, args, target, target_line, target_col),
        "invertir" => invertir(eval, args, target, target_line, target_col),

        // Funciones miembro de las listas
        "agregar" => agregar(eval, args, target, target_line, target_col, env),
        "indice" => indice(eval, args, target, target_line, target_col, env),
        // "ordenar" => ordenar(eval, args, target, target_line, target_col, env),
        "concatenar" => concatenar(eval, args, target, target_line, target_col, env),
        "eliminar_indice" => eliminar_indice(eval, args, target, target_line, target_col, env),
        "juntar" => juntar(eval, args, target, target_line, target_col, env),

        // Funciones miembro de los dicccionarios
        "llaves" => llaves(eval, args, target, target_line, target_col),
        "valores" => valores(eval, args, target, target_line, target_col),

        // Funciones miembro de las cadenas
        "separar" => separar(eval, args, target, target_line, target_col, env),
        "caracter" => caracter(eval, args, target, target_line, target_col, env),
        "caracteres" => caracteres(eval, args, target, target_line, target_col),
        "es_alfabetico" => es_alfabetico(eval, args, target, target_line, target_col),
        "es_numerico" => es_numerico(eval, args, target, target_line, target_col),
        "es_alfanumerico" => es_alfanumerico(eval, args, target, target_line, target_col),
        "inicia_con" => inicia_con(eval, args, target, target_line, target_col, env),
        "termina_con" => termina_con(eval, args, target, target_line, target_col, env),
        "a_mayusculas" => a_mayusculas(eval, args, target, target_line, target_col),
        "a_minusculas" => a_minusculas(eval, args, target, target_line, target_col),
        "reemplazar" => reemplazar(eval, args, target, target_line, target_col, env),
        "recortar" => recortar(eval, args, target, target_line, target_col),
        "subcadena" => subcadena(eval, args, target, target_line, target_col, env),
        "a_numerico" => a_numerico(eval, args, target, target_line, target_col),
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

fn missmatch_type(
    eval: &mut Evaluator,
    name: &str,
    obj_type: &str,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    ResultObj::Copy(Object::Error(eval.create_msg_err(
        format!(
            "El tipo de dato {} no posee el miembro `{}`.",
            obj_type, name
        ),
        target_line,
        target_col,
    )))
}

fn missmatch_type_arg(
    eval: &mut Evaluator,
    name: &str,
    obj_type: &str,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    ResultObj::Copy(Object::Error(eval.create_msg_err(
        format!("Se espera un tipo de dato {}, no {}.", name, obj_type),
        target_line,
        target_col,
    )))
}

fn missmatch_args(
    eval: &mut Evaluator,
    max: usize,
    len: usize,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    ResultObj::Copy(Object::Error(eval.create_msg_err(
        format!("Se encontro {} argumentos de {}.", len, max),
        target_line,
        target_col,
    )))
}

// TODO sumar el numero de caracteres a las columnas

pub fn eliminar(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    let obj_to_remove = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&obj_to_remove) {
        return obj_to_remove;
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "eliminar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => match list.iter().position(|obj| *obj == obj_to_remove) {
                Some(index) => list.remove(index),
                None => ResultObj::Copy(Object::Null),
            },
            Object::Dictionary(ref mut dict) => match dict.remove(&obj_to_remove) {
                Some(obj) => obj,
                None => ResultObj::Copy(Object::Null),
            },
            ref obj => missmatch_type(eval, "eliminar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn limpiar(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "limpiar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => {
                list.clear();
                ResultObj::Copy(Object::Void)
            }
            Object::Dictionary(ref mut dict) => {
                dict.clear();
                ResultObj::Copy(Object::Void)
            }
            Object::String(ref mut string) => {
                string.clear();
                ResultObj::Copy(Object::Void)
            }
            ref obj => missmatch_type(eval, "limpiar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn buscar(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    let find_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&find_obj) {
        return find_obj;
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "buscar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref list) => match list.iter().find(|obj| *obj == &find_obj) {
                Some(obj) => obj.clone(),
                None => ResultObj::Copy(Object::Null),
            },
            Object::String(ref string) => match find_obj {
                ResultObj::Copy(string2_obj) => match string2_obj {
                    Object::String(string2) => match string.find(&string2) {
                        Some(index) => ResultObj::Copy(Object::Numeric(Numeric::Int(index as i64))),
                        None => ResultObj::Copy(Object::Null),
                    },
                    obj => {
                        missmatch_type_arg(eval, "cadena", &obj.get_type(), target_line, target_col)
                    }
                },
                ResultObj::Ref(obj) => missmatch_type_arg(
                    eval,
                    "cadena",
                    &obj.borrow().get_type(),
                    target_line,
                    target_col,
                ),
            },
            ref obj => missmatch_type(eval, "buscar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn insertar(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 2 {
        return missmatch_args(eval, 2, args.len(), target_line, target_col);
    }
    let insert_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&insert_obj) {
        return insert_obj;
    }
    let index_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&index_obj) {
        return index_obj;
    }
    let index;
    if let ResultObj::Copy(Object::Numeric(Numeric::Int(int))) = index_obj {
        if int < 0 {
            return ResultObj::Copy(Object::Error(eval.create_msg_err(
                "El indice debe ser un numero positivo.".into(),
                target_line,
                target_col,
            )));
        }
        index = int;
    } else {
        return missmatch_type_arg(
            eval,
            "numerico entero",
            &index_obj.get_type(),
            target_line,
            target_col,
        );
    }

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "insertar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => {
                if (index as usize) < list.len() {
                    return ResultObj::Copy(Object::Error(eval.create_msg_err(
                        "El indice esta fuera del rango.".into(),
                        target_line,
                        target_col,
                    )));
                }
                list.insert(index as usize, insert_obj);
                ResultObj::Copy(Object::Void)
            }
            Object::String(ref mut string) => match insert_obj {
                ResultObj::Copy(insert_obj) => match insert_obj {
                    Object::String(string2) => {
                        if (index as usize) < string.len() {
                            return ResultObj::Copy(Object::Error(eval.create_msg_err(
                                "El indice esta fuera del rango.".into(),
                                target_line,
                                target_col,
                            )));
                        }
                        string.insert_str(index as usize, &string2);
                        ResultObj::Copy(Object::Void)
                    }
                    obj => {
                        missmatch_type_arg(eval, "cadena", &obj.get_type(), target_line, target_col)
                    }
                },
                ResultObj::Ref(obj) => missmatch_type_arg(
                    eval,
                    "cadena",
                    &obj.borrow().get_type(),
                    target_line,
                    target_col,
                ),
            },
            ref obj => missmatch_type(eval, "insertar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn vacio(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "vacio", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref list) => ResultObj::Copy(Object::Boolean(list.is_empty())),
            Object::Dictionary(ref dict) => ResultObj::Copy(Object::Boolean(dict.is_empty())),
            Object::String(ref string) => ResultObj::Copy(Object::Boolean(string.is_empty())),
            ref obj => missmatch_type(eval, "vavio", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn invertir(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "invertir", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => {
                list.reverse();
                ResultObj::Copy(Object::Void)
            }
            Object::String(ref mut string) => {
                *string = string.chars().rev().collect::<String>();
                ResultObj::Copy(Object::Void)
            }
            ref obj => missmatch_type(eval, "invertir", &obj.get_type(), target_line, target_col),
        },
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
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "agregar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => {
                let new_obj = eval.eval_expression(args.remove(0), env);
                if eval.is_error(&new_obj) {
                    return new_obj;
                }
                list.push(new_obj);
                ResultObj::Copy(Object::Void)
            }
            ref obj => missmatch_type(eval, "agregar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn indice(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    let find_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&find_obj) {
        return find_obj;
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "indice", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref list) => match list.iter().position(|obj| obj == &find_obj) {
                Some(index) => ResultObj::Copy(Object::Numeric(Numeric::Int(index as i64))),
                None => ResultObj::Copy(Object::Null),
            },
            ref obj => missmatch_type(eval, "indice", &obj.get_type(), target_line, target_col),
        },
    }
}

#[allow(dead_code, unused)]
pub fn ordenar(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    // Para un futuro donde se pase una funcion para comparar
    // if args.len() > 1 {
    //     return missmatch_args(eval, 1, args.len(), target_line, target_col);
    // }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "ordenar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(mut ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => {
                list.sort_by(|a, b| {
                    match (a, b) {
                        (ResultObj::Copy(n1), ResultObj::Copy(n2)) => {
                            todo!()
                        }
                        (ResultObj::Ref(ref_obj), ResultObj::Copy(n2)) => {
                            match *ref_obj.as_ref().borrow() {
                                Object::String(_) => {}
                                ref obj => todo!(),
                            }
                        }
                        (ResultObj::Copy(n1), ResultObj::Ref(ref_obj)) => {
                            match *ref_obj.as_ref().borrow() {
                                Object::String(_) => {}
                                ref obj => todo!(),
                            }
                        }
                        (obj, obj2) => {
                            todo!()
                        }
                    }
                    todo!()
                });
                ResultObj::Copy(Object::Null)
            }
            Object::String(ref mut string) => {
                let mut chars = string.chars().collect::<Vec<char>>();
                chars.sort();
                ResultObj::Ref(new_rc_object(Object::String(
                    chars.iter().collect::<String>(),
                )))
            }
            ref obj => missmatch_type(eval, "ordenar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn concatenar(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    let concat_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&concat_obj) {
        return concat_obj;
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "concatenar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => match concat_obj {
                ResultObj::Ref(list2_ref) => match *list2_ref.as_ref().borrow_mut() {
                    Object::List(ref mut list2) => {
                        list.append(list2);
                        ResultObj::Copy(Object::Void)
                    }
                    ref obj => {
                        missmatch_type_arg(eval, "lista", &obj.get_type(), target_line, target_col)
                    }
                },
                ResultObj::Copy(obj) => {
                    missmatch_type_arg(eval, "lista", &obj.get_type(), target_line, target_col)
                }
            },
            ref obj => missmatch_type(eval, "concatener", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn eliminar_indice(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    let index_to_remove = eval.eval_expression(args.remove(0), env);
    let index;
    if eval.is_error(&index_to_remove) {
        return index_to_remove;
    }
    match index_to_remove {
        ResultObj::Copy(Object::Numeric(Numeric::Int(i))) => {
            index = i;
        }
        obj => {
            return missmatch_type_arg(
                eval,
                "numerico entero",
                &obj.get_type(),
                target_line,
                target_col,
            )
        }
    }

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "eliminar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => {
                if (index as usize) < list.len() {
                    return list.remove(index as usize);
                }
                ResultObj::Copy(Object::Null)
            }
            ref obj => missmatch_type(eval, "eliminar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn juntar(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    let join;
    let join_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&join_obj) {
        return join_obj;
    }
    match join_obj {
        ResultObj::Ref(join_ref) => match *join_ref.borrow() {
            Object::String(ref string) => {
                join = string.clone();
            }
            ref obj => {
                return missmatch_type_arg(
                    eval,
                    "numerico entero",
                    &obj.get_type(),
                    target_line,
                    target_col,
                )
            }
        },
        obj => {
            return missmatch_type_arg(
                eval,
                "numerico entero",
                &obj.get_type(),
                target_line,
                target_col,
            )
        }
    };

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "separar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref list) => {
                let join = list
                    .iter()
                    .map(|obj| obj.to_string())
                    .collect::<Vec<_>>()
                    .join(&join);
                ResultObj::Ref(new_rc_object(Object::String(join)))
            }
            ref obj => missmatch_type(eval, "separar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn llaves(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "llaves", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::Dictionary(ref dict) => ResultObj::Ref(new_rc_object(Object::List(
                dict.keys().clone().map(|obj| obj.clone()).collect(),
            ))),
            ref obj => missmatch_type(eval, "llaves", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn valores(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "valores", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::Dictionary(ref dict) => ResultObj::Ref(new_rc_object(Object::List(
                dict.values().map(|obj| obj.clone()).collect(),
            ))),
            ref obj => missmatch_type(eval, "valores", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn separar(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    let split;
    let split_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&split_obj) {
        return split_obj;
    }
    match split_obj {
        ResultObj::Ref(split_ref) => match *split_ref.borrow() {
            Object::String(ref string) => {
                split = string.clone();
            }
            ref obj => {
                return missmatch_type_arg(
                    eval,
                    "numerico entero",
                    &obj.get_type(),
                    target_line,
                    target_col,
                )
            }
        },
        obj => {
            return missmatch_type_arg(
                eval,
                "numerico entero",
                &obj.get_type(),
                target_line,
                target_col,
            )
        }
    };

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "separar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => {
                let split = string
                    .split(&split)
                    .map(|w| ResultObj::Ref(new_rc_object(Object::String(w.into()))))
                    .collect::<Vec<_>>();
                ResultObj::Ref(new_rc_object(Object::List(split)))
            }
            ref obj => missmatch_type(eval, "separar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn caracter(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    let index_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&index_obj) {
        return index_obj;
    }
    let index;
    if let ResultObj::Copy(Object::Numeric(Numeric::Int(int))) = index_obj {
        if int < 0 {
            return ResultObj::Copy(Object::Error(eval.create_msg_err(
                "El indice debe ser un numero positivo.".into(),
                target_line,
                target_col,
            )));
        }
        index = int;
    } else {
        return missmatch_type_arg(
            eval,
            "numerico entero",
            &index_obj.get_type(),
            target_line,
            target_col,
        );
    }

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "caracter", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => match string.chars().nth(index as usize) {
                Some(c) => ResultObj::Ref(new_rc_object(Object::String(c.to_string()))),
                None => ResultObj::Copy(Object::Null),
            },
            ref obj => missmatch_type(eval, "caracter", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn caracteres(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "caracteres", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => ResultObj::Ref(new_rc_object(Object::List(
                string
                    .chars()
                    .map(|c| ResultObj::Copy(Object::Numeric(Numeric::Int(c as i64))))
                    .collect::<Vec<ResultObj>>(),
            ))),
            ref obj => missmatch_type(eval, "caracteres", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn es_alfabetico(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type(
            eval,
            "es_alfabetico",
            &obj.get_type(),
            target_line,
            target_col,
        ),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => ResultObj::Copy(Object::Boolean(
                string.chars().fold(true, |acc, c| c.is_alphabetic() && acc),
            )),
            ref obj => missmatch_type(
                eval,
                "es_alfabetico",
                &obj.get_type(),
                target_line,
                target_col,
            ),
        },
    }
}

pub fn es_numerico(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type(
            eval,
            "es_numerico",
            &obj.get_type(),
            target_line,
            target_col,
        ),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => ResultObj::Copy(Object::Boolean(
                string.chars().fold(true, |acc, c| c.is_numeric() && acc),
            )),
            ref obj => missmatch_type(
                eval,
                "es_numerico",
                &obj.get_type(),
                target_line,
                target_col,
            ),
        },
    }
}

pub fn es_alfanumerico(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type(
            eval,
            "es_alfanumerico",
            &obj.get_type(),
            target_line,
            target_col,
        ),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => ResultObj::Copy(Object::Boolean(
                string
                    .chars()
                    .fold(true, |acc, c| c.is_alphanumeric() && acc),
            )),
            ref obj => missmatch_type(
                eval,
                "es_alfanumerico",
                &obj.get_type(),
                target_line,
                target_col,
            ),
        },
    }
}

pub fn inicia_con(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    let pattern_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&pattern_obj) {
        return pattern_obj;
    }
    let pattern: String;
    if let ResultObj::Ref(ref_pattern_obj) = pattern_obj {
        if let Object::String(ref string) = *ref_pattern_obj.borrow() {
            pattern = string.clone();
        } else {
            return missmatch_type_arg(
                eval,
                "cadena",
                &ref_pattern_obj.borrow().get_type(),
                target_line,
                target_col,
            );
        }
    } else {
        return missmatch_type_arg(
            eval,
            "cadena",
            &pattern_obj.get_type(),
            target_line,
            target_col,
        );
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "inicia_con", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => {
                ResultObj::Copy(Object::Boolean(string.starts_with(&pattern)))
            }
            ref obj => missmatch_type(eval, "inicia_con", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn termina_con(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 1 {
        return missmatch_args(eval, 1, args.len(), target_line, target_col);
    }
    let pattern_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&pattern_obj) {
        return pattern_obj;
    }
    let pattern: String;
    if let ResultObj::Ref(ref_pattern_obj) = pattern_obj {
        if let Object::String(ref string) = *ref_pattern_obj.borrow() {
            pattern = string.clone();
        } else {
            return missmatch_type_arg(
                eval,
                "cadena",
                &ref_pattern_obj.borrow().get_type(),
                target_line,
                target_col,
            );
        }
    } else {
        return missmatch_type_arg(
            eval,
            "cadena",
            &pattern_obj.get_type(),
            target_line,
            target_col,
        );
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type(
            eval,
            "termina_con",
            &obj.get_type(),
            target_line,
            target_col,
        ),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => {
                ResultObj::Copy(Object::Boolean(string.starts_with(&pattern)))
            }
            ref obj => missmatch_type(
                eval,
                "termina_con",
                &obj.get_type(),
                target_line,
                target_col,
            ),
        },
    }
}

pub fn a_mayusculas(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type(
            eval,
            "a_mayusculas",
            &obj.get_type(),
            target_line,
            target_col,
        ),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref mut string) => {
                ResultObj::Ref(new_rc_object(Object::String(string.to_uppercase())))
            }
            ref obj => missmatch_type(
                eval,
                "a_mayusculas",
                &obj.get_type(),
                target_line,
                target_col,
            ),
        },
    }
}

pub fn a_minusculas(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type(
            eval,
            "a_minusculas",
            &obj.get_type(),
            target_line,
            target_col,
        ),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref mut string) => {
                ResultObj::Ref(new_rc_object(Object::String(string.to_lowercase())))
            }
            ref obj => missmatch_type(
                eval,
                "a_minusculas",
                &obj.get_type(),
                target_line,
                target_col,
            ),
        },
    }
}

pub fn reemplazar(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 2 {
        return missmatch_args(eval, 2, args.len(), target_line, target_col);
    }
    let pattern_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&pattern_obj) {
        return pattern_obj;
    }
    let new_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&pattern_obj) {
        return pattern_obj;
    }

    let pattern: String;
    let new: String;
    match (pattern_obj, new_obj) {
        (ResultObj::Ref(ref_pattern_obj), ResultObj::Ref(ref_new_obj)) => {
            match (&*ref_pattern_obj.borrow(), &*ref_new_obj.borrow()) {
                (Object::String(string_pat), Object::String(string_new)) => {
                    pattern = string_pat.clone();
                    new = string_new.clone();
                }
                (Object::String(_), obj) => {
                    return missmatch_type_arg(
                        eval,
                        "cadena",
                        &obj.get_type(),
                        target_line,
                        target_col,
                    );
                }
                (obj, Object::String(_)) => {
                    return missmatch_type_arg(
                        eval,
                        "cadena",
                        &obj.get_type(),
                        target_line,
                        target_col,
                    );
                }
                (obj, _) => {
                    return missmatch_type_arg(
                        eval,
                        "cadena",
                        &obj.get_type(),
                        target_line,
                        target_col,
                    );
                }
            }
        }
        (ResultObj::Ref(_), ResultObj::Copy(err_obj)) => {
            return missmatch_type_arg(
                eval,
                "cadena",
                &err_obj.get_type(),
                target_line,
                target_col,
            );
        }
        (ResultObj::Copy(err_obj), ResultObj::Ref(_)) => {
            return missmatch_type_arg(
                eval,
                "cadena",
                &err_obj.get_type(),
                target_line,
                target_col,
            );
        }
        (obj, _) => {
            return missmatch_type_arg(eval, "cadena", &obj.get_type(), target_line, target_col);
        }
    }

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "reemplazar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref mut string) => {
                *string = string.replace(&pattern, &new);
                ResultObj::Copy(Object::Void)
            }
            ref obj => missmatch_type(eval, "reemplazar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn recortar(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "recortar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref mut string) => {
                *string = string.trim().into();
                ResultObj::Copy(Object::Void)
            }
            ref obj => missmatch_type(eval, "recortar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn subcadena(
    eval: &mut Evaluator,
    mut args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
    env: &RcEnvironment,
) -> ResultObj {
    if args.len() != 2 {
        return missmatch_args(eval, 2, args.len(), target_line, target_col);
    }
    let pos_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&pos_obj) {
        return pos_obj;
    }
    let len_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&len_obj) {
        return len_obj;
    }

    let pos;
    let len;

    match (pos_obj, len_obj) {
        (
            ResultObj::Copy(Object::Numeric(Numeric::Int(p))),
            ResultObj::Copy(Object::Numeric(Numeric::Int(l))),
        ) => {
            if p < 0 || l < 0 {
                return ResultObj::Copy(Object::Error(eval.create_msg_err(
                    "El indice debe ser un numero positivo.".into(),
                    target_line,
                    target_col,
                )));
            }
            pos = p;
            len = l;
        }
        (ResultObj::Copy(Object::Numeric(Numeric::Int(_))), err_obj) => {
            return missmatch_type_arg(
                eval,
                "cadena",
                &err_obj.get_type(),
                target_line,
                target_col,
            );
        }
        (err_obj, _) => {
            return missmatch_type_arg(
                eval,
                "cadena",
                &err_obj.get_type(),
                target_line,
                target_col,
            );
        }
    }

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "subcadena", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => {
                if (pos as usize) > string.len() {
                    return ResultObj::Copy(Object::Error(eval.create_msg_err(
                        "El indice esta fuera del rango.".into(),
                        target_line,
                        target_col,
                    )));
                }
                if len > (string.len() as i64) - pos {
                    return ResultObj::Copy(Object::Error(eval.create_msg_err(
                        "El indice esta fuera del rango.".into(),
                        target_line,
                        target_col,
                    )));
                }

                let sub_str: String = string
                    .chars()
                    .skip(len as usize)
                    .take(pos as usize)
                    .collect();
                ResultObj::Ref(new_rc_object(Object::String(sub_str)))
            }
            ref obj => missmatch_type(eval, "subcadena", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn a_numerico(
    eval: &mut Evaluator,
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(eval, 0, args.len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type(eval, "a_numerico", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => {
                let mut lexer = Lexer::new(string.chars().collect());
                let token = lexer.next_token();
                if let TokenType::Numeric(num) = token.r#type {
                    return ResultObj::Copy(Object::Numeric(num));
                } else if let TokenType::Illegal(c) = token.r#type {
                    return ResultObj::Copy(Object::Error(eval.create_msg_err(
                        format!(
                            "Se encontro un simbolo ilegal `{}` durante la conversion",
                            c
                        ),
                        target_line,
                        target_col,
                    )));
                }
                missmatch_type(
                    eval,
                    "a_numerico",
                    &ref_obj.borrow().get_type(),
                    target_line,
                    target_col,
                )
            }
            ref obj => missmatch_type(eval, "a_numerico", &obj.get_type(), target_line, target_col),
        },
    }
}
