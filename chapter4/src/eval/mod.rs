use crate::{
    errors::Error,
    syntax::{Exp, Op, Program},
};
use std::io::stdin;

pub mod value;
pub use value::Value;

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
                    Ok(Value::Bool(cmp.apply(fst, snd)))
                }
            }
        }
        Exp::Let {
            var,
            bound_exp,
            in_exp,
        } => {
            let bound_evaled = eval(*bound_exp)?;
            eval(in_exp.subst(&var, bound_evaled.into()))
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
