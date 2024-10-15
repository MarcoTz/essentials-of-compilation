use super::{
    errors::Error, syntax::Exp as FullExp, syntax::Module as FullModule, syntax::Stmt as FullStmt,
    BinOp, UnaryOp, Var,
};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone)]
pub enum Atm {
    Constant(i64),
    Name(Var),
}

impl fmt::Display for Atm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atm::Constant(i) => i.fmt(f),
            Atm::Name(var) => var.fmt(f),
        }
    }
}

impl TryFrom<Exp> for Atm {
    type Error = Error;
    fn try_from(exp: Exp) -> Result<Atm, Self::Error> {
        if let Exp::Atm(at) = exp {
            Ok(at)
        } else {
            Err(Error::NotAnAtom(exp))
        }
    }
}

impl TryFrom<Stmt> for Atm {
    type Error = Error;
    fn try_from(st: Stmt) -> Result<Atm, Self::Error> {
        let exp = <Stmt as TryInto<Exp>>::try_into(st)?;
        exp.try_into()
    }
}

#[derive(Debug, Clone)]
pub enum Exp {
    Atm(Atm),
    InputInt,
    UnaryOp { op: UnaryOp, exp: Atm },
    BinOp { exp1: Atm, op: BinOp, exp2: Atm },
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Atm(at) => at.fmt(f),
            Exp::InputInt => f.write_str("input_int"),
            Exp::UnaryOp { op, exp } => write!(f, "{op}{exp}"),
            Exp::BinOp { exp1, op, exp2 } => write!(f, "{exp1}{op}{exp2}"),
        }
    }
}

impl From<Atm> for Exp {
    fn from(at: Atm) -> Exp {
        Exp::Atm(at)
    }
}

impl TryFrom<Stmt> for Exp {
    type Error = Error;
    fn try_from(st: Stmt) -> Result<Exp, Self::Error> {
        match st {
            Stmt::Print(_) => Err(Error::NotAnExpression(st)),
            Stmt::Exp(e) => Ok(e),
            Stmt::Assign(_, _) => Err(Error::NotAnExpression(st)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Vec<Atm>),
    Exp(Exp),
    Assign(Var, Exp),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Print(atms) => write!(
                f,
                "print({})",
                atms.iter()
                    .map(|at| format!("{}", at))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Stmt::Exp(e) => e.fmt(f),
            Stmt::Assign(var, exp) => write!(f, "{var} = {exp}"),
        }
    }
}

pub type Module = Vec<Stmt>;

impl From<Exp> for Stmt {
    fn from(exp: Exp) -> Stmt {
        Stmt::Exp(exp)
    }
}

impl From<Atm> for Stmt {
    fn from(at: Atm) -> Stmt {
        Stmt::Exp(at.into())
    }
}

pub struct RemoveState {
    assigned_vars: HashSet<Var>,
}

impl RemoveState {
    pub fn fresh_var(&mut self) -> Var {
        let prefix = "x".to_owned();
        let mut i = 0;
        let mut new_var = prefix.clone() + &i.to_string();
        while self.assigned_vars.contains(&new_var) {
            i += 1;
            new_var = prefix.clone() + &i.to_string();
        }
        new_var
    }
}

pub fn remove_complex_operands(m: FullModule, st: &mut RemoveState) -> Module {
    let mut new_mod = vec![];
    for full_stmt in m.into_iter() {
        let new_sts = remove_operands_st(full_stmt, st);
        new_mod.extend(new_sts)
    }
    new_mod
}

pub fn remove_operands_st(stmt: FullStmt, st: &mut RemoveState) -> Module {
    match stmt {
        FullStmt::Exp(exp) => remove_operands_exp(exp, st),
        FullStmt::Assign { var, exp } => {
            let mut exp_conv = remove_operands_exp(exp, st);
            let last = exp_conv.pop().unwrap();
            exp_conv.push(Stmt::Assign(var, last.try_into().unwrap()));
            exp_conv
        }
        FullStmt::Print(exp) => {
            let mut new_stmts = remove_operands_exp(exp, st);
            let new_print = new_stmts.pop().unwrap();
            let new_stmt = Stmt::Print(vec![new_print.try_into().unwrap()]);
            new_stmts.push(new_stmt);
            new_stmts
        }
    }
}

pub fn remove_operands_exp(exp: FullExp, st: &mut RemoveState) -> Module {
    match exp {
        FullExp::Name(name) => vec![Atm::Name(name).into()],
        FullExp::Constant(i) => vec![Atm::Constant(i).into()],
        FullExp::InputInt => {
            let new_exp = Exp::InputInt;
            let new_name = st.fresh_var();
            let assign = Stmt::Assign(new_name.clone(), new_exp.clone());
            let atm = Atm::Name(new_name);
            vec![new_exp.into(), assign, atm.into()]
        }
        FullExp::UnaryOp { op, exp } => {
            let mut new_mod = remove_operands_exp(*exp, st);
            let last_st = new_mod.pop().unwrap();
            let new_name = st.fresh_var();
            let new_assign = Stmt::Assign(new_name.clone(), last_st.try_into().unwrap());
            let new_atm = Atm::Name(new_name);
            let new_op = Exp::UnaryOp {
                op,
                exp: new_atm.clone(),
            };
            new_mod.push(new_assign);
            new_mod.push(new_op.into());
            new_mod.push(new_atm.into());
            new_mod
        }
        FullExp::BinOp { exp1, op, exp2 } => {
            let mut new_mod1 = remove_operands_exp(*exp1, st);
            let last_st1 = new_mod1.pop().unwrap();
            let new_name1 = st.fresh_var();
            let new_assign1 = Stmt::Assign(new_name1.clone(), last_st1.try_into().unwrap());
            let new_atm1 = Atm::Name(new_name1);

            let mut new_mod2 = remove_operands_exp(*exp2, st);
            let last_st2 = new_mod2.pop().unwrap();
            let new_name2 = st.fresh_var();
            let new_assign2 = Stmt::Assign(new_name2.clone(), last_st2.try_into().unwrap());
            let new_atm2 = Atm::Name(new_name2);

            let new_op = Exp::BinOp {
                op,
                exp1: new_atm1.clone(),
                exp2: new_atm2.clone(),
            };

            let mut new_mod = new_mod1;
            new_mod.extend(new_mod2);
            new_mod.push(new_assign1);
            new_mod.push(new_assign2);
            new_mod.push(new_atm1.into());
            new_mod.push(new_atm2.into());
            new_mod.push(new_op.into());
            new_mod
        }
    }
}
