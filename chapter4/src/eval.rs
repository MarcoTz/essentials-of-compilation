use crate::{
    errors::Error,
    syntax::{types::Type, Cmp, Exp, Op, Program, Var},
};
use std::{fmt, io::stdin};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

impl Value {
    fn as_int(self) -> Result<i64, Error> {
        if let Value::Int(i) = self {
            Ok(i)
        } else {
            Err(Error::BadValue {
                found: self,
                expected: Type::Int,
            })
        }
    }

    fn as_bool(self) -> Result<bool, Error> {
        if let Value::Bool(b) = self {
            Ok(b)
        } else {
            Err(Error::BadValue {
                found: self,
                expected: Type::Bool,
            })
        }
    }
}

pub fn eval(prog: Program) -> Result<Value, Error> {
    match prog {
        Exp::Int(i) => Ok(Value::Int(i)),
        Exp::Bool(b) => Ok(Value::Bool(b)),
        Exp::Var(v) => Err(Error::FreeVar(v)),
        Exp::Prim { op, args } => {
            check_arity(op, args.len())?;
            let evaled = args
                .into_iter()
                .map(eval)
                .collect::<Result<Vec<Value>, Error>>()?;
            match op {
                Op::Read => Ok(Value::Int(get_input())),
                Op::Plus => {
                    let fst = evaled[0].as_int()?;
                    let snd = evaled[1].as_int()?;
                    Ok(Value::Int(fst + snd))
                }
                Op::Sub => {
                    let fst = evaled[0].as_int()?;
                    let snd = evaled[1].as_int()?;
                    Ok(Value::Int(fst - snd))
                }
                Op::Neg => {
                    let arg = evaled[0].as_int()?;
                    Ok(Value::Int(-1 * arg))
                }
                Op::And => {
                    let fst = evaled[0].as_bool()?;
                    let snd = evaled[1].as_bool()?;
                    Ok(Value::Bool(fst && snd))
                }
                Op::Or => {
                    let fst = evaled[0].as_bool()?;
                    let snd = evaled[1].as_bool()?;
                    Ok(Value::Bool(fst || snd))
                }
                Op::Not => {
                    let arg = evaled[0].as_bool()?;
                    Ok(Value::Bool(!arg))
                }
                Op::Cmp(cmp) => {
                    let fst = evaled[0].as_int()?;
                    let snd = evaled[1].as_int()?;
                    match cmp {
                        Cmp::Equal => Ok(Value::Bool(fst == snd)),
                        Cmp::Less => Ok(Value::Bool(fst < snd)),
                        Cmp::LessEq => Ok(Value::Bool(fst <= snd)),
                        Cmp::Greater => Ok(Value::Bool(fst > snd)),
                        Cmp::GreaterEq => Ok(Value::Bool(fst >= snd)),
                    }
                }
            }
        }
        Exp::Let {
            var,
            bound_exp,
            in_exp,
        } => {
            let bound_evaled = eval(*bound_exp)?;
            eval(subst(*in_exp, var, bound_evaled))
        }
        Exp::If { ifc, thenc, elsec } => {
            let if_evaled = eval(*ifc)?.as_bool()?;
            if if_evaled {
                eval(*thenc)
            } else {
                eval(*elsec)
            }
        }
    }
}
impl From<Value> for Exp {
    fn from(val: Value) -> Exp {
        match val {
            Value::Int(i) => Exp::Int(i),
            Value::Bool(b) => Exp::Bool(b),
        }
    }
}

fn subst(exp: Exp, v: Var, val: Value) -> Exp {
    match exp {
        Exp::Int(_) | Exp::Bool(_) => exp,
        Exp::Var(var) => {
            if var == v {
                val.into()
            } else {
                Exp::Var(var)
            }
        }
        Exp::Prim { op, args } => Exp::Prim {
            op,
            args: args
                .into_iter()
                .map(|exp| subst(exp, v.clone(), val))
                .collect(),
        },
        Exp::Let {
            var,
            bound_exp,
            in_exp,
        } => {
            if var == v {
                Exp::Let {
                    var,
                    bound_exp,
                    in_exp,
                }
            } else {
                Exp::Let {
                    var,
                    bound_exp: Box::new(subst(*bound_exp, v.clone(), val)),
                    in_exp: Box::new(subst(*in_exp, v, val)),
                }
            }
        }
        Exp::If { ifc, thenc, elsec } => Exp::If {
            ifc: Box::new(subst(*ifc, v.clone(), val)),
            thenc: Box::new(subst(*thenc, v.clone(), val)),
            elsec: Box::new(subst(*elsec, v, val)),
        },
    }
}

fn check_arity(op: Op, num_args: usize) -> Result<(), Error> {
    let arity = op.arity();
    if arity == num_args {
        Ok(())
    } else {
        Err(Error::ArityMismatch {
            op,
            found: num_args,
            expected: arity,
        })
    }
}

fn get_input() -> i64 {
    let mut inp: String = String::default();
    stdin().read_line(&mut inp).unwrap_or_default();
    match inp.trim_end().parse::<i64>() {
        Ok(i) => i,
        Err(_) => get_input(),
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(i) => f.write_str(&i.to_string()),
            Value::Bool(b) => f.write_str(&b.to_string()),
        }
    }
}
