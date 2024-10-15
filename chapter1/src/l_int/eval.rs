use super::{
    errors::Error,
    exp::Exp,
    functions::{Call, ExpFunction, StmtFunction},
    stmt::Stmt,
    BinOp, Module, UnaryOp,
};
use std::io::stdin;

pub fn interp_exp(e: Exp) -> Result<i32, Error> {
    match e {
        Exp::Constant(i) => Ok(i),
        Exp::Call(Call {
            name: ExpFunction::InputInt,
            args,
        }) => {
            if args.is_empty() {
                Ok(())
            } else {
                Err(Error::WrongNumArgsExp {
                    name: ExpFunction::InputInt,
                    found: args.len(),
                    expected: 0_usize,
                })
            }?;

            let mut inp: String = String::default();
            stdin().read_line(&mut inp).unwrap_or_default();
            let mut res = inp.trim_end().parse::<i32>();
            while res.is_err() {
                stdin().read_line(&mut inp).unwrap_or_default();
                res = inp.trim_end().parse::<i32>();
            }
            Ok(res.unwrap())
        }
        Exp::UnaryOp { op, exp } => match op {
            UnaryOp::Neg => {
                let res = interp_exp(*exp)?;
                Ok(-res)
            }
        },
        Exp::BinOp { op, exp1, exp2 } => {
            let i1 = interp_exp(*exp1)?;
            let i2 = interp_exp(*exp2)?;
            match op {
                BinOp::Sub => Ok(i1 - i2),
                BinOp::Add => Ok(i1 + i2),
            }
        }
    }
}

pub fn interp_stmt(st: Stmt) -> Result<Option<i32>, Error> {
    match st {
        Stmt::Call(Call {
            name: StmtFunction::Print,
            args,
        }) => {
            let arg = args.first().unwrap();
            println!("{}", arg);
            Ok(None)
        }
        Stmt::Exp(e) => interp_exp(e).map(Some),
    }
}

pub fn interp_lint(m: Module) -> Result<Vec<i32>, Error> {
    let mut results = vec![];
    for stmt in m.into_iter() {
        let res = interp_stmt(stmt)?;
        if let Some(i) = res {
            results.push(i)
        }
    }
    Ok(results)
}
