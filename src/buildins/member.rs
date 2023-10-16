use crate::{
    eval::{
        environment::RcEnvironment,
        evaluator::{create_msg_err, Evaluator},
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
        "limpiar" => limpiar(args, target, target_line, target_col),
        "buscar" => buscar(eval, args, target, target_line, target_col, env),
        "insertar" => insertar(eval, args, target, target_line, target_col, env),
        "vacio" => vacio(args, target, target_line, target_col),
        "invertir" => invertir(args, target, target_line, target_col),

        // Funciones miembro de las listas
        "agregar" => agregar(eval, args, target, target_line, target_col, env),
        "indice" => indice(eval, args, target, target_line, target_col, env),
        "ordenar" => ordenar(eval, args, target, target_line, target_col, env),
        "concatenar" => concatenar(eval, args, target, target_line, target_col, env),
        "eliminar_indice" => eliminar_indice(eval, args, target, target_line, target_col, env),
        "juntar" => juntar(eval, args, target, target_line, target_col, env),

        // Funciones miembro de los dicccionarios
        "llaves" => llaves(args, target, target_line, target_col),
        "valores" => valores(args, target, target_line, target_col),

        // Funciones miembro de las cadenas
        "separar" => separar(eval, args, target, target_line, target_col, env),
        "caracter" => caracter(eval, args, target, target_line, target_col, env),
        "caracteres" => caracteres(args, target, target_line, target_col),
        "es_alfabetico" => es_alfabetico(args, target, target_line, target_col),
        "es_numerico" => es_numerico(args, target, target_line, target_col),
        "es_alfanumerico" => es_alfanumerico(args, target, target_line, target_col),
        "inicia_con" => inicia_con(eval, args, target, target_line, target_col, env),
        "termina_con" => termina_con(eval, args, target, target_line, target_col, env),
        "a_mayusculas" => a_mayusculas(args, target, target_line, target_col),
        "a_minusculas" => a_minusculas(args, target, target_line, target_col),
        "reemplazar" => reemplazar(eval, args, target, target_line, target_col, env),
        "recortar" => recortar(args, target, target_line, target_col),
        "subcadena" => subcadena(eval, args, target, target_line, target_col, env),
        "a_numerico" => a_numerico(args, target, target_line, target_col),
        _ => ResultObj::Copy(Object::Error(create_msg_err(
            format!(
                "El tipo de dato {} no posee el miembro `{}`",
                target.get_type(),
                identifier
            ),
            target_line,
            target_col + 2,
        ))),
    }
}

fn missmatch_type(name: &str, obj_type: &str, target_line: usize, target_col: usize) -> ResultObj {
    ResultObj::Copy(Object::Error(create_msg_err(
        format!(
            "El tipo de dato {} no posee el miembro `{}`",
            obj_type, name
        ),
        target_line,
        target_col + 2,
    )))
}

fn missmatch_type_arg(
    name: &str,
    obj_type: &str,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    ResultObj::Copy(Object::Error(create_msg_err(
        format!("Se espera un tipo de dato {}, no {}.", name, obj_type),
        target_line,
        target_col + name.len(),
    )))
}

fn missmatch_args(
    max: usize,
    len: usize,
    name_len: usize,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    ResultObj::Copy(Object::Error(create_msg_err(
        format!("Se encontro {} argumentos de {}", len, max),
        target_line,
        target_col + name_len + 3 + len,
    )))
}

fn quick_sort(slice: &mut [ResultObj]) -> Option<ResultObj> {
    let len = slice.len();
    _quick_sort(slice, 0, (len - 1) as isize)
}

fn _quick_sort(slice: &mut [ResultObj], low: isize, high: isize) -> Option<ResultObj> {
    if low < high {
        match partition(slice, low, high) {
            Ok(p) => {
                let mut res = _quick_sort(slice, low, p - 1);
                if res.is_some() {
                    return res;
                }
                res = _quick_sort(slice, p + 1, high);
                if res.is_some() {
                    return res;
                }
            }
            Err(res) => return Some(res),
        }
    }
    None
}

fn partition(slice: &mut [ResultObj], low: isize, high: isize) -> Result<isize, ResultObj> {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;
    loop {
        store_index += 1;
        loop {
            match slice[store_index as usize].partial_cmp(&slice[pivot]) {
                Some(ord) => match ord {
                    std::cmp::Ordering::Less => store_index += 1,
                    _ => break,
                },
                None => todo!("Error no se puede comparar"),
            }
        }

        last_index -= 1;

        while last_index >= 0 {
            match slice[last_index as usize].partial_cmp(&slice[pivot]) {
                Some(ord) => match ord {
                    std::cmp::Ordering::Greater => last_index -= 1,
                    _ => break,
                },
                None => todo!("Error no se puede comparar"),
            }
        }

        if store_index >= last_index {
            break;
        } else {
            slice.swap(store_index as usize, last_index as usize);
        }
    }
    slice.swap(store_index as usize, pivot as usize);
    Ok(store_index)
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
        return missmatch_args(1, args.len(), "eliminar".len(), target_line, target_col);
    }
    let obj_to_remove = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&obj_to_remove) {
        return obj_to_remove;
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("eliminar", &obj.get_type(), target_line, target_col)
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
            ref obj => missmatch_type("eliminar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn limpiar(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "limpiar".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type("limpiar", &obj.get_type(), target_line, target_col),
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
            ref obj => missmatch_type("limpiar", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(1, args.len(), "buscar".len(), target_line, target_col);
    }
    let find_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&find_obj) {
        return find_obj;
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type("buscar", &obj.get_type(), target_line, target_col),
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
                    obj => missmatch_type_arg("cadena", &obj.get_type(), target_line, target_col),
                },
                ResultObj::Ref(obj) => {
                    missmatch_type_arg("cadena", &obj.borrow().get_type(), target_line, target_col)
                }
            },
            ref obj => missmatch_type("buscar", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(2, args.len(), "insertar".len(), target_line, target_col);
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
            return ResultObj::Copy(Object::Error(create_msg_err(
                "El indice debe ser un numero positivo.".into(),
                target_line,
                target_col,
            )));
        }
        index = int;
    } else {
        return missmatch_type_arg(
            "numerico entero",
            &index_obj.get_type(),
            target_line,
            target_col,
        );
    }

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("insertar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => {
                if (index as usize) < list.len() {
                    return ResultObj::Copy(Object::Error(create_msg_err(
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
                            return ResultObj::Copy(Object::Error(create_msg_err(
                                "El indice esta fuera del rango.".into(),
                                target_line,
                                target_col,
                            )));
                        }
                        string.insert_str(index as usize, &string2);
                        ResultObj::Copy(Object::Void)
                    }
                    obj => missmatch_type_arg("cadena", &obj.get_type(), target_line, target_col),
                },
                ResultObj::Ref(obj) => {
                    missmatch_type_arg("cadena", &obj.borrow().get_type(), target_line, target_col)
                }
            },
            ref obj => missmatch_type("insertar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn vacio(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "vacio".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type("vacio", &obj.get_type(), target_line, target_col),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref list) => ResultObj::Copy(Object::Boolean(list.is_empty())),
            Object::Dictionary(ref dict) => ResultObj::Copy(Object::Boolean(dict.is_empty())),
            Object::String(ref string) => ResultObj::Copy(Object::Boolean(string.is_empty())),
            ref obj => missmatch_type("vacio", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn invertir(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "invertir".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("invertir", &obj.get_type(), target_line, target_col)
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
            ref obj => missmatch_type("invertir", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(1, args.len(), "agregar".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type("agregar", &obj.get_type(), target_line, target_col),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => {
                let new_obj = eval.eval_expression(args.remove(0), env);
                if eval.is_error(&new_obj) {
                    return new_obj;
                }
                list.push(new_obj);
                ResultObj::Copy(Object::Void)
            }
            ref obj => missmatch_type("agregar", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(1, args.len(), "indice".len(), target_line, target_col);
    }
    let find_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&find_obj) {
        return find_obj;
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type("indice", &obj.get_type(), target_line, target_col),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref list) => match list.iter().position(|obj| obj == &find_obj) {
                Some(index) => ResultObj::Copy(Object::Numeric(Numeric::Int(index as i64))),
                None => ResultObj::Copy(Object::Null),
            },
            ref obj => missmatch_type("indice", &obj.get_type(), target_line, target_col),
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
    if args.len() != 0 {
        return missmatch_args(0, args.len(), "ordenar".len(), target_line, target_col);
    }

    match target {
        ResultObj::Copy(obj) => missmatch_type("ordenar", &obj.get_type(), target_line, target_col),
        ResultObj::Ref(mut ref_obj) => match *ref_obj.clone().borrow_mut() {
            Object::List(ref mut list) => {
                // if !args.is_empty() {
                //     let ord_func_expr = args.remove(0);
                //     let line = ord_func_expr.line;
                //     let col = ord_func_expr.col;
                //     let ord_func_obj = eval.eval_call(ord_func_expr, vec![], env);
                //     if eval.is_error(&ord_func_obj) {
                //         return ord_func_obj;
                //     }
                //     return ResultObj::Ref(ref_obj);
                // }
                match quick_sort(list) {
                    Some(res) => res,
                    None => ResultObj::Ref(ref_obj),
                }
            }
            Object::String(ref mut string) => {
                let mut chars = string.chars().collect::<Vec<char>>();
                chars.sort();
                ResultObj::Ref(new_rc_object(Object::String(
                    chars.iter().collect::<String>(),
                )))
            }
            ref obj => missmatch_type("ordenar", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(1, args.len(), "concatenar".len(), target_line, target_col);
    }
    let concat_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&concat_obj) {
        return concat_obj;
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("concatenar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => match concat_obj {
                ResultObj::Ref(list2_ref) => match *list2_ref.as_ref().borrow_mut() {
                    Object::List(ref mut list2) => {
                        list.append(list2);
                        ResultObj::Copy(Object::Void)
                    }
                    ref obj => {
                        missmatch_type_arg("lista", &obj.get_type(), target_line, target_col)
                    }
                },
                ResultObj::Copy(obj) => {
                    missmatch_type_arg("lista", &obj.get_type(), target_line, target_col)
                }
            },
            ref obj => missmatch_type("concatenar", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(1, args.len(), "eliminar".len(), target_line, target_col);
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
            return missmatch_type_arg("numerico entero", &obj.get_type(), target_line, target_col)
        }
    }

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("eliminar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref mut list) => {
                if (index as usize) < list.len() {
                    return list.remove(index as usize);
                }
                ResultObj::Copy(Object::Null)
            }
            ref obj => missmatch_type("eliminar", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(1, args.len(), "separar".len(), target_line, target_col);
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
                    "numerico entero",
                    &obj.get_type(),
                    target_line,
                    target_col,
                )
            }
        },
        obj => {
            return missmatch_type_arg("numerico entero", &obj.get_type(), target_line, target_col)
        }
    };

    match target {
        ResultObj::Copy(obj) => missmatch_type("separar", &obj.get_type(), target_line, target_col),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::List(ref list) => {
                let join = list
                    .iter()
                    .map(|obj| obj.to_string())
                    .collect::<Vec<_>>()
                    .join(&join);
                ResultObj::Ref(new_rc_object(Object::String(join)))
            }
            ref obj => missmatch_type("separar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn llaves(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "llaves".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type("llaves", &obj.get_type(), target_line, target_col),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::Dictionary(ref dict) => ResultObj::Ref(new_rc_object(Object::List(
                dict.keys().clone().map(|obj| obj.clone()).collect(),
            ))),
            ref obj => missmatch_type("llaves", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn valores(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "valores".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => missmatch_type("valores", &obj.get_type(), target_line, target_col),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::Dictionary(ref dict) => ResultObj::Ref(new_rc_object(Object::List(
                dict.values().map(|obj| obj.clone()).collect(),
            ))),
            ref obj => missmatch_type("valores", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(1, args.len(), "separar".len(), target_line, target_col);
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
                    "numerico entero",
                    &obj.get_type(),
                    target_line,
                    target_col,
                )
            }
        },
        obj => {
            return missmatch_type_arg("numerico entero", &obj.get_type(), target_line, target_col)
        }
    };

    match target {
        ResultObj::Copy(obj) => missmatch_type("separar", &obj.get_type(), target_line, target_col),
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => {
                let split = string
                    .split(&split)
                    .map(|w| ResultObj::Ref(new_rc_object(Object::String(w.into()))))
                    .collect::<Vec<_>>();
                ResultObj::Ref(new_rc_object(Object::List(split)))
            }
            ref obj => missmatch_type("separar", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(1, args.len(), "caracter".len(), target_line, target_col);
    }
    let index_obj = eval.eval_expression(args.remove(0), env);
    if eval.is_error(&index_obj) {
        return index_obj;
    }
    let index;
    if let ResultObj::Copy(Object::Numeric(Numeric::Int(int))) = index_obj {
        if int < 0 {
            return ResultObj::Copy(Object::Error(create_msg_err(
                "El indice debe ser un numero positivo.".into(),
                target_line,
                target_col,
            )));
        }
        index = int;
    } else {
        return missmatch_type_arg(
            "numerico entero",
            &index_obj.get_type(),
            target_line,
            target_col,
        );
    }

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("caracter", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => match string.chars().nth(index as usize) {
                Some(c) => ResultObj::Ref(new_rc_object(Object::String(c.to_string()))),
                None => ResultObj::Copy(Object::Null),
            },
            ref obj => missmatch_type("caracter", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn caracteres(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "caracteres".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("caracteres", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => ResultObj::Ref(new_rc_object(Object::List(
                string
                    .chars()
                    .map(|c| ResultObj::Copy(Object::Numeric(Numeric::Int(c as i64))))
                    .collect::<Vec<ResultObj>>(),
            ))),
            ref obj => missmatch_type("caracteres", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn es_alfabetico(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(
            0,
            args.len(),
            "es_alfabetico".len(),
            target_line,
            target_col,
        );
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("es_alfabetico", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => ResultObj::Copy(Object::Boolean(
                string.chars().fold(true, |acc, c| c.is_alphabetic() && acc),
            )),
            ref obj => missmatch_type("es_alfabetico", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn es_numerico(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "es_numerico".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("es_numerico", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => ResultObj::Copy(Object::Boolean(
                string.chars().fold(true, |acc, c| c.is_numeric() && acc),
            )),
            ref obj => missmatch_type("es_numerico", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn es_alfanumerico(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(
            0,
            args.len(),
            "es_alfanumerico".len(),
            target_line,
            target_col,
        );
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("es_alfanumerico", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => ResultObj::Copy(Object::Boolean(
                string
                    .chars()
                    .fold(true, |acc, c| c.is_alphanumeric() && acc),
            )),
            ref obj => missmatch_type("es_alfanumerico", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(1, args.len(), "inicia_con".len(), target_line, target_col);
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
                "cadena",
                &ref_pattern_obj.borrow().get_type(),
                target_line,
                target_col,
            );
        }
    } else {
        return missmatch_type_arg("cadena", &pattern_obj.get_type(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("inicia_con", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => {
                ResultObj::Copy(Object::Boolean(string.starts_with(&pattern)))
            }
            ref obj => missmatch_type("inicia_con", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(1, args.len(), "termina_con".len(), target_line, target_col);
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
                "cadena",
                &ref_pattern_obj.borrow().get_type(),
                target_line,
                target_col,
            );
        }
    } else {
        return missmatch_type_arg("cadena", &pattern_obj.get_type(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("termina_con", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => {
                ResultObj::Copy(Object::Boolean(string.starts_with(&pattern)))
            }
            ref obj => missmatch_type("termina_con", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn a_mayusculas(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "a_mayusculas".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("a_mayusculas", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref mut string) => {
                ResultObj::Ref(new_rc_object(Object::String(string.to_uppercase())))
            }
            ref obj => missmatch_type("a_mayusculas", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn a_minusculas(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "a_minusculas".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("a_minusculas", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref mut string) => {
                ResultObj::Ref(new_rc_object(Object::String(string.to_lowercase())))
            }
            ref obj => missmatch_type("a_minusculas", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(2, args.len(), "reemplazar".len(), target_line, target_col);
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
                    return missmatch_type_arg("cadena", &obj.get_type(), target_line, target_col);
                }
                (obj, Object::String(_)) => {
                    return missmatch_type_arg("cadena", &obj.get_type(), target_line, target_col);
                }
                (obj, _) => {
                    return missmatch_type_arg("cadena", &obj.get_type(), target_line, target_col);
                }
            }
        }
        (ResultObj::Ref(_), ResultObj::Copy(err_obj)) => {
            return missmatch_type_arg("cadena", &err_obj.get_type(), target_line, target_col);
        }
        (ResultObj::Copy(err_obj), ResultObj::Ref(_)) => {
            return missmatch_type_arg("cadena", &err_obj.get_type(), target_line, target_col);
        }
        (obj, _) => {
            return missmatch_type_arg("cadena", &obj.get_type(), target_line, target_col);
        }
    }

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("reemplazar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref mut string) => {
                *string = string.replace(&pattern, &new);
                ResultObj::Copy(Object::Void)
            }
            ref obj => missmatch_type("reemplazar", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn recortar(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "recortar".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("recortar", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref mut string) => {
                *string = string.trim().into();
                ResultObj::Copy(Object::Void)
            }
            ref obj => missmatch_type("recortar", &obj.get_type(), target_line, target_col),
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
        return missmatch_args(2, args.len(), "subcadena".len(), target_line, target_col);
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
                return ResultObj::Copy(Object::Error(create_msg_err(
                    "El indice debe ser un numero positivo.".into(),
                    target_line,
                    target_col,
                )));
            }
            pos = p;
            len = l;
        }
        (ResultObj::Copy(Object::Numeric(Numeric::Int(_))), err_obj) => {
            return missmatch_type_arg("cadena", &err_obj.get_type(), target_line, target_col);
        }
        (err_obj, _) => {
            return missmatch_type_arg("cadena", &err_obj.get_type(), target_line, target_col);
        }
    }

    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("subcadena", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => {
                if (pos as usize) > string.len() {
                    return ResultObj::Copy(Object::Error(create_msg_err(
                        "El indice esta fuera del rango.".into(),
                        target_line,
                        target_col,
                    )));
                }
                if len > (string.len() as i64) - pos {
                    return ResultObj::Copy(Object::Error(create_msg_err(
                        "El indice esta fuera del rango.".into(),
                        target_line,
                        target_col,
                    )));
                }

                let sub_str: String = string
                    .chars()
                    .skip(pos as usize)
                    .take(len as usize)
                    .collect();
                ResultObj::Ref(new_rc_object(Object::String(sub_str)))
            }
            ref obj => missmatch_type("subcadena", &obj.get_type(), target_line, target_col),
        },
    }
}

pub fn a_numerico(
    args: FnParams,
    target: ResultObj,
    target_line: usize,
    target_col: usize,
) -> ResultObj {
    if !args.is_empty() {
        return missmatch_args(0, args.len(), "a_numerico".len(), target_line, target_col);
    }
    match target {
        ResultObj::Copy(obj) => {
            missmatch_type("a_numerico", &obj.get_type(), target_line, target_col)
        }
        ResultObj::Ref(ref_obj) => match *ref_obj.as_ref().borrow_mut() {
            Object::String(ref string) => {
                let mut lexer = Lexer::new(string.chars().collect());
                let token = lexer.next_token();
                if let TokenType::Numeric(num) = token.r#type {
                    return ResultObj::Copy(Object::Numeric(num));
                } else if let TokenType::Illegal(c) = token.r#type {
                    return ResultObj::Copy(Object::Error(create_msg_err(
                        format!(
                            "Se encontro un simbolo ilegal `{}` durante la conversion",
                            c
                        ),
                        target_line,
                        target_col,
                    )));
                }
                missmatch_type(
                    "a_numerico",
                    &ref_obj.borrow().get_type(),
                    target_line,
                    target_col,
                )
            }
            ref obj => missmatch_type("a_numerico", &obj.get_type(), target_line, target_col),
        },
    }
}
