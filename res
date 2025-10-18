  left: Program { blocks: [
  Block { label: "start", tail: Tail { stmts: [Assign { var: "x", bound: At m(Integer(0)) }, Assign { var: "y", bound: Atm(Integer(5)) }, Assign { var: "z", bound: Cmp { left: Variable("x"), cmp: Lt, right: Integer(1) } }], cont: If { cond: Variable("z"), then_label: "block_0 ", else_label: "block_1" } } },
  Block { label: "block_0", tail: Tail { stmts: [Assign { var: "w", bo und: Cmp { left: Variable("x"), cmp: Eq, right: Integer(0) } }], cont: If { cond: Variable("w"), the n_label: "block_2", else_label: "block_3" } } },
  Block { label: "block_2", tail: Tail { stmts: [Assi gn { var: "z", bound: BinOp { fst: Variable("y"), op: Add, snd: Integer(2) } }, Print(Variable("z")) ], cont: Return(Unit) } },
  Block { label: "block_3", tail: Tail { stmts: [Print(Variable("y"))], con t: Return(Unit) } },
  Block { label: "block_1", tail: Tail { stmts: [Assign { var: "z", bound: BinOp { fst: Variable("y"), op: Add, snd: Integer(10) } }, Print(Variable("z"))], cont: Return(Unit) } }] }
