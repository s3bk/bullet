use builder::{Builder, NodeResult};
use func::Func;
use eval::Command;
use error::Error;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Array {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22bench_22(&'input str),
        Term_22def_22(&'input str),
        Term_22eval_22(&'input str),
        Term_22_b7_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cpL_2b_22_23(&'input str),
        Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(NodeResult),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<NodeResult>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<NodeResult>),
        Nt_28_3cName_3e_20_22_2c_22_29(&'input str),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'input str>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'input str>),
        NtArray(NodeResult),
        NtComma_3cExpr_3e(Vec<NodeResult>),
        NtComma_3cName_3e(Vec<&'input str>),
        NtCommaE(Vec<NodeResult>),
        NtCommaS(Vec<&'input str>),
        NtCommand(Result<Command<'input>, Error>),
        NtExpr(NodeResult),
        NtFactor(NodeResult),
        NtName(&'input str),
        NtNum(NodeResult),
        NtNumFloat(NodeResult),
        NtOp(Func),
        NtPow(NodeResult),
        NtTerm(NodeResult),
        NtTuple(NodeResult),
        NtVar(NodeResult),
        Nt____Array(NodeResult),
        Nt____CommaE(Vec<NodeResult>),
        Nt____CommaS(Vec<&'input str>),
        Nt____Command(Result<Command<'input>, Error>),
        Nt____Expr(NodeResult),
        Nt____Factor(NodeResult),
        Nt____Op(Func),
        Nt____Pow(NodeResult),
        Nt____Term(NodeResult),
        Nt____Tuple(NodeResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44,
        // State 2
        17, 0, 0, 0, 0, 18, 0, 0, 3, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 3
        17, 0, 0, 0, 0, 18, 0, 0, 3, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 4
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 5
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, -12, 0, 24, 25, 26, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, -25, 27, -25, -25, -25, 28, 0, 0, -25, 0, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 9
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 10
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 11
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 12
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 13
        17, -36, -36, -36, -36, -36, -36, 0, 3, -36, 31, 0, 0, 0, -36, 19, 20, 21, 0,
        // State 14
        -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 15
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 16
        17, 0, 0, 0, 0, 18, 0, 0, 3, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 17
        17, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 18
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 19
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 20
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 21
        0, -13, 0, 24, 34, 26, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 23
        17, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 24
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 25
        17, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 26
        17, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 27
        17, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 28
        17, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 29
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 30
        17, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 31
        0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, -24, 27, -24, -24, -24, 28, 0, 0, -24, 0, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 33
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 34
        0, -22, 27, -22, -22, -22, 28, 0, 0, -22, 0, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 35
        0, -23, 27, -23, -23, -23, 28, 0, 0, -23, 0, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 36
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 37
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 38
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 39
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 40
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -44,
        0,
        0,
        -41,
        -16,
        0,
        0,
        0,
        -43,
        -37,
        -38,
        -29,
        0,
        -40,
        -39,
        0,
        0,
        -31,
        -32,
        -30,
        0,
        -11,
        0,
        -4,
        0,
        0,
        0,
        0,
        -35,
        0,
        0,
        0,
        -5,
        0,
        0,
        -26,
        -28,
        -27,
        -34,
        -42,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 4, 0, 0, 0, 5, 6, 0, 7, 0, 0, 8, 9, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 22, 9, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 30, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 4, 0, 0, 0, 5, 6, 0, 32, 0, 0, 8, 9, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 33, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 35, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 36, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 37, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 38, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 39, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 40, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bench""###,
            r###""def""###,
            r###""eval""###,
            r###""·""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+\\.[0-9]+"#"###,
            r###"r#"\\pL+"#"###,
            r###"r#"d/d(\\pL+)"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Array<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
    ) -> Result<NodeResult, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
                (2, _) if true => 17,
                (3, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3a_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22bench_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22def_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22eval_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_b7_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5cpL_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<NodeResult,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(51);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Name> ",") = Name, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Name> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")+ = Name, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Array = "[", CommaE, "]" => ActionFn(24);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArray(__nt), __end));
                6
            }
            12 => {
                // Comma<Expr> = Expr => ActionFn(53);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            13 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(54);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            14 => {
                // Comma<Name> = Name => ActionFn(57);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            15 => {
                // Comma<Name> = (<Name> ",")+, Name => ActionFn(58);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            16 => {
                // CommaE = Comma<Expr> => ActionFn(22);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaE(__nt), __end));
                9
            }
            17 => {
                // CommaS = Comma<Name> => ActionFn(34);
                let __sym0 = __pop_NtComma_3cName_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaS(__nt), __end));
                10
            }
            18 => {
                // Command = "def", r#"\\pL+"#, "(", CommaS, ")", ":=", Expr => ActionFn(35);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22_3a_3d_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtCommaS(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __sym0 = __pop_Term_22def_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action35::<>(builder, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            19 => {
                // Command = "eval", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22eval_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            20 => {
                // Command = "bench", Expr => ActionFn(37);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22bench_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            21 => {
                // Command = Expr => ActionFn(38);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            22 => {
                // Expr = Expr, "+", Factor => ActionFn(10);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            23 => {
                // Expr = Expr, "-", Factor => ActionFn(11);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            24 => {
                // Expr = "-", Factor => ActionFn(12);
                let __sym1 = __pop_NtFactor(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            25 => {
                // Expr = Factor => ActionFn(13);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            26 => {
                // Factor = Factor, "*", Pow => ActionFn(14);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            27 => {
                // Factor = Factor, "·", Pow => ActionFn(15);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_b7_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            28 => {
                // Factor = Factor, "/", Pow => ActionFn(16);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            29 => {
                // Factor = Pow => ActionFn(17);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            30 => {
                // Name = r#"\\pL+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                14
            }
            31 => {
                // Num = r#"[0-9]+"# => ActionFn(30);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                15
            }
            32 => {
                // NumFloat = r#"[0-9]+\\.[0-9]+"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFloat(__nt), __end));
                16
            }
            33 => {
                // Op = r#"d/d(\\pL+)"# => ActionFn(18);
                let __sym0 = __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                17
            }
            34 => {
                // Pow = Term, "^", Pow => ActionFn(19);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            35 => {
                // Pow = Term, Pow => ActionFn(20);
                let __sym1 = __pop_NtPow(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            36 => {
                // Pow = Term => ActionFn(21);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            37 => {
                // Term = Num => ActionFn(25);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            38 => {
                // Term = NumFloat => ActionFn(26);
                let __sym0 = __pop_NtNumFloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            39 => {
                // Term = Var => ActionFn(27);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            40 => {
                // Term = Tuple => ActionFn(28);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            41 => {
                // Term = Array => ActionFn(29);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            42 => {
                // Tuple = "(", CommaE, ")" => ActionFn(23);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTuple(__nt), __end));
                20
            }
            43 => {
                // Var = Name => ActionFn(32);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                21
            }
            44 => {
                // __Array = Array => ActionFn(6);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            45 => {
                // __CommaE = CommaE => ActionFn(4);
                let __sym0 = __pop_NtCommaE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaE(__nt), __end));
                23
            }
            46 => {
                // __CommaS = CommaS => ActionFn(8);
                let __sym0 = __pop_NtCommaS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaS(__nt), __end));
                24
            }
            47 => {
                // __Command = Command => ActionFn(9);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Command(__nt), __end));
                25
            }
            48 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                26
            }
            49 => {
                // __Factor = Factor => ActionFn(1);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Factor(__nt), __end));
                27
            }
            50 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                28
            }
            51 => {
                // __Pow = Pow => ActionFn(3);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Pow(__nt), __end));
                29
            }
            52 => {
                // __Term = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                30
            }
            53 => {
                // __Tuple = Tuple => ActionFn(5);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Tuple(__nt), __end));
                31
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bench_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bench_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22def_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22def_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22eval_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22eval_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_b7_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_b7_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cpL_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cpL_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArray<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArray(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cName_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cName_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumFloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Array<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Array(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Command<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Command(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Factor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Factor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Pow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Pow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Tuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Tuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Array::parse_Array;

mod __parse__CommaE {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22bench_22(&'input str),
        Term_22def_22(&'input str),
        Term_22eval_22(&'input str),
        Term_22_b7_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cpL_2b_22_23(&'input str),
        Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(NodeResult),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<NodeResult>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<NodeResult>),
        Nt_28_3cName_3e_20_22_2c_22_29(&'input str),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'input str>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'input str>),
        NtArray(NodeResult),
        NtComma_3cExpr_3e(Vec<NodeResult>),
        NtComma_3cName_3e(Vec<&'input str>),
        NtCommaE(Vec<NodeResult>),
        NtCommaS(Vec<&'input str>),
        NtCommand(Result<Command<'input>, Error>),
        NtExpr(NodeResult),
        NtFactor(NodeResult),
        NtName(&'input str),
        NtNum(NodeResult),
        NtNumFloat(NodeResult),
        NtOp(Func),
        NtPow(NodeResult),
        NtTerm(NodeResult),
        NtTuple(NodeResult),
        NtVar(NodeResult),
        Nt____Array(NodeResult),
        Nt____CommaE(Vec<NodeResult>),
        Nt____CommaS(Vec<&'input str>),
        Nt____Command(Result<Command<'input>, Error>),
        Nt____Expr(NodeResult),
        Nt____Factor(NodeResult),
        Nt____Op(Func),
        Nt____Pow(NodeResult),
        Nt____Term(NodeResult),
        Nt____Tuple(NodeResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        15, 0, 0, 0, 0, 16, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 1
        15, 0, 0, 0, 0, 16, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 2
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 3
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 4
        -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45,
        // State 5
        0, -12, 0, 22, 23, 24, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, -25, 25, -25, -25, -25, 26, 0, 0, -25, 0, 0, 0, 0, 27, 0, 0, 0, 0,
        // State 7
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 8
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 9
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 10
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 11
        15, -36, -36, -36, -36, -36, -36, 0, 17, -36, 29, 0, 0, 0, -36, 18, 19, 20, 0,
        // State 12
        -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 13
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 14
        15, 0, 0, 0, 0, 16, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 15
        15, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 16
        15, 0, 0, 0, 0, 16, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 17
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 18
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 19
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 20
        0, -13, 0, 22, 33, 24, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        15, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 22
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 23
        15, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 24
        15, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 25
        15, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 26
        15, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 27
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 28
        15, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 29
        0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -24, 25, -24, -24, -24, 26, 0, 0, -24, 0, 0, 0, 0, 27, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 33
        0, -22, 25, -22, -22, -22, 26, 0, 0, -22, 0, 0, 0, 0, 27, 0, 0, 0, 0,
        // State 34
        0, -23, 25, -23, -23, -23, 26, 0, 0, -23, 0, 0, 0, 0, 27, 0, 0, 0, 0,
        // State 35
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 36
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 37
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 38
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 39
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 40
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        0,
        -41,
        -16,
        -45,
        -12,
        -25,
        -43,
        -37,
        -38,
        -29,
        -36,
        -40,
        -39,
        0,
        0,
        0,
        -31,
        -32,
        -30,
        -13,
        0,
        -4,
        0,
        0,
        0,
        0,
        -35,
        0,
        0,
        -24,
        0,
        -5,
        -22,
        -23,
        -26,
        -28,
        -27,
        -34,
        -42,
        -11,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 0, 0, 0, 3, 4, 0, 5, 0, 0, 6, 7, 8, 9, 10, 0, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 21, 7, 8, 9, 10, 0, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 0, 28, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 2, 0, 0, 0, 3, 4, 0, 30, 0, 0, 6, 7, 8, 9, 10, 0, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 31, 8, 9, 10, 0, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 2, 0, 0, 0, 3, 4, 0, 32, 0, 0, 6, 7, 8, 9, 10, 0, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 34, 8, 9, 10, 0, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 35, 8, 9, 10, 0, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 0, 36, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 0, 37, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 0, 38, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 0, 39, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bench""###,
            r###""def""###,
            r###""eval""###,
            r###""·""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+\\.[0-9]+"#"###,
            r###"r#"\\pL+"#"###,
            r###"r#"d/d(\\pL+)"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_CommaE<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
    ) -> Result<Vec<NodeResult>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
                (2, _) if true => 17,
                (3, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3a_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22bench_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22def_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22eval_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_b7_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5cpL_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<NodeResult>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(51);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Name> ",") = Name, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Name> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")+ = Name, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Array = "[", CommaE, "]" => ActionFn(24);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArray(__nt), __end));
                6
            }
            12 => {
                // Comma<Expr> = Expr => ActionFn(53);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            13 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(54);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            14 => {
                // Comma<Name> = Name => ActionFn(57);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            15 => {
                // Comma<Name> = (<Name> ",")+, Name => ActionFn(58);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            16 => {
                // CommaE = Comma<Expr> => ActionFn(22);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaE(__nt), __end));
                9
            }
            17 => {
                // CommaS = Comma<Name> => ActionFn(34);
                let __sym0 = __pop_NtComma_3cName_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaS(__nt), __end));
                10
            }
            18 => {
                // Command = "def", r#"\\pL+"#, "(", CommaS, ")", ":=", Expr => ActionFn(35);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22_3a_3d_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtCommaS(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __sym0 = __pop_Term_22def_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action35::<>(builder, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            19 => {
                // Command = "eval", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22eval_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            20 => {
                // Command = "bench", Expr => ActionFn(37);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22bench_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            21 => {
                // Command = Expr => ActionFn(38);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            22 => {
                // Expr = Expr, "+", Factor => ActionFn(10);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            23 => {
                // Expr = Expr, "-", Factor => ActionFn(11);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            24 => {
                // Expr = "-", Factor => ActionFn(12);
                let __sym1 = __pop_NtFactor(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            25 => {
                // Expr = Factor => ActionFn(13);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            26 => {
                // Factor = Factor, "*", Pow => ActionFn(14);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            27 => {
                // Factor = Factor, "·", Pow => ActionFn(15);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_b7_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            28 => {
                // Factor = Factor, "/", Pow => ActionFn(16);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            29 => {
                // Factor = Pow => ActionFn(17);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            30 => {
                // Name = r#"\\pL+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                14
            }
            31 => {
                // Num = r#"[0-9]+"# => ActionFn(30);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                15
            }
            32 => {
                // NumFloat = r#"[0-9]+\\.[0-9]+"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFloat(__nt), __end));
                16
            }
            33 => {
                // Op = r#"d/d(\\pL+)"# => ActionFn(18);
                let __sym0 = __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                17
            }
            34 => {
                // Pow = Term, "^", Pow => ActionFn(19);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            35 => {
                // Pow = Term, Pow => ActionFn(20);
                let __sym1 = __pop_NtPow(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            36 => {
                // Pow = Term => ActionFn(21);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            37 => {
                // Term = Num => ActionFn(25);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            38 => {
                // Term = NumFloat => ActionFn(26);
                let __sym0 = __pop_NtNumFloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            39 => {
                // Term = Var => ActionFn(27);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            40 => {
                // Term = Tuple => ActionFn(28);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            41 => {
                // Term = Array => ActionFn(29);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            42 => {
                // Tuple = "(", CommaE, ")" => ActionFn(23);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTuple(__nt), __end));
                20
            }
            43 => {
                // Var = Name => ActionFn(32);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                21
            }
            44 => {
                // __Array = Array => ActionFn(6);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Array(__nt), __end));
                22
            }
            45 => {
                // __CommaE = CommaE => ActionFn(4);
                let __sym0 = __pop_NtCommaE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            46 => {
                // __CommaS = CommaS => ActionFn(8);
                let __sym0 = __pop_NtCommaS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaS(__nt), __end));
                24
            }
            47 => {
                // __Command = Command => ActionFn(9);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Command(__nt), __end));
                25
            }
            48 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                26
            }
            49 => {
                // __Factor = Factor => ActionFn(1);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Factor(__nt), __end));
                27
            }
            50 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                28
            }
            51 => {
                // __Pow = Pow => ActionFn(3);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Pow(__nt), __end));
                29
            }
            52 => {
                // __Term = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                30
            }
            53 => {
                // __Tuple = Tuple => ActionFn(5);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Tuple(__nt), __end));
                31
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bench_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bench_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22def_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22def_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22eval_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22eval_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_b7_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_b7_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cpL_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cpL_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArray<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArray(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cName_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cName_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumFloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Array<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Array(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Command<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Command(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Factor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Factor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Pow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Pow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Tuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Tuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__CommaE::parse_CommaE;

mod __parse__CommaS {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22bench_22(&'input str),
        Term_22def_22(&'input str),
        Term_22eval_22(&'input str),
        Term_22_b7_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cpL_2b_22_23(&'input str),
        Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(NodeResult),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<NodeResult>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<NodeResult>),
        Nt_28_3cName_3e_20_22_2c_22_29(&'input str),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'input str>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'input str>),
        NtArray(NodeResult),
        NtComma_3cExpr_3e(Vec<NodeResult>),
        NtComma_3cName_3e(Vec<&'input str>),
        NtCommaE(Vec<NodeResult>),
        NtCommaS(Vec<&'input str>),
        NtCommand(Result<Command<'input>, Error>),
        NtExpr(NodeResult),
        NtFactor(NodeResult),
        NtName(&'input str),
        NtNum(NodeResult),
        NtNumFloat(NodeResult),
        NtOp(Func),
        NtPow(NodeResult),
        NtTerm(NodeResult),
        NtTuple(NodeResult),
        NtVar(NodeResult),
        Nt____Array(NodeResult),
        Nt____CommaE(Vec<NodeResult>),
        Nt____CommaS(Vec<&'input str>),
        Nt____Command(Result<Command<'input>, Error>),
        Nt____Expr(NodeResult),
        Nt____Factor(NodeResult),
        Nt____Op(Func),
        Nt____Pow(NodeResult),
        Nt____Term(NodeResult),
        Nt____Tuple(NodeResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0,
        // State 2
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 3
        -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46,
        // State 4
        0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 6
        0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 8
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        0,
        -17,
        -46,
        -14,
        -30,
        -15,
        -9,
        -10,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bench""###,
            r###""def""###,
            r###""eval""###,
            r###""·""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+\\.[0-9]+"#"###,
            r###"r#"\\pL+"#"###,
            r###"r#"d/d(\\pL+)"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_CommaS<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
    ) -> Result<Vec<&'input str>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
                (2, _) if true => 17,
                (3, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3a_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22bench_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22def_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22eval_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_b7_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5cpL_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<&'input str>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(51);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Name> ",") = Name, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Name> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")+ = Name, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Array = "[", CommaE, "]" => ActionFn(24);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArray(__nt), __end));
                6
            }
            12 => {
                // Comma<Expr> = Expr => ActionFn(53);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            13 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(54);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            14 => {
                // Comma<Name> = Name => ActionFn(57);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            15 => {
                // Comma<Name> = (<Name> ",")+, Name => ActionFn(58);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            16 => {
                // CommaE = Comma<Expr> => ActionFn(22);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaE(__nt), __end));
                9
            }
            17 => {
                // CommaS = Comma<Name> => ActionFn(34);
                let __sym0 = __pop_NtComma_3cName_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaS(__nt), __end));
                10
            }
            18 => {
                // Command = "def", r#"\\pL+"#, "(", CommaS, ")", ":=", Expr => ActionFn(35);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22_3a_3d_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtCommaS(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __sym0 = __pop_Term_22def_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action35::<>(builder, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            19 => {
                // Command = "eval", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22eval_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            20 => {
                // Command = "bench", Expr => ActionFn(37);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22bench_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            21 => {
                // Command = Expr => ActionFn(38);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            22 => {
                // Expr = Expr, "+", Factor => ActionFn(10);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            23 => {
                // Expr = Expr, "-", Factor => ActionFn(11);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            24 => {
                // Expr = "-", Factor => ActionFn(12);
                let __sym1 = __pop_NtFactor(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            25 => {
                // Expr = Factor => ActionFn(13);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            26 => {
                // Factor = Factor, "*", Pow => ActionFn(14);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            27 => {
                // Factor = Factor, "·", Pow => ActionFn(15);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_b7_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            28 => {
                // Factor = Factor, "/", Pow => ActionFn(16);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            29 => {
                // Factor = Pow => ActionFn(17);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            30 => {
                // Name = r#"\\pL+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                14
            }
            31 => {
                // Num = r#"[0-9]+"# => ActionFn(30);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                15
            }
            32 => {
                // NumFloat = r#"[0-9]+\\.[0-9]+"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFloat(__nt), __end));
                16
            }
            33 => {
                // Op = r#"d/d(\\pL+)"# => ActionFn(18);
                let __sym0 = __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                17
            }
            34 => {
                // Pow = Term, "^", Pow => ActionFn(19);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            35 => {
                // Pow = Term, Pow => ActionFn(20);
                let __sym1 = __pop_NtPow(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            36 => {
                // Pow = Term => ActionFn(21);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            37 => {
                // Term = Num => ActionFn(25);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            38 => {
                // Term = NumFloat => ActionFn(26);
                let __sym0 = __pop_NtNumFloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            39 => {
                // Term = Var => ActionFn(27);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            40 => {
                // Term = Tuple => ActionFn(28);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            41 => {
                // Term = Array => ActionFn(29);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            42 => {
                // Tuple = "(", CommaE, ")" => ActionFn(23);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTuple(__nt), __end));
                20
            }
            43 => {
                // Var = Name => ActionFn(32);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                21
            }
            44 => {
                // __Array = Array => ActionFn(6);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Array(__nt), __end));
                22
            }
            45 => {
                // __CommaE = CommaE => ActionFn(4);
                let __sym0 = __pop_NtCommaE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaE(__nt), __end));
                23
            }
            46 => {
                // __CommaS = CommaS => ActionFn(8);
                let __sym0 = __pop_NtCommaS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            47 => {
                // __Command = Command => ActionFn(9);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Command(__nt), __end));
                25
            }
            48 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                26
            }
            49 => {
                // __Factor = Factor => ActionFn(1);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Factor(__nt), __end));
                27
            }
            50 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                28
            }
            51 => {
                // __Pow = Pow => ActionFn(3);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Pow(__nt), __end));
                29
            }
            52 => {
                // __Term = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                30
            }
            53 => {
                // __Tuple = Tuple => ActionFn(5);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Tuple(__nt), __end));
                31
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bench_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bench_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22def_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22def_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22eval_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22eval_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_b7_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_b7_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cpL_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cpL_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArray<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArray(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cName_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cName_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumFloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Array<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Array(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Command<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Command(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Factor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Factor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Pow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Pow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Tuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Tuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__CommaS::parse_CommaS;

mod __parse__Command {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22bench_22(&'input str),
        Term_22def_22(&'input str),
        Term_22eval_22(&'input str),
        Term_22_b7_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cpL_2b_22_23(&'input str),
        Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(NodeResult),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<NodeResult>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<NodeResult>),
        Nt_28_3cName_3e_20_22_2c_22_29(&'input str),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'input str>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'input str>),
        NtArray(NodeResult),
        NtComma_3cExpr_3e(Vec<NodeResult>),
        NtComma_3cName_3e(Vec<&'input str>),
        NtCommaE(Vec<NodeResult>),
        NtCommaS(Vec<&'input str>),
        NtCommand(Result<Command<'input>, Error>),
        NtExpr(NodeResult),
        NtFactor(NodeResult),
        NtName(&'input str),
        NtNum(NodeResult),
        NtNumFloat(NodeResult),
        NtOp(Func),
        NtPow(NodeResult),
        NtTerm(NodeResult),
        NtTuple(NodeResult),
        NtVar(NodeResult),
        Nt____Array(NodeResult),
        Nt____CommaE(Vec<NodeResult>),
        Nt____CommaS(Vec<&'input str>),
        Nt____Command(Result<Command<'input>, Error>),
        Nt____Expr(NodeResult),
        Nt____Factor(NodeResult),
        Nt____Op(Func),
        Nt____Pow(NodeResult),
        Nt____Term(NodeResult),
        Nt____Tuple(NodeResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        13, 0, 0, 0, 0, 14, 0, 0, 15, 0, 0, 16, 17, 18, 0, 19, 20, 21, 0,
        // State 1
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 2
        -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47,
        // State 3
        0, 0, 0, 22, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, -25, 24, -25, -25, -25, 25, 0, 0, -25, 0, 0, 0, 0, 26, 0, 0, 0, 0,
        // State 5
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 6
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 7
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 8
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 9
        13, -36, -36, -36, -36, -36, -36, 0, 15, -36, 28, 0, 0, 0, -36, 19, 20, 21, 0,
        // State 10
        -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 11
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 12
        13, 0, 0, 0, 0, 14, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 13
        13, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 14
        13, 0, 0, 0, 0, 14, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 15
        13, 0, 0, 0, 0, 14, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0,
        // State 17
        13, 0, 0, 0, 0, 14, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 18
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 19
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 20
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 21
        13, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 22
        13, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 23
        13, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 24
        13, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 25
        13, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 26
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 27
        13, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 28
        13, 0, 0, 0, 0, 14, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 29
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 30
        0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -12, 0, 22, 46, 23, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, -24, 24, -24, -24, -24, 25, 0, 0, -24, 0, 0, 0, 0, 26, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 22, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 22, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, -22, 24, -22, -22, -22, 25, 0, 0, -22, 0, 0, 0, 0, 26, 0, 0, 0, 0,
        // State 38
        0, -23, 24, -23, -23, -23, 25, 0, 0, -23, 0, 0, 0, 0, 26, 0, 0, 0, 0,
        // State 39
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 40
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 41
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 42
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 43
        0, -13, 0, 22, 49, 23, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 45
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 46
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0,
        // State 48
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0,
        // State 50
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 51
        0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -14, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -15, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 56
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 57
        13, 0, 0, 0, 0, 14, 0, 0, 15, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 58
        0, 0, 0, 22, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -41,
        -47,
        -21,
        -25,
        -43,
        -37,
        -38,
        -29,
        -36,
        -40,
        -39,
        0,
        0,
        0,
        0,
        0,
        0,
        -31,
        -32,
        -30,
        0,
        0,
        0,
        0,
        0,
        -35,
        0,
        0,
        -16,
        0,
        0,
        -24,
        0,
        -20,
        0,
        -19,
        -22,
        -23,
        -26,
        -28,
        -27,
        -34,
        0,
        -42,
        -4,
        -11,
        0,
        -5,
        0,
        -17,
        0,
        0,
        0,
        0,
        -9,
        -10,
        0,
        -18,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 4, 5, 6, 7, 8, 0, 9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 6, 7, 8, 0, 27, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 29, 0, 0, 0, 2, 30, 0, 31, 0, 0, 32, 5, 6, 7, 8, 0, 9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 33, 6, 7, 8, 0, 9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 29, 0, 0, 0, 2, 30, 0, 34, 0, 0, 32, 5, 6, 7, 8, 0, 9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 35, 5, 6, 7, 8, 0, 9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 37, 5, 6, 7, 8, 0, 9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 38, 6, 7, 8, 0, 9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 39, 6, 7, 8, 0, 9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 6, 7, 8, 0, 40, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 6, 7, 8, 0, 41, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 6, 7, 8, 0, 42, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 6, 7, 8, 0, 43, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 44, 5, 6, 7, 8, 0, 9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 50, 0, 0, 51, 0, 52, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 59, 5, 6, 7, 8, 0, 9, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bench""###,
            r###""def""###,
            r###""eval""###,
            r###""·""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+\\.[0-9]+"#"###,
            r###"r#"\\pL+"#"###,
            r###"r#"d/d(\\pL+)"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Command<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
    ) -> Result<Result<Command<'input>, Error>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
                (2, _) if true => 17,
                (3, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3a_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22bench_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22def_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22eval_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_b7_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5cpL_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Result<Command<'input>, Error>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(51);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Name> ",") = Name, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Name> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")+ = Name, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Array = "[", CommaE, "]" => ActionFn(24);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArray(__nt), __end));
                6
            }
            12 => {
                // Comma<Expr> = Expr => ActionFn(53);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            13 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(54);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            14 => {
                // Comma<Name> = Name => ActionFn(57);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            15 => {
                // Comma<Name> = (<Name> ",")+, Name => ActionFn(58);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            16 => {
                // CommaE = Comma<Expr> => ActionFn(22);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaE(__nt), __end));
                9
            }
            17 => {
                // CommaS = Comma<Name> => ActionFn(34);
                let __sym0 = __pop_NtComma_3cName_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaS(__nt), __end));
                10
            }
            18 => {
                // Command = "def", r#"\\pL+"#, "(", CommaS, ")", ":=", Expr => ActionFn(35);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22_3a_3d_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtCommaS(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __sym0 = __pop_Term_22def_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action35::<>(builder, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            19 => {
                // Command = "eval", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22eval_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            20 => {
                // Command = "bench", Expr => ActionFn(37);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22bench_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            21 => {
                // Command = Expr => ActionFn(38);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            22 => {
                // Expr = Expr, "+", Factor => ActionFn(10);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            23 => {
                // Expr = Expr, "-", Factor => ActionFn(11);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            24 => {
                // Expr = "-", Factor => ActionFn(12);
                let __sym1 = __pop_NtFactor(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            25 => {
                // Expr = Factor => ActionFn(13);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            26 => {
                // Factor = Factor, "*", Pow => ActionFn(14);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            27 => {
                // Factor = Factor, "·", Pow => ActionFn(15);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_b7_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            28 => {
                // Factor = Factor, "/", Pow => ActionFn(16);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            29 => {
                // Factor = Pow => ActionFn(17);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            30 => {
                // Name = r#"\\pL+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                14
            }
            31 => {
                // Num = r#"[0-9]+"# => ActionFn(30);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                15
            }
            32 => {
                // NumFloat = r#"[0-9]+\\.[0-9]+"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFloat(__nt), __end));
                16
            }
            33 => {
                // Op = r#"d/d(\\pL+)"# => ActionFn(18);
                let __sym0 = __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                17
            }
            34 => {
                // Pow = Term, "^", Pow => ActionFn(19);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            35 => {
                // Pow = Term, Pow => ActionFn(20);
                let __sym1 = __pop_NtPow(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            36 => {
                // Pow = Term => ActionFn(21);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            37 => {
                // Term = Num => ActionFn(25);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            38 => {
                // Term = NumFloat => ActionFn(26);
                let __sym0 = __pop_NtNumFloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            39 => {
                // Term = Var => ActionFn(27);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            40 => {
                // Term = Tuple => ActionFn(28);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            41 => {
                // Term = Array => ActionFn(29);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            42 => {
                // Tuple = "(", CommaE, ")" => ActionFn(23);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTuple(__nt), __end));
                20
            }
            43 => {
                // Var = Name => ActionFn(32);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                21
            }
            44 => {
                // __Array = Array => ActionFn(6);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Array(__nt), __end));
                22
            }
            45 => {
                // __CommaE = CommaE => ActionFn(4);
                let __sym0 = __pop_NtCommaE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaE(__nt), __end));
                23
            }
            46 => {
                // __CommaS = CommaS => ActionFn(8);
                let __sym0 = __pop_NtCommaS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaS(__nt), __end));
                24
            }
            47 => {
                // __Command = Command => ActionFn(9);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            48 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                26
            }
            49 => {
                // __Factor = Factor => ActionFn(1);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Factor(__nt), __end));
                27
            }
            50 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                28
            }
            51 => {
                // __Pow = Pow => ActionFn(3);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Pow(__nt), __end));
                29
            }
            52 => {
                // __Term = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                30
            }
            53 => {
                // __Tuple = Tuple => ActionFn(5);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Tuple(__nt), __end));
                31
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bench_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bench_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22def_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22def_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22eval_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22eval_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_b7_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_b7_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cpL_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cpL_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArray<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArray(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cName_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cName_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumFloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Array<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Array(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Command<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Command(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Factor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Factor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Pow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Pow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Tuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Tuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Command::parse_Command;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22bench_22(&'input str),
        Term_22def_22(&'input str),
        Term_22eval_22(&'input str),
        Term_22_b7_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cpL_2b_22_23(&'input str),
        Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(NodeResult),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<NodeResult>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<NodeResult>),
        Nt_28_3cName_3e_20_22_2c_22_29(&'input str),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'input str>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'input str>),
        NtArray(NodeResult),
        NtComma_3cExpr_3e(Vec<NodeResult>),
        NtComma_3cName_3e(Vec<&'input str>),
        NtCommaE(Vec<NodeResult>),
        NtCommaS(Vec<&'input str>),
        NtCommand(Result<Command<'input>, Error>),
        NtExpr(NodeResult),
        NtFactor(NodeResult),
        NtName(&'input str),
        NtNum(NodeResult),
        NtNumFloat(NodeResult),
        NtOp(Func),
        NtPow(NodeResult),
        NtTerm(NodeResult),
        NtTuple(NodeResult),
        NtVar(NodeResult),
        Nt____Array(NodeResult),
        Nt____CommaE(Vec<NodeResult>),
        Nt____CommaS(Vec<&'input str>),
        Nt____Command(Result<Command<'input>, Error>),
        Nt____Expr(NodeResult),
        Nt____Factor(NodeResult),
        Nt____Op(Func),
        Nt____Pow(NodeResult),
        Nt____Term(NodeResult),
        Nt____Tuple(NodeResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        12, 0, 0, 0, 0, 13, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 1
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 2
        0, 0, 0, 18, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, -25, 20, -25, -25, -25, 21, 0, 0, -25, 0, 0, 0, 0, 22, 0, 0, 0, 0,
        // State 4
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 5
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 6
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 7
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 8
        12, -36, -36, -36, -36, -36, -36, 0, 14, -36, 24, 0, 0, 0, -36, 15, 16, 17, 0,
        // State 9
        -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 10
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 11
        12, 0, 0, 0, 0, 13, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 12
        12, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 13
        12, 0, 0, 0, 0, 13, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 14
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 15
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 16
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 17
        12, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 18
        12, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 19
        12, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 20
        12, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 21
        12, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 22
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 23
        12, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 24
        12, 0, 0, 0, 0, 13, 0, 0, 14, 0, 0, 0, 0, 0, 0, 15, 16, 17, 0,
        // State 25
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 26
        0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, -12, 0, 18, 39, 19, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -24, 20, -24, -24, -24, 21, 0, 0, -24, 0, 0, 0, 0, 22, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -22, 20, -22, -22, -22, 21, 0, 0, -22, 0, 0, 0, 0, 22, 0, 0, 0, 0,
        // State 31
        0, -23, 20, -23, -23, -23, 21, 0, 0, -23, 0, 0, 0, 0, 22, 0, 0, 0, 0,
        // State 32
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 33
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 34
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 35
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 36
        0, -13, 0, 18, 41, 19, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 38
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 39
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 40
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -41,
        -48,
        -25,
        -43,
        -37,
        -38,
        -29,
        -36,
        -40,
        -39,
        0,
        0,
        0,
        -31,
        -32,
        -30,
        0,
        0,
        0,
        0,
        0,
        -35,
        0,
        0,
        -16,
        0,
        0,
        -24,
        0,
        -22,
        -23,
        -26,
        -28,
        -27,
        -34,
        0,
        -42,
        -4,
        -11,
        -5,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 4, 5, 6, 7, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 0, 23, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 25, 0, 0, 0, 2, 26, 0, 27, 0, 0, 28, 4, 5, 6, 7, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 29, 5, 6, 7, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 25, 0, 0, 0, 2, 26, 0, 30, 0, 0, 28, 4, 5, 6, 7, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 31, 5, 6, 7, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 32, 5, 6, 7, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 0, 33, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 0, 34, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 0, 35, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 0, 36, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 37, 4, 5, 6, 7, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bench""###,
            r###""def""###,
            r###""eval""###,
            r###""·""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+\\.[0-9]+"#"###,
            r###"r#"\\pL+"#"###,
            r###"r#"d/d(\\pL+)"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Expr<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
    ) -> Result<NodeResult, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
                (2, _) if true => 17,
                (3, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3a_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22bench_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22def_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22eval_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_b7_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5cpL_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<NodeResult,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(51);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Name> ",") = Name, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Name> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")+ = Name, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Array = "[", CommaE, "]" => ActionFn(24);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArray(__nt), __end));
                6
            }
            12 => {
                // Comma<Expr> = Expr => ActionFn(53);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            13 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(54);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            14 => {
                // Comma<Name> = Name => ActionFn(57);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            15 => {
                // Comma<Name> = (<Name> ",")+, Name => ActionFn(58);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            16 => {
                // CommaE = Comma<Expr> => ActionFn(22);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaE(__nt), __end));
                9
            }
            17 => {
                // CommaS = Comma<Name> => ActionFn(34);
                let __sym0 = __pop_NtComma_3cName_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaS(__nt), __end));
                10
            }
            18 => {
                // Command = "def", r#"\\pL+"#, "(", CommaS, ")", ":=", Expr => ActionFn(35);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22_3a_3d_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtCommaS(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __sym0 = __pop_Term_22def_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action35::<>(builder, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            19 => {
                // Command = "eval", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22eval_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            20 => {
                // Command = "bench", Expr => ActionFn(37);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22bench_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            21 => {
                // Command = Expr => ActionFn(38);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            22 => {
                // Expr = Expr, "+", Factor => ActionFn(10);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            23 => {
                // Expr = Expr, "-", Factor => ActionFn(11);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            24 => {
                // Expr = "-", Factor => ActionFn(12);
                let __sym1 = __pop_NtFactor(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            25 => {
                // Expr = Factor => ActionFn(13);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            26 => {
                // Factor = Factor, "*", Pow => ActionFn(14);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            27 => {
                // Factor = Factor, "·", Pow => ActionFn(15);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_b7_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            28 => {
                // Factor = Factor, "/", Pow => ActionFn(16);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            29 => {
                // Factor = Pow => ActionFn(17);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            30 => {
                // Name = r#"\\pL+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                14
            }
            31 => {
                // Num = r#"[0-9]+"# => ActionFn(30);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                15
            }
            32 => {
                // NumFloat = r#"[0-9]+\\.[0-9]+"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFloat(__nt), __end));
                16
            }
            33 => {
                // Op = r#"d/d(\\pL+)"# => ActionFn(18);
                let __sym0 = __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                17
            }
            34 => {
                // Pow = Term, "^", Pow => ActionFn(19);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            35 => {
                // Pow = Term, Pow => ActionFn(20);
                let __sym1 = __pop_NtPow(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            36 => {
                // Pow = Term => ActionFn(21);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            37 => {
                // Term = Num => ActionFn(25);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            38 => {
                // Term = NumFloat => ActionFn(26);
                let __sym0 = __pop_NtNumFloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            39 => {
                // Term = Var => ActionFn(27);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            40 => {
                // Term = Tuple => ActionFn(28);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            41 => {
                // Term = Array => ActionFn(29);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            42 => {
                // Tuple = "(", CommaE, ")" => ActionFn(23);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTuple(__nt), __end));
                20
            }
            43 => {
                // Var = Name => ActionFn(32);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                21
            }
            44 => {
                // __Array = Array => ActionFn(6);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Array(__nt), __end));
                22
            }
            45 => {
                // __CommaE = CommaE => ActionFn(4);
                let __sym0 = __pop_NtCommaE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaE(__nt), __end));
                23
            }
            46 => {
                // __CommaS = CommaS => ActionFn(8);
                let __sym0 = __pop_NtCommaS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaS(__nt), __end));
                24
            }
            47 => {
                // __Command = Command => ActionFn(9);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Command(__nt), __end));
                25
            }
            48 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            49 => {
                // __Factor = Factor => ActionFn(1);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Factor(__nt), __end));
                27
            }
            50 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                28
            }
            51 => {
                // __Pow = Pow => ActionFn(3);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Pow(__nt), __end));
                29
            }
            52 => {
                // __Term = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                30
            }
            53 => {
                // __Tuple = Tuple => ActionFn(5);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Tuple(__nt), __end));
                31
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bench_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bench_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22def_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22def_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22eval_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22eval_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_b7_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_b7_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cpL_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cpL_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArray<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArray(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cName_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cName_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumFloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Array<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Array(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Command<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Command(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Factor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Factor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Pow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Pow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Tuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Tuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expr::parse_Expr;

mod __parse__Factor {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22bench_22(&'input str),
        Term_22def_22(&'input str),
        Term_22eval_22(&'input str),
        Term_22_b7_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cpL_2b_22_23(&'input str),
        Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(NodeResult),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<NodeResult>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<NodeResult>),
        Nt_28_3cName_3e_20_22_2c_22_29(&'input str),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'input str>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'input str>),
        NtArray(NodeResult),
        NtComma_3cExpr_3e(Vec<NodeResult>),
        NtComma_3cName_3e(Vec<&'input str>),
        NtCommaE(Vec<NodeResult>),
        NtCommaS(Vec<&'input str>),
        NtCommand(Result<Command<'input>, Error>),
        NtExpr(NodeResult),
        NtFactor(NodeResult),
        NtName(&'input str),
        NtNum(NodeResult),
        NtNumFloat(NodeResult),
        NtOp(Func),
        NtPow(NodeResult),
        NtTerm(NodeResult),
        NtTuple(NodeResult),
        NtVar(NodeResult),
        Nt____Array(NodeResult),
        Nt____CommaE(Vec<NodeResult>),
        Nt____CommaS(Vec<&'input str>),
        Nt____Command(Result<Command<'input>, Error>),
        Nt____Expr(NodeResult),
        Nt____Factor(NodeResult),
        Nt____Op(Func),
        Nt____Pow(NodeResult),
        Nt____Term(NodeResult),
        Nt____Tuple(NodeResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        11, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 1
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 2
        0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0,
        // State 3
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 4
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 5
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 6
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 7
        11, -36, -36, -36, -36, -36, -36, 0, 12, -36, 20, 0, 0, 0, -36, 13, 14, 15, 0,
        // State 8
        -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 9
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 10
        11, 0, 0, 0, 0, 26, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 11
        11, 0, 0, 0, 0, 26, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 12
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 13
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 14
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 15
        11, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 16
        11, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 17
        11, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 18
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 19
        11, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 20
        11, 0, 0, 0, 0, 26, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 21
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 22
        0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, -12, 0, 34, 35, 36, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, -25, 16, -25, -25, -25, 17, 0, 0, -25, 0, 0, 0, 0, 18, 0, 0, 0, 0,
        // State 25
        11, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 28
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 29
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 30
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 31
        0, -13, 0, 34, 39, 36, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 33
        11, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 34
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 35
        11, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 14, 15, 0,
        // State 36
        0, -24, 16, -24, -24, -24, 17, 0, 0, -24, 0, 0, 0, 0, 18, 0, 0, 0, 0,
        // State 37
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 38
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 39
        0, -22, 16, -22, -22, -22, 17, 0, 0, -22, 0, 0, 0, 0, 18, 0, 0, 0, 0,
        // State 40
        0, -23, 16, -23, -23, -23, 17, 0, 0, -23, 0, 0, 0, 0, 18, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -41,
        -49,
        -43,
        -37,
        -38,
        -29,
        -36,
        -40,
        -39,
        0,
        0,
        -31,
        -32,
        -30,
        0,
        0,
        0,
        -35,
        0,
        0,
        -16,
        0,
        0,
        0,
        0,
        0,
        -26,
        -28,
        -27,
        -34,
        0,
        -42,
        0,
        -4,
        0,
        0,
        -11,
        -5,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 3, 4, 5, 6, 0, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 5, 6, 0, 19, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 21, 0, 0, 0, 2, 22, 0, 23, 0, 0, 24, 25, 4, 5, 6, 0, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 21, 0, 0, 0, 2, 22, 0, 27, 0, 0, 24, 25, 4, 5, 6, 0, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 5, 6, 0, 28, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 5, 6, 0, 29, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 5, 6, 0, 30, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 5, 6, 0, 31, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 32, 25, 4, 5, 6, 0, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 37, 4, 5, 6, 0, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 40, 4, 5, 6, 0, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 41, 4, 5, 6, 0, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bench""###,
            r###""def""###,
            r###""eval""###,
            r###""·""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+\\.[0-9]+"#"###,
            r###"r#"\\pL+"#"###,
            r###"r#"d/d(\\pL+)"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Factor<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
    ) -> Result<NodeResult, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
                (2, _) if true => 17,
                (3, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3a_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22bench_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22def_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22eval_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_b7_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5cpL_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<NodeResult,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(51);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Name> ",") = Name, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Name> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")+ = Name, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Array = "[", CommaE, "]" => ActionFn(24);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArray(__nt), __end));
                6
            }
            12 => {
                // Comma<Expr> = Expr => ActionFn(53);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            13 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(54);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            14 => {
                // Comma<Name> = Name => ActionFn(57);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            15 => {
                // Comma<Name> = (<Name> ",")+, Name => ActionFn(58);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            16 => {
                // CommaE = Comma<Expr> => ActionFn(22);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaE(__nt), __end));
                9
            }
            17 => {
                // CommaS = Comma<Name> => ActionFn(34);
                let __sym0 = __pop_NtComma_3cName_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaS(__nt), __end));
                10
            }
            18 => {
                // Command = "def", r#"\\pL+"#, "(", CommaS, ")", ":=", Expr => ActionFn(35);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22_3a_3d_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtCommaS(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __sym0 = __pop_Term_22def_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action35::<>(builder, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            19 => {
                // Command = "eval", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22eval_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            20 => {
                // Command = "bench", Expr => ActionFn(37);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22bench_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            21 => {
                // Command = Expr => ActionFn(38);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            22 => {
                // Expr = Expr, "+", Factor => ActionFn(10);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            23 => {
                // Expr = Expr, "-", Factor => ActionFn(11);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            24 => {
                // Expr = "-", Factor => ActionFn(12);
                let __sym1 = __pop_NtFactor(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            25 => {
                // Expr = Factor => ActionFn(13);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            26 => {
                // Factor = Factor, "*", Pow => ActionFn(14);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            27 => {
                // Factor = Factor, "·", Pow => ActionFn(15);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_b7_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            28 => {
                // Factor = Factor, "/", Pow => ActionFn(16);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            29 => {
                // Factor = Pow => ActionFn(17);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            30 => {
                // Name = r#"\\pL+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                14
            }
            31 => {
                // Num = r#"[0-9]+"# => ActionFn(30);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                15
            }
            32 => {
                // NumFloat = r#"[0-9]+\\.[0-9]+"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFloat(__nt), __end));
                16
            }
            33 => {
                // Op = r#"d/d(\\pL+)"# => ActionFn(18);
                let __sym0 = __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                17
            }
            34 => {
                // Pow = Term, "^", Pow => ActionFn(19);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            35 => {
                // Pow = Term, Pow => ActionFn(20);
                let __sym1 = __pop_NtPow(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            36 => {
                // Pow = Term => ActionFn(21);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            37 => {
                // Term = Num => ActionFn(25);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            38 => {
                // Term = NumFloat => ActionFn(26);
                let __sym0 = __pop_NtNumFloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            39 => {
                // Term = Var => ActionFn(27);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            40 => {
                // Term = Tuple => ActionFn(28);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            41 => {
                // Term = Array => ActionFn(29);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            42 => {
                // Tuple = "(", CommaE, ")" => ActionFn(23);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTuple(__nt), __end));
                20
            }
            43 => {
                // Var = Name => ActionFn(32);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                21
            }
            44 => {
                // __Array = Array => ActionFn(6);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Array(__nt), __end));
                22
            }
            45 => {
                // __CommaE = CommaE => ActionFn(4);
                let __sym0 = __pop_NtCommaE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaE(__nt), __end));
                23
            }
            46 => {
                // __CommaS = CommaS => ActionFn(8);
                let __sym0 = __pop_NtCommaS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaS(__nt), __end));
                24
            }
            47 => {
                // __Command = Command => ActionFn(9);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Command(__nt), __end));
                25
            }
            48 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                26
            }
            49 => {
                // __Factor = Factor => ActionFn(1);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            50 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                28
            }
            51 => {
                // __Pow = Pow => ActionFn(3);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Pow(__nt), __end));
                29
            }
            52 => {
                // __Term = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                30
            }
            53 => {
                // __Tuple = Tuple => ActionFn(5);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Tuple(__nt), __end));
                31
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bench_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bench_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22def_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22def_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22eval_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22eval_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_b7_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_b7_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cpL_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cpL_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArray<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArray(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cName_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cName_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumFloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Array<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Array(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Command<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Command(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Factor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Factor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Pow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Pow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Tuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Tuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Factor::parse_Factor;

mod __parse__Op {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22bench_22(&'input str),
        Term_22def_22(&'input str),
        Term_22eval_22(&'input str),
        Term_22_b7_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cpL_2b_22_23(&'input str),
        Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(NodeResult),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<NodeResult>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<NodeResult>),
        Nt_28_3cName_3e_20_22_2c_22_29(&'input str),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'input str>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'input str>),
        NtArray(NodeResult),
        NtComma_3cExpr_3e(Vec<NodeResult>),
        NtComma_3cName_3e(Vec<&'input str>),
        NtCommaE(Vec<NodeResult>),
        NtCommaS(Vec<&'input str>),
        NtCommand(Result<Command<'input>, Error>),
        NtExpr(NodeResult),
        NtFactor(NodeResult),
        NtName(&'input str),
        NtNum(NodeResult),
        NtNumFloat(NodeResult),
        NtOp(Func),
        NtPow(NodeResult),
        NtTerm(NodeResult),
        NtTuple(NodeResult),
        NtVar(NodeResult),
        Nt____Array(NodeResult),
        Nt____CommaE(Vec<NodeResult>),
        Nt____CommaS(Vec<&'input str>),
        Nt____Command(Result<Command<'input>, Error>),
        Nt____Expr(NodeResult),
        Nt____Factor(NodeResult),
        Nt____Op(Func),
        Nt____Pow(NodeResult),
        Nt____Term(NodeResult),
        Nt____Tuple(NodeResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
        // State 1
        -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50, -50,
        // State 2
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -50,
        -33,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bench""###,
            r###""def""###,
            r###""eval""###,
            r###""·""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+\\.[0-9]+"#"###,
            r###"r#"\\pL+"#"###,
            r###"r#"d/d(\\pL+)"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Op<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
    ) -> Result<Func, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
                (2, _) if true => 17,
                (3, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3a_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22bench_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22def_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22eval_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_b7_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5cpL_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Func,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(51);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Name> ",") = Name, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Name> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")+ = Name, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Array = "[", CommaE, "]" => ActionFn(24);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArray(__nt), __end));
                6
            }
            12 => {
                // Comma<Expr> = Expr => ActionFn(53);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            13 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(54);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            14 => {
                // Comma<Name> = Name => ActionFn(57);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            15 => {
                // Comma<Name> = (<Name> ",")+, Name => ActionFn(58);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            16 => {
                // CommaE = Comma<Expr> => ActionFn(22);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaE(__nt), __end));
                9
            }
            17 => {
                // CommaS = Comma<Name> => ActionFn(34);
                let __sym0 = __pop_NtComma_3cName_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaS(__nt), __end));
                10
            }
            18 => {
                // Command = "def", r#"\\pL+"#, "(", CommaS, ")", ":=", Expr => ActionFn(35);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22_3a_3d_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtCommaS(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __sym0 = __pop_Term_22def_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action35::<>(builder, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            19 => {
                // Command = "eval", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22eval_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            20 => {
                // Command = "bench", Expr => ActionFn(37);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22bench_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            21 => {
                // Command = Expr => ActionFn(38);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            22 => {
                // Expr = Expr, "+", Factor => ActionFn(10);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            23 => {
                // Expr = Expr, "-", Factor => ActionFn(11);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            24 => {
                // Expr = "-", Factor => ActionFn(12);
                let __sym1 = __pop_NtFactor(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            25 => {
                // Expr = Factor => ActionFn(13);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            26 => {
                // Factor = Factor, "*", Pow => ActionFn(14);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            27 => {
                // Factor = Factor, "·", Pow => ActionFn(15);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_b7_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            28 => {
                // Factor = Factor, "/", Pow => ActionFn(16);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            29 => {
                // Factor = Pow => ActionFn(17);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            30 => {
                // Name = r#"\\pL+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                14
            }
            31 => {
                // Num = r#"[0-9]+"# => ActionFn(30);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                15
            }
            32 => {
                // NumFloat = r#"[0-9]+\\.[0-9]+"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFloat(__nt), __end));
                16
            }
            33 => {
                // Op = r#"d/d(\\pL+)"# => ActionFn(18);
                let __sym0 = __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                17
            }
            34 => {
                // Pow = Term, "^", Pow => ActionFn(19);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            35 => {
                // Pow = Term, Pow => ActionFn(20);
                let __sym1 = __pop_NtPow(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            36 => {
                // Pow = Term => ActionFn(21);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            37 => {
                // Term = Num => ActionFn(25);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            38 => {
                // Term = NumFloat => ActionFn(26);
                let __sym0 = __pop_NtNumFloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            39 => {
                // Term = Var => ActionFn(27);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            40 => {
                // Term = Tuple => ActionFn(28);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            41 => {
                // Term = Array => ActionFn(29);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            42 => {
                // Tuple = "(", CommaE, ")" => ActionFn(23);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTuple(__nt), __end));
                20
            }
            43 => {
                // Var = Name => ActionFn(32);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                21
            }
            44 => {
                // __Array = Array => ActionFn(6);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Array(__nt), __end));
                22
            }
            45 => {
                // __CommaE = CommaE => ActionFn(4);
                let __sym0 = __pop_NtCommaE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaE(__nt), __end));
                23
            }
            46 => {
                // __CommaS = CommaS => ActionFn(8);
                let __sym0 = __pop_NtCommaS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaS(__nt), __end));
                24
            }
            47 => {
                // __Command = Command => ActionFn(9);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Command(__nt), __end));
                25
            }
            48 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                26
            }
            49 => {
                // __Factor = Factor => ActionFn(1);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Factor(__nt), __end));
                27
            }
            50 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            51 => {
                // __Pow = Pow => ActionFn(3);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Pow(__nt), __end));
                29
            }
            52 => {
                // __Term = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                30
            }
            53 => {
                // __Tuple = Tuple => ActionFn(5);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Tuple(__nt), __end));
                31
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bench_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bench_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22def_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22def_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22eval_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22eval_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_b7_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_b7_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cpL_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cpL_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArray<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArray(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cName_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cName_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumFloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Array<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Array(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Command<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Command(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Factor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Factor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Pow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Pow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Tuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Tuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Op::parse_Op;

mod __parse__Pow {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22bench_22(&'input str),
        Term_22def_22(&'input str),
        Term_22eval_22(&'input str),
        Term_22_b7_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cpL_2b_22_23(&'input str),
        Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(NodeResult),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<NodeResult>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<NodeResult>),
        Nt_28_3cName_3e_20_22_2c_22_29(&'input str),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'input str>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'input str>),
        NtArray(NodeResult),
        NtComma_3cExpr_3e(Vec<NodeResult>),
        NtComma_3cName_3e(Vec<&'input str>),
        NtCommaE(Vec<NodeResult>),
        NtCommaS(Vec<&'input str>),
        NtCommand(Result<Command<'input>, Error>),
        NtExpr(NodeResult),
        NtFactor(NodeResult),
        NtName(&'input str),
        NtNum(NodeResult),
        NtNumFloat(NodeResult),
        NtOp(Func),
        NtPow(NodeResult),
        NtTerm(NodeResult),
        NtTuple(NodeResult),
        NtVar(NodeResult),
        Nt____Array(NodeResult),
        Nt____CommaE(Vec<NodeResult>),
        Nt____CommaS(Vec<&'input str>),
        Nt____Command(Result<Command<'input>, Error>),
        Nt____Expr(NodeResult),
        Nt____Factor(NodeResult),
        Nt____Op(Func),
        Nt____Pow(NodeResult),
        Nt____Term(NodeResult),
        Nt____Tuple(NodeResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 1
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 2
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 3
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 4
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 5
        -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51, -51,
        // State 6
        10, -36, -36, -36, -36, -36, -36, 0, 11, -36, 16, 0, 0, 0, -36, 12, 13, 14, 0,
        // State 7
        -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 8
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 9
        10, 0, 0, 0, 0, 23, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 10
        10, 0, 0, 0, 0, 23, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 11
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 12
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 13
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 14
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 15
        10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 16
        10, 0, 0, 0, 0, 23, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 17
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 18
        0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, -12, 0, 28, 29, 30, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, -25, 31, -25, -25, -25, 32, 0, 0, -25, 0, 0, 0, 0, 33, 0, 0, 0, 0,
        // State 21
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 22
        10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 25
        0, -13, 0, 28, 36, 30, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 27
        10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 28
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 29
        10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 30
        10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 31
        10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 32
        10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0,
        // State 33
        0, -24, 31, -24, -24, -24, 32, 0, 0, -24, 0, 0, 0, 0, 33, 0, 0, 0, 0,
        // State 34
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 35
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 36
        0, -22, 31, -22, -22, -22, 32, 0, 0, -22, 0, 0, 0, 0, 33, 0, 0, 0, 0,
        // State 37
        0, -23, 31, -23, -23, -23, 32, 0, 0, -23, 0, 0, 0, 0, 33, 0, 0, 0, 0,
        // State 38
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 39
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 40
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -41,
        -43,
        -37,
        -38,
        -51,
        -36,
        -40,
        -39,
        0,
        0,
        -31,
        -32,
        -30,
        -35,
        0,
        0,
        -16,
        0,
        0,
        0,
        -29,
        0,
        0,
        -34,
        0,
        -42,
        0,
        -4,
        0,
        0,
        0,
        0,
        0,
        -11,
        -5,
        0,
        0,
        -26,
        -28,
        -27,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 15, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 17, 0, 0, 0, 2, 18, 0, 19, 0, 0, 20, 21, 3, 4, 5, 0, 22, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 17, 0, 0, 0, 2, 18, 0, 24, 0, 0, 20, 21, 3, 4, 5, 0, 22, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 25, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 26, 21, 3, 4, 5, 0, 22, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 34, 3, 4, 5, 0, 22, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 37, 3, 4, 5, 0, 22, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 38, 3, 4, 5, 0, 22, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 39, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 40, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 41, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bench""###,
            r###""def""###,
            r###""eval""###,
            r###""·""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+\\.[0-9]+"#"###,
            r###"r#"\\pL+"#"###,
            r###"r#"d/d(\\pL+)"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Pow<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
    ) -> Result<NodeResult, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
                (2, _) if true => 17,
                (3, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3a_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22bench_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22def_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22eval_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_b7_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5cpL_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<NodeResult,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(51);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Name> ",") = Name, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Name> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")+ = Name, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Array = "[", CommaE, "]" => ActionFn(24);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArray(__nt), __end));
                6
            }
            12 => {
                // Comma<Expr> = Expr => ActionFn(53);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            13 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(54);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            14 => {
                // Comma<Name> = Name => ActionFn(57);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            15 => {
                // Comma<Name> = (<Name> ",")+, Name => ActionFn(58);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            16 => {
                // CommaE = Comma<Expr> => ActionFn(22);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaE(__nt), __end));
                9
            }
            17 => {
                // CommaS = Comma<Name> => ActionFn(34);
                let __sym0 = __pop_NtComma_3cName_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaS(__nt), __end));
                10
            }
            18 => {
                // Command = "def", r#"\\pL+"#, "(", CommaS, ")", ":=", Expr => ActionFn(35);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22_3a_3d_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtCommaS(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __sym0 = __pop_Term_22def_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action35::<>(builder, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            19 => {
                // Command = "eval", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22eval_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            20 => {
                // Command = "bench", Expr => ActionFn(37);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22bench_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            21 => {
                // Command = Expr => ActionFn(38);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            22 => {
                // Expr = Expr, "+", Factor => ActionFn(10);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            23 => {
                // Expr = Expr, "-", Factor => ActionFn(11);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            24 => {
                // Expr = "-", Factor => ActionFn(12);
                let __sym1 = __pop_NtFactor(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            25 => {
                // Expr = Factor => ActionFn(13);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            26 => {
                // Factor = Factor, "*", Pow => ActionFn(14);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            27 => {
                // Factor = Factor, "·", Pow => ActionFn(15);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_b7_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            28 => {
                // Factor = Factor, "/", Pow => ActionFn(16);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            29 => {
                // Factor = Pow => ActionFn(17);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            30 => {
                // Name = r#"\\pL+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                14
            }
            31 => {
                // Num = r#"[0-9]+"# => ActionFn(30);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                15
            }
            32 => {
                // NumFloat = r#"[0-9]+\\.[0-9]+"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFloat(__nt), __end));
                16
            }
            33 => {
                // Op = r#"d/d(\\pL+)"# => ActionFn(18);
                let __sym0 = __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                17
            }
            34 => {
                // Pow = Term, "^", Pow => ActionFn(19);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            35 => {
                // Pow = Term, Pow => ActionFn(20);
                let __sym1 = __pop_NtPow(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            36 => {
                // Pow = Term => ActionFn(21);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            37 => {
                // Term = Num => ActionFn(25);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            38 => {
                // Term = NumFloat => ActionFn(26);
                let __sym0 = __pop_NtNumFloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            39 => {
                // Term = Var => ActionFn(27);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            40 => {
                // Term = Tuple => ActionFn(28);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            41 => {
                // Term = Array => ActionFn(29);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            42 => {
                // Tuple = "(", CommaE, ")" => ActionFn(23);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTuple(__nt), __end));
                20
            }
            43 => {
                // Var = Name => ActionFn(32);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                21
            }
            44 => {
                // __Array = Array => ActionFn(6);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Array(__nt), __end));
                22
            }
            45 => {
                // __CommaE = CommaE => ActionFn(4);
                let __sym0 = __pop_NtCommaE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaE(__nt), __end));
                23
            }
            46 => {
                // __CommaS = CommaS => ActionFn(8);
                let __sym0 = __pop_NtCommaS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaS(__nt), __end));
                24
            }
            47 => {
                // __Command = Command => ActionFn(9);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Command(__nt), __end));
                25
            }
            48 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                26
            }
            49 => {
                // __Factor = Factor => ActionFn(1);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Factor(__nt), __end));
                27
            }
            50 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                28
            }
            51 => {
                // __Pow = Pow => ActionFn(3);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            52 => {
                // __Term = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                30
            }
            53 => {
                // __Tuple = Tuple => ActionFn(5);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Tuple(__nt), __end));
                31
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bench_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bench_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22def_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22def_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22eval_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22eval_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_b7_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_b7_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cpL_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cpL_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArray<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArray(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cName_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cName_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumFloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Array<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Array(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Command<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Command(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Factor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Factor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Pow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Pow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Tuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Tuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Pow::parse_Pow;

mod __parse__Term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22bench_22(&'input str),
        Term_22def_22(&'input str),
        Term_22eval_22(&'input str),
        Term_22_b7_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cpL_2b_22_23(&'input str),
        Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(NodeResult),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<NodeResult>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<NodeResult>),
        Nt_28_3cName_3e_20_22_2c_22_29(&'input str),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'input str>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'input str>),
        NtArray(NodeResult),
        NtComma_3cExpr_3e(Vec<NodeResult>),
        NtComma_3cName_3e(Vec<&'input str>),
        NtCommaE(Vec<NodeResult>),
        NtCommaS(Vec<&'input str>),
        NtCommand(Result<Command<'input>, Error>),
        NtExpr(NodeResult),
        NtFactor(NodeResult),
        NtName(&'input str),
        NtNum(NodeResult),
        NtNumFloat(NodeResult),
        NtOp(Func),
        NtPow(NodeResult),
        NtTerm(NodeResult),
        NtTuple(NodeResult),
        NtVar(NodeResult),
        Nt____Array(NodeResult),
        Nt____CommaE(Vec<NodeResult>),
        Nt____CommaS(Vec<&'input str>),
        Nt____Command(Result<Command<'input>, Error>),
        Nt____Expr(NodeResult),
        Nt____Factor(NodeResult),
        Nt____Op(Func),
        Nt____Pow(NodeResult),
        Nt____Term(NodeResult),
        Nt____Tuple(NodeResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        9, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 1
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 2
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 3
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 4
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 5
        -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52,
        // State 6
        -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 7
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 8
        9, 0, 0, 0, 0, 21, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 9
        9, 0, 0, 0, 0, 21, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 10
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 11
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 12
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 13
        9, 0, 0, 0, 0, 21, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 14
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 15
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, -12, 0, 25, 26, 27, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, -25, 28, -25, -25, -25, 29, 0, 0, -25, 0, 0, 0, 0, 30, 0, 0, 0, 0,
        // State 18
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 19
        9, -36, -36, -36, -36, -36, -36, 0, 10, -36, 32, 0, 0, 0, -36, 11, 12, 13, 0,
        // State 20
        9, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, -13, 0, 25, 35, 27, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 24
        9, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 25
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 26
        9, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 27
        9, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 28
        9, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 29
        9, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 30
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 31
        9, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0,
        // State 32
        0, -24, 28, -24, -24, -24, 29, 0, 0, -24, 0, 0, 0, 0, 30, 0, 0, 0, 0,
        // State 33
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 34
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 35
        0, -22, 28, -22, -22, -22, 29, 0, 0, -22, 0, 0, 0, 0, 30, 0, 0, 0, 0,
        // State 36
        0, -23, 28, -23, -23, -23, 29, 0, 0, -23, 0, 0, 0, 0, 30, 0, 0, 0, 0,
        // State 37
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 38
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 39
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 40
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -41,
        -43,
        -37,
        -38,
        -52,
        -40,
        -39,
        0,
        0,
        -31,
        -32,
        -30,
        0,
        -16,
        0,
        0,
        0,
        -29,
        0,
        0,
        0,
        0,
        -42,
        0,
        -4,
        0,
        0,
        0,
        0,
        -35,
        0,
        0,
        -11,
        -5,
        0,
        0,
        -26,
        -28,
        -27,
        -34,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 0, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 14, 0, 0, 0, 2, 15, 0, 16, 0, 0, 17, 18, 3, 4, 5, 0, 19, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 14, 0, 0, 0, 2, 15, 0, 22, 0, 0, 17, 18, 3, 4, 5, 0, 19, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 23, 18, 3, 4, 5, 0, 19, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 31, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 33, 3, 4, 5, 0, 19, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 36, 3, 4, 5, 0, 19, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 37, 3, 4, 5, 0, 19, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 38, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 39, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 40, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 41, 20, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bench""###,
            r###""def""###,
            r###""eval""###,
            r###""·""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+\\.[0-9]+"#"###,
            r###"r#"\\pL+"#"###,
            r###"r#"d/d(\\pL+)"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Term<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
    ) -> Result<NodeResult, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
                (2, _) if true => 17,
                (3, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3a_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22bench_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22def_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22eval_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_b7_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5cpL_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<NodeResult,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(51);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Name> ",") = Name, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Name> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")+ = Name, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Array = "[", CommaE, "]" => ActionFn(24);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArray(__nt), __end));
                6
            }
            12 => {
                // Comma<Expr> = Expr => ActionFn(53);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            13 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(54);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            14 => {
                // Comma<Name> = Name => ActionFn(57);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            15 => {
                // Comma<Name> = (<Name> ",")+, Name => ActionFn(58);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            16 => {
                // CommaE = Comma<Expr> => ActionFn(22);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaE(__nt), __end));
                9
            }
            17 => {
                // CommaS = Comma<Name> => ActionFn(34);
                let __sym0 = __pop_NtComma_3cName_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaS(__nt), __end));
                10
            }
            18 => {
                // Command = "def", r#"\\pL+"#, "(", CommaS, ")", ":=", Expr => ActionFn(35);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22_3a_3d_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtCommaS(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __sym0 = __pop_Term_22def_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action35::<>(builder, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            19 => {
                // Command = "eval", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22eval_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            20 => {
                // Command = "bench", Expr => ActionFn(37);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22bench_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            21 => {
                // Command = Expr => ActionFn(38);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            22 => {
                // Expr = Expr, "+", Factor => ActionFn(10);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            23 => {
                // Expr = Expr, "-", Factor => ActionFn(11);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            24 => {
                // Expr = "-", Factor => ActionFn(12);
                let __sym1 = __pop_NtFactor(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            25 => {
                // Expr = Factor => ActionFn(13);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            26 => {
                // Factor = Factor, "*", Pow => ActionFn(14);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            27 => {
                // Factor = Factor, "·", Pow => ActionFn(15);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_b7_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            28 => {
                // Factor = Factor, "/", Pow => ActionFn(16);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            29 => {
                // Factor = Pow => ActionFn(17);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            30 => {
                // Name = r#"\\pL+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                14
            }
            31 => {
                // Num = r#"[0-9]+"# => ActionFn(30);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                15
            }
            32 => {
                // NumFloat = r#"[0-9]+\\.[0-9]+"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFloat(__nt), __end));
                16
            }
            33 => {
                // Op = r#"d/d(\\pL+)"# => ActionFn(18);
                let __sym0 = __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                17
            }
            34 => {
                // Pow = Term, "^", Pow => ActionFn(19);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            35 => {
                // Pow = Term, Pow => ActionFn(20);
                let __sym1 = __pop_NtPow(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            36 => {
                // Pow = Term => ActionFn(21);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            37 => {
                // Term = Num => ActionFn(25);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            38 => {
                // Term = NumFloat => ActionFn(26);
                let __sym0 = __pop_NtNumFloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            39 => {
                // Term = Var => ActionFn(27);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            40 => {
                // Term = Tuple => ActionFn(28);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            41 => {
                // Term = Array => ActionFn(29);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            42 => {
                // Tuple = "(", CommaE, ")" => ActionFn(23);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTuple(__nt), __end));
                20
            }
            43 => {
                // Var = Name => ActionFn(32);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                21
            }
            44 => {
                // __Array = Array => ActionFn(6);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Array(__nt), __end));
                22
            }
            45 => {
                // __CommaE = CommaE => ActionFn(4);
                let __sym0 = __pop_NtCommaE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaE(__nt), __end));
                23
            }
            46 => {
                // __CommaS = CommaS => ActionFn(8);
                let __sym0 = __pop_NtCommaS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaS(__nt), __end));
                24
            }
            47 => {
                // __Command = Command => ActionFn(9);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Command(__nt), __end));
                25
            }
            48 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                26
            }
            49 => {
                // __Factor = Factor => ActionFn(1);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Factor(__nt), __end));
                27
            }
            50 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                28
            }
            51 => {
                // __Pow = Pow => ActionFn(3);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Pow(__nt), __end));
                29
            }
            52 => {
                // __Term = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            53 => {
                // __Tuple = Tuple => ActionFn(5);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Tuple(__nt), __end));
                31
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bench_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bench_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22def_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22def_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22eval_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22eval_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_b7_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_b7_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cpL_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cpL_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArray<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArray(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cName_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cName_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumFloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Array<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Array(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Command<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Command(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Factor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Factor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Pow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Pow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Tuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Tuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Term::parse_Term;

mod __parse__Tuple {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22bench_22(&'input str),
        Term_22def_22(&'input str),
        Term_22eval_22(&'input str),
        Term_22_b7_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5c_5cpL_2b_22_23(&'input str),
        Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(&'input str),
        Nt_28_3cExpr_3e_20_22_2c_22_29(NodeResult),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<NodeResult>),
        Nt_28_3cExpr_3e_20_22_2c_22_29_2b(::std::vec::Vec<NodeResult>),
        Nt_28_3cName_3e_20_22_2c_22_29(&'input str),
        Nt_28_3cName_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'input str>),
        Nt_28_3cName_3e_20_22_2c_22_29_2b(::std::vec::Vec<&'input str>),
        NtArray(NodeResult),
        NtComma_3cExpr_3e(Vec<NodeResult>),
        NtComma_3cName_3e(Vec<&'input str>),
        NtCommaE(Vec<NodeResult>),
        NtCommaS(Vec<&'input str>),
        NtCommand(Result<Command<'input>, Error>),
        NtExpr(NodeResult),
        NtFactor(NodeResult),
        NtName(&'input str),
        NtNum(NodeResult),
        NtNumFloat(NodeResult),
        NtOp(Func),
        NtPow(NodeResult),
        NtTerm(NodeResult),
        NtTuple(NodeResult),
        NtVar(NodeResult),
        Nt____Array(NodeResult),
        Nt____CommaE(Vec<NodeResult>),
        Nt____CommaS(Vec<&'input str>),
        Nt____Command(Result<Command<'input>, Error>),
        Nt____Expr(NodeResult),
        Nt____Factor(NodeResult),
        Nt____Op(Func),
        Nt____Pow(NodeResult),
        Nt____Term(NodeResult),
        Nt____Tuple(NodeResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53,
        // State 2
        3, 0, 0, 0, 0, 17, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 3
        3, 0, 0, 0, 0, 17, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 4
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 5
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 6
        0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, -12, 0, 24, 25, 26, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, -25, 27, -25, -25, -25, 28, 0, 0, -25, 0, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 9
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 10
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 11
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 12
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 13
        3, -36, -36, -36, -36, -36, -36, 0, 18, -36, 31, 0, 0, 0, -36, 19, 20, 21, 0,
        // State 14
        -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 15
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 16
        3, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 17
        3, 0, 0, 0, 0, 17, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 18
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 19
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 20
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 21
        0, -13, 0, 24, 34, 26, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 23
        3, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 24
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 25
        3, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 26
        3, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 27
        3, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 28
        3, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 29
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 30
        3, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 0,
        // State 31
        0, -24, 27, -24, -24, -24, 28, 0, 0, -24, 0, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 34
        0, -22, 27, -22, -22, -22, 28, 0, 0, -22, 0, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 35
        0, -23, 27, -23, -23, -23, 28, 0, 0, -23, 0, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 36
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 37
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 38
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 39
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 40
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -53,
        0,
        0,
        -41,
        -16,
        0,
        0,
        0,
        -43,
        -37,
        -38,
        -29,
        0,
        -40,
        -39,
        0,
        0,
        -31,
        -32,
        -30,
        0,
        -42,
        0,
        -4,
        0,
        0,
        0,
        0,
        -35,
        0,
        0,
        0,
        -5,
        0,
        0,
        -26,
        -28,
        -27,
        -34,
        -11,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 4, 0, 0, 0, 5, 6, 0, 7, 0, 0, 8, 9, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 22, 9, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 30, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 32, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 4, 0, 0, 0, 5, 6, 0, 33, 0, 0, 8, 9, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 35, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 36, 10, 11, 12, 0, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 37, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 38, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 39, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0, 40, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":=""###,
            r###""[""###,
            r###""]""###,
            r###""^""###,
            r###""bench""###,
            r###""def""###,
            r###""eval""###,
            r###""·""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+\\.[0-9]+"#"###,
            r###"r#"\\pL+"#"###,
            r###"r#"d/d(\\pL+)"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Tuple<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
    ) -> Result<NodeResult, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
                (2, _) if true => 17,
                (3, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3a_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22bench_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22def_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22eval_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_b7_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5c_5cpL_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'b,
    >(
        builder: &'b Builder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<NodeResult,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Expr> ",") = Expr, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Expr> ",")* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Expr> ",")+ = Expr, "," => ActionFn(51);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<Name> ",") = Name, "," => ActionFn(46);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<Name> ",")* =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(builder, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<Name> ",")* = (<Name> ",")+ => ActionFn(45);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<Name> ",")+ = Name, "," => ActionFn(55);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<Name> ",")+ = (<Name> ",")+, Name, "," => ActionFn(56);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // Array = "[", CommaE, "]" => ActionFn(24);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArray(__nt), __end));
                6
            }
            12 => {
                // Comma<Expr> = Expr => ActionFn(53);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            13 => {
                // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(54);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cExpr_3e(__nt), __end));
                7
            }
            14 => {
                // Comma<Name> = Name => ActionFn(57);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            15 => {
                // Comma<Name> = (<Name> ",")+, Name => ActionFn(58);
                let __sym1 = __pop_NtName(__symbols);
                let __sym0 = __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cName_3e(__nt), __end));
                8
            }
            16 => {
                // CommaE = Comma<Expr> => ActionFn(22);
                let __sym0 = __pop_NtComma_3cExpr_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaE(__nt), __end));
                9
            }
            17 => {
                // CommaS = Comma<Name> => ActionFn(34);
                let __sym0 = __pop_NtComma_3cName_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommaS(__nt), __end));
                10
            }
            18 => {
                // Command = "def", r#"\\pL+"#, "(", CommaS, ")", ":=", Expr => ActionFn(35);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22_3a_3d_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtCommaS(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __sym0 = __pop_Term_22def_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action35::<>(builder, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            19 => {
                // Command = "eval", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22eval_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            20 => {
                // Command = "bench", Expr => ActionFn(37);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22bench_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            21 => {
                // Command = Expr => ActionFn(38);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCommand(__nt), __end));
                11
            }
            22 => {
                // Expr = Expr, "+", Factor => ActionFn(10);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            23 => {
                // Expr = Expr, "-", Factor => ActionFn(11);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            24 => {
                // Expr = "-", Factor => ActionFn(12);
                let __sym1 = __pop_NtFactor(__symbols);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            25 => {
                // Expr = Factor => ActionFn(13);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                12
            }
            26 => {
                // Factor = Factor, "*", Pow => ActionFn(14);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            27 => {
                // Factor = Factor, "·", Pow => ActionFn(15);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_b7_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            28 => {
                // Factor = Factor, "/", Pow => ActionFn(16);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            29 => {
                // Factor = Pow => ActionFn(17);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                13
            }
            30 => {
                // Name = r#"\\pL+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_5cpL_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtName(__nt), __end));
                14
            }
            31 => {
                // Num = r#"[0-9]+"# => ActionFn(30);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                15
            }
            32 => {
                // NumFloat = r#"[0-9]+\\.[0-9]+"# => ActionFn(31);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFloat(__nt), __end));
                16
            }
            33 => {
                // Op = r#"d/d(\\pL+)"# => ActionFn(18);
                let __sym0 = __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                17
            }
            34 => {
                // Pow = Term, "^", Pow => ActionFn(19);
                let __sym2 = __pop_NtPow(__symbols);
                let __sym1 = __pop_Term_22_5e_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            35 => {
                // Pow = Term, Pow => ActionFn(20);
                let __sym1 = __pop_NtPow(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            36 => {
                // Pow = Term => ActionFn(21);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPow(__nt), __end));
                18
            }
            37 => {
                // Term = Num => ActionFn(25);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            38 => {
                // Term = NumFloat => ActionFn(26);
                let __sym0 = __pop_NtNumFloat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            39 => {
                // Term = Var => ActionFn(27);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            40 => {
                // Term = Tuple => ActionFn(28);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            41 => {
                // Term = Array => ActionFn(29);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                19
            }
            42 => {
                // Tuple = "(", CommaE, ")" => ActionFn(23);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtCommaE(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTuple(__nt), __end));
                20
            }
            43 => {
                // Var = Name => ActionFn(32);
                let __sym0 = __pop_NtName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                21
            }
            44 => {
                // __Array = Array => ActionFn(6);
                let __sym0 = __pop_NtArray(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Array(__nt), __end));
                22
            }
            45 => {
                // __CommaE = CommaE => ActionFn(4);
                let __sym0 = __pop_NtCommaE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaE(__nt), __end));
                23
            }
            46 => {
                // __CommaS = CommaS => ActionFn(8);
                let __sym0 = __pop_NtCommaS(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____CommaS(__nt), __end));
                24
            }
            47 => {
                // __Command = Command => ActionFn(9);
                let __sym0 = __pop_NtCommand(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Command(__nt), __end));
                25
            }
            48 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                26
            }
            49 => {
                // __Factor = Factor => ActionFn(1);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Factor(__nt), __end));
                27
            }
            50 => {
                // __Op = Op => ActionFn(2);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Op(__nt), __end));
                28
            }
            51 => {
                // __Pow = Pow => ActionFn(3);
                let __sym0 = __pop_NtPow(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Pow(__nt), __end));
                29
            }
            52 => {
                // __Term = Term => ActionFn(7);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                30
            }
            53 => {
                // __Tuple = Tuple => ActionFn(5);
                let __sym0 = __pop_NtTuple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bench_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bench_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22def_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22def_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22eval_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22eval_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_b7_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_b7_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cpL_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cpL_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22d_2fd_28_5c_5cpL_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cExpr_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cExpr_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cName_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cName_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArray<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArray(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cExpr_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cExpr_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cName_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cName_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCommand<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCommand(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumFloat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFloat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Array<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Array(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<NodeResult>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CommaS<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CommaS(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Command<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Result<Command<'input>, Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Command(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Factor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Factor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Op<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Func, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Op(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Pow<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Pow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Tuple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NodeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Tuple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Tuple::parse_Tuple;
mod __intern_token {
    #![allow(unused_imports)]
    use builder::{Builder, NodeResult};
    use func::Func;
    use eval::Command;
    use error::Error;
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:[0-9])+",
                "^(?u:[0-9])+(?u:\\.)(?u:[0-9])+",
                "^(?u:[A-Za-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮͰ-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁҊ-ԯԱ-Ֆՙ-ՙա-ևא-תװ-ײؠ-يٮ-ٯٱ-ۓە-ەۥ-ۦۮ-ۯۺ-ۼۿ-ۿܐ-ܐܒ-ܯݍ-ޥޱ-ޱߊ-ߪߴ-ߵߺ-ߺࠀ-ࠕࠚ-ࠚࠤ-ࠤࠨ-ࠨࡀ-ࡘࢠ-ࢴऄ-हऽ-ऽॐ-ॐक़-ॡॱ-ঀঅ-ঌএ-ঐও-নপ-রল-লশ-হঽ-ঽৎ-ৎড়-ঢ়য়-ৡৰ-ৱਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹਖ਼-ੜਫ਼-ਫ਼ੲ-ੴઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હઽ-ઽૐ-ૐૠ-ૡૹ-ૹଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହଽ-ଽଡ଼-ଢ଼ୟ-ୡୱ-ୱஃ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹௐ-ௐఅ-ఌఎ-ఐఒ-నప-హఽ-ఽౘ-ౚౠ-ౡಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹಽ-ಽೞ-ೞೠ-ೡೱ-ೲഅ-ഌഎ-ഐഒ-ഺഽ-ഽൎ-ൎൟ-ൡൺ-ൿඅ-ඖක-නඳ-රල-ලව-ෆก-ะา-ำเ-ๆກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ະາ-ຳຽ-ຽເ-ໄໆ-ໆໜ-ໟༀ-ༀཀ-ཇཉ-ཬྈ-ྌက-ဪဿ-ဿၐ-ၕၚ-ၝၡ-ၡၥ-ၦၮ-ၰၵ-ႁႎ-ႎႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛱ-ᛸᜀ-ᜌᜎ-ᜑᜠ-ᜱᝀ-ᝑᝠ-ᝬᝮ-ᝰក-ឳៗ-ៗៜ-ៜᠠ-ᡷᢀ-ᢨᢪ-ᢪᢰ-ᣵᤀ-ᤞᥐ-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉᨀ-ᨖᨠ-ᩔᪧ-ᪧᬅ-ᬳᭅ-ᭋᮃ-ᮠᮮ-ᮯᮺ-ᯥᰀ-ᰣᱍ-ᱏᱚ-ᱽᳩ-ᳬᳮ-ᳱᳵ-ᳶᴀ-ᶿḀ-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼⁱ-ⁱⁿ-ⁿₐ-ₜℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎↃ-ↄⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳮⳲ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯⶀ-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⸯ-ⸯ々-〆〱-〵〻-〼ぁ-ゖゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘟꘪ-ꘫꙀ-ꙮꙿ-ꚝꚠ-ꛥꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠁꠃ-ꠅꠇ-ꠊꠌ-ꠢꡀ-ꡳꢂ-ꢳꣲ-ꣷꣻ-ꣻꣽ-ꣽꤊ-ꤥꤰ-ꥆꥠ-ꥼꦄ-ꦲꧏ-ꧏꧠ-ꧤꧦ-ꧯꧺ-ꧾꨀ-ꨨꩀ-ꩂꩄ-ꩋꩠ-ꩶꩺ-ꩺꩾ-ꪯꪱ-ꪱꪵ-ꪶꪹ-ꪽꫀ-ꫀꫂ-ꫂꫛ-ꫝꫠ-ꫪꫲ-ꫴꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯢ가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-יִײַ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻﹰ-ﹴﹶ-ﻼＡ-Ｚａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐊀-𐊜𐊠-𐋐𐌀-𐌟𐌰-𐍀𐍂-𐍉𐍐-𐍵𐎀-𐎝𐎠-𐏃𐏈-𐏏𐐀-𐒝𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨀𐨐-𐨓𐨕-𐨗𐨙-𐨳𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫤𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀃-𑀷𑂃-𑂯𑃐-𑃨𑄃-𑄦𑅐-𑅲𑅶-𑅶𑆃-𑆲𑇁-𑇄𑇚-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈫𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋞𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌽-𑌽𑍐-𑍐𑍝-𑍡𑒀-𑒯𑓄-𑓅𑓇-𑓇𑖀-𑖮𑗘-𑗛𑘀-𑘯𑙄-𑙄𑚀-𑚪𑜀-𑜙𑢠-𑣟𑣿-𑣿𑫀-𑫸𒀀-𒎙𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖫐-𖫭𖬀-𖬯𖭀-𖭃𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽐𖾓-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𞠀-𞣄𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀])+",
                "^(?u:d/d)((?u:[A-Za-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮͰ-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁҊ-ԯԱ-Ֆՙ-ՙա-ևא-תװ-ײؠ-يٮ-ٯٱ-ۓە-ەۥ-ۦۮ-ۯۺ-ۼۿ-ۿܐ-ܐܒ-ܯݍ-ޥޱ-ޱߊ-ߪߴ-ߵߺ-ߺࠀ-ࠕࠚ-ࠚࠤ-ࠤࠨ-ࠨࡀ-ࡘࢠ-ࢴऄ-हऽ-ऽॐ-ॐक़-ॡॱ-ঀঅ-ঌএ-ঐও-নপ-রল-লশ-হঽ-ঽৎ-ৎড়-ঢ়য়-ৡৰ-ৱਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹਖ਼-ੜਫ਼-ਫ਼ੲ-ੴઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હઽ-ઽૐ-ૐૠ-ૡૹ-ૹଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହଽ-ଽଡ଼-ଢ଼ୟ-ୡୱ-ୱஃ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹௐ-ௐఅ-ఌఎ-ఐఒ-నప-హఽ-ఽౘ-ౚౠ-ౡಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹಽ-ಽೞ-ೞೠ-ೡೱ-ೲഅ-ഌഎ-ഐഒ-ഺഽ-ഽൎ-ൎൟ-ൡൺ-ൿඅ-ඖක-නඳ-රල-ලව-ෆก-ะา-ำเ-ๆກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ະາ-ຳຽ-ຽເ-ໄໆ-ໆໜ-ໟༀ-ༀཀ-ཇཉ-ཬྈ-ྌက-ဪဿ-ဿၐ-ၕၚ-ၝၡ-ၡၥ-ၦၮ-ၰၵ-ႁႎ-ႎႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛱ-ᛸᜀ-ᜌᜎ-ᜑᜠ-ᜱᝀ-ᝑᝠ-ᝬᝮ-ᝰក-ឳៗ-ៗៜ-ៜᠠ-ᡷᢀ-ᢨᢪ-ᢪᢰ-ᣵᤀ-ᤞᥐ-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉᨀ-ᨖᨠ-ᩔᪧ-ᪧᬅ-ᬳᭅ-ᭋᮃ-ᮠᮮ-ᮯᮺ-ᯥᰀ-ᰣᱍ-ᱏᱚ-ᱽᳩ-ᳬᳮ-ᳱᳵ-ᳶᴀ-ᶿḀ-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼⁱ-ⁱⁿ-ⁿₐ-ₜℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎↃ-ↄⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳮⳲ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯⶀ-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⸯ-ⸯ々-〆〱-〵〻-〼ぁ-ゖゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘟꘪ-ꘫꙀ-ꙮꙿ-ꚝꚠ-ꛥꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠁꠃ-ꠅꠇ-ꠊꠌ-ꠢꡀ-ꡳꢂ-ꢳꣲ-ꣷꣻ-ꣻꣽ-ꣽꤊ-ꤥꤰ-ꥆꥠ-ꥼꦄ-ꦲꧏ-ꧏꧠ-ꧤꧦ-ꧯꧺ-ꧾꨀ-ꨨꩀ-ꩂꩄ-ꩋꩠ-ꩶꩺ-ꩺꩾ-ꪯꪱ-ꪱꪵ-ꪶꪹ-ꪽꫀ-ꫀꫂ-ꫂꫛ-ꫝꫠ-ꫪꫲ-ꫴꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯢ가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-יִײַ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻﹰ-ﹴﹶ-ﻼＡ-Ｚａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐊀-𐊜𐊠-𐋐𐌀-𐌟𐌰-𐍀𐍂-𐍉𐍐-𐍵𐎀-𐎝𐎠-𐏃𐏈-𐏏𐐀-𐒝𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨀𐨐-𐨓𐨕-𐨗𐨙-𐨳𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫤𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀃-𑀷𑂃-𑂯𑃐-𑃨𑄃-𑄦𑅐-𑅲𑅶-𑅶𑆃-𑆲𑇁-𑇄𑇚-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈫𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋞𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌽-𑌽𑍐-𑍐𑍝-𑍡𑒀-𑒯𑓄-𑓅𑓇-𑓇𑖀-𑖮𑗘-𑗛𑘀-𑘯𑙄-𑙄𑚀-𑚪𑜀-𑜙𑢠-𑣟𑣿-𑣿𑫀-𑫸𒀀-𒎙𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖫐-𖫭𖬀-𖬯𖭀-𖭃𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽐𖾓-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𞠀-𞣄𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀])+)",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:\\*)",
                "^(?u:\\+)",
                "^(?u:,)",
                "^(?u:\\-)",
                "^(?u:/)",
                "^(?u::=)",
                "^(?u:\\[)",
                "^(?u:\\])",
                "^(?u:\\^)",
                "^(?u:bench)",
                "^(?u:def)",
                "^(?u:eval)",
                "^(?u:·)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+(?u:\\.)(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:[A-Za-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮͰ-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁҊ-ԯԱ-Ֆՙ-ՙա-ևא-תװ-ײؠ-يٮ-ٯٱ-ۓە-ەۥ-ۦۮ-ۯۺ-ۼۿ-ۿܐ-ܐܒ-ܯݍ-ޥޱ-ޱߊ-ߪߴ-ߵߺ-ߺࠀ-ࠕࠚ-ࠚࠤ-ࠤࠨ-ࠨࡀ-ࡘࢠ-ࢴऄ-हऽ-ऽॐ-ॐक़-ॡॱ-ঀঅ-ঌএ-ঐও-নপ-রল-লশ-হঽ-ঽৎ-ৎড়-ঢ়য়-ৡৰ-ৱਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹਖ਼-ੜਫ਼-ਫ਼ੲ-ੴઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હઽ-ઽૐ-ૐૠ-ૡૹ-ૹଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହଽ-ଽଡ଼-ଢ଼ୟ-ୡୱ-ୱஃ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹௐ-ௐఅ-ఌఎ-ఐఒ-నప-హఽ-ఽౘ-ౚౠ-ౡಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹಽ-ಽೞ-ೞೠ-ೡೱ-ೲഅ-ഌഎ-ഐഒ-ഺഽ-ഽൎ-ൎൟ-ൡൺ-ൿඅ-ඖක-නඳ-රල-ලව-ෆก-ะา-ำเ-ๆກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ະາ-ຳຽ-ຽເ-ໄໆ-ໆໜ-ໟༀ-ༀཀ-ཇཉ-ཬྈ-ྌက-ဪဿ-ဿၐ-ၕၚ-ၝၡ-ၡၥ-ၦၮ-ၰၵ-ႁႎ-ႎႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛱ-ᛸᜀ-ᜌᜎ-ᜑᜠ-ᜱᝀ-ᝑᝠ-ᝬᝮ-ᝰក-ឳៗ-ៗៜ-ៜᠠ-ᡷᢀ-ᢨᢪ-ᢪᢰ-ᣵᤀ-ᤞᥐ-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉᨀ-ᨖᨠ-ᩔᪧ-ᪧᬅ-ᬳᭅ-ᭋᮃ-ᮠᮮ-ᮯᮺ-ᯥᰀ-ᰣᱍ-ᱏᱚ-ᱽᳩ-ᳬᳮ-ᳱᳵ-ᳶᴀ-ᶿḀ-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼⁱ-ⁱⁿ-ⁿₐ-ₜℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎↃ-ↄⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳮⳲ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯⶀ-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⸯ-ⸯ々-〆〱-〵〻-〼ぁ-ゖゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘟꘪ-ꘫꙀ-ꙮꙿ-ꚝꚠ-ꛥꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠁꠃ-ꠅꠇ-ꠊꠌ-ꠢꡀ-ꡳꢂ-ꢳꣲ-ꣷꣻ-ꣻꣽ-ꣽꤊ-ꤥꤰ-ꥆꥠ-ꥼꦄ-ꦲꧏ-ꧏꧠ-ꧤꧦ-ꧯꧺ-ꧾꨀ-ꨨꩀ-ꩂꩄ-ꩋꩠ-ꩶꩺ-ꩺꩾ-ꪯꪱ-ꪱꪵ-ꪶꪹ-ꪽꫀ-ꫀꫂ-ꫂꫛ-ꫝꫠ-ꫪꫲ-ꫴꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯢ가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-יִײַ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻﹰ-ﹴﹶ-ﻼＡ-Ｚａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐊀-𐊜𐊠-𐋐𐌀-𐌟𐌰-𐍀𐍂-𐍉𐍐-𐍵𐎀-𐎝𐎠-𐏃𐏈-𐏏𐐀-𐒝𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨀𐨐-𐨓𐨕-𐨗𐨙-𐨳𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫤𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀃-𑀷𑂃-𑂯𑃐-𑃨𑄃-𑄦𑅐-𑅲𑅶-𑅶𑆃-𑆲𑇁-𑇄𑇚-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈫𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋞𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌽-𑌽𑍐-𑍐𑍝-𑍡𑒀-𑒯𑓄-𑓅𑓇-𑓇𑖀-𑖮𑗘-𑗛𑘀-𑘯𑙄-𑙄𑚀-𑚪𑜀-𑜙𑢠-𑣟𑣿-𑣿𑫀-𑫸𒀀-𒎙𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖫐-𖫭𖬀-𖬯𖭀-𖭃𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽐𖾓-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𞠀-𞣄𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀])+").unwrap(),
                __regex::Regex::new("^(?u:d/d)((?u:[A-Za-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮͰ-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁҊ-ԯԱ-Ֆՙ-ՙա-ևא-תװ-ײؠ-يٮ-ٯٱ-ۓە-ەۥ-ۦۮ-ۯۺ-ۼۿ-ۿܐ-ܐܒ-ܯݍ-ޥޱ-ޱߊ-ߪߴ-ߵߺ-ߺࠀ-ࠕࠚ-ࠚࠤ-ࠤࠨ-ࠨࡀ-ࡘࢠ-ࢴऄ-हऽ-ऽॐ-ॐक़-ॡॱ-ঀঅ-ঌএ-ঐও-নপ-রল-লশ-হঽ-ঽৎ-ৎড়-ঢ়য়-ৡৰ-ৱਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹਖ਼-ੜਫ਼-ਫ਼ੲ-ੴઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હઽ-ઽૐ-ૐૠ-ૡૹ-ૹଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହଽ-ଽଡ଼-ଢ଼ୟ-ୡୱ-ୱஃ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹௐ-ௐఅ-ఌఎ-ఐఒ-నప-హఽ-ఽౘ-ౚౠ-ౡಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹಽ-ಽೞ-ೞೠ-ೡೱ-ೲഅ-ഌഎ-ഐഒ-ഺഽ-ഽൎ-ൎൟ-ൡൺ-ൿඅ-ඖක-නඳ-රල-ලව-ෆก-ะา-ำเ-ๆກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ະາ-ຳຽ-ຽເ-ໄໆ-ໆໜ-ໟༀ-ༀཀ-ཇཉ-ཬྈ-ྌက-ဪဿ-ဿၐ-ၕၚ-ၝၡ-ၡၥ-ၦၮ-ၰၵ-ႁႎ-ႎႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛱ-ᛸᜀ-ᜌᜎ-ᜑᜠ-ᜱᝀ-ᝑᝠ-ᝬᝮ-ᝰក-ឳៗ-ៗៜ-ៜᠠ-ᡷᢀ-ᢨᢪ-ᢪᢰ-ᣵᤀ-ᤞᥐ-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉᨀ-ᨖᨠ-ᩔᪧ-ᪧᬅ-ᬳᭅ-ᭋᮃ-ᮠᮮ-ᮯᮺ-ᯥᰀ-ᰣᱍ-ᱏᱚ-ᱽᳩ-ᳬᳮ-ᳱᳵ-ᳶᴀ-ᶿḀ-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼⁱ-ⁱⁿ-ⁿₐ-ₜℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎↃ-ↄⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳮⳲ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯⶀ-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⸯ-ⸯ々-〆〱-〵〻-〼ぁ-ゖゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘟꘪ-ꘫꙀ-ꙮꙿ-ꚝꚠ-ꛥꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠁꠃ-ꠅꠇ-ꠊꠌ-ꠢꡀ-ꡳꢂ-ꢳꣲ-ꣷꣻ-ꣻꣽ-ꣽꤊ-ꤥꤰ-ꥆꥠ-ꥼꦄ-ꦲꧏ-ꧏꧠ-ꧤꧦ-ꧯꧺ-ꧾꨀ-ꨨꩀ-ꩂꩄ-ꩋꩠ-ꩶꩺ-ꩺꩾ-ꪯꪱ-ꪱꪵ-ꪶꪹ-ꪽꫀ-ꫀꫂ-ꫂꫛ-ꫝꫠ-ꫪꫲ-ꫴꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯢ가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-יִײַ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻﹰ-ﹴﹶ-ﻼＡ-Ｚａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐊀-𐊜𐊠-𐋐𐌀-𐌟𐌰-𐍀𐍂-𐍉𐍐-𐍵𐎀-𐎝𐎠-𐏃𐏈-𐏏𐐀-𐒝𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨀𐨐-𐨓𐨕-𐨗𐨙-𐨳𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫤𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀃-𑀷𑂃-𑂯𑃐-𑃨𑄃-𑄦𑅐-𑅲𑅶-𑅶𑆃-𑆲𑇁-𑇄𑇚-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈫𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋞𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌽-𑌽𑍐-𑍐𑍝-𑍡𑒀-𑒯𑓄-𑓅𑓇-𑓇𑖀-𑖮𑗘-𑗛𑘀-𑘯𑙄-𑙄𑚀-𑚪𑜀-𑜙𑢠-𑣟𑣿-𑣿𑫀-𑫸𒀀-𒎙𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖫐-𖫭𖬀-𖬯𖭀-𖭃𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽐𖾓-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𞠀-𞣄𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀])+)").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\*)").unwrap(),
                __regex::Regex::new("^(?u:\\+)").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u:\\-)").unwrap(),
                __regex::Regex::new("^(?u:/)").unwrap(),
                __regex::Regex::new("^(?u::=)").unwrap(),
                __regex::Regex::new("^(?u:\\[)").unwrap(),
                __regex::Regex::new("^(?u:\\])").unwrap(),
                __regex::Regex::new("^(?u:\\^)").unwrap(),
                __regex::Regex::new("^(?u:bench)").unwrap(),
                __regex::Regex::new("^(?u:def)").unwrap(),
                __regex::Regex::new("^(?u:eval)").unwrap(),
                __regex::Regex::new("^(?u:·)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 19 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, Func, usize),
) -> Func
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, Vec<NodeResult>, usize),
) -> Vec<NodeResult>
{
    (__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action7<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action8<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, Vec<&'input str>, usize),
) -> Vec<&'input str>
{
    (__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, Result<Command<'input>, Error>, usize),
) -> Result<Command<'input>, Error>
{
    (__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, l, _): (usize, NodeResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeResult, usize),
) -> NodeResult
{
    builder.add(l?, r?)
}

#[allow(unused_variables)]
fn __action11<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, l, _): (usize, NodeResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeResult, usize),
) -> NodeResult
{
    builder.sub(l?, r?)
}

#[allow(unused_variables)]
fn __action12<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, f, _): (usize, NodeResult, usize),
) -> NodeResult
{
    builder.neg(f?)
}

#[allow(unused_variables)]
fn __action13<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, l, _): (usize, NodeResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeResult, usize),
) -> NodeResult
{
    builder.mul(l?, r?)
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, l, _): (usize, NodeResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeResult, usize),
) -> NodeResult
{
    builder.mul(l?, r?)
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, l, _): (usize, NodeResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, NodeResult, usize),
) -> NodeResult
{
    builder.div(l?, r?)
}

#[allow(unused_variables)]
fn __action17<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action18<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, v, _): (usize, &'input str, usize),
) -> Func
{
    Func::Diff(v[3..].into())
}

#[allow(unused_variables)]
fn __action19<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, b, _): (usize, NodeResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, NodeResult, usize),
) -> NodeResult
{
    builder.pow(b?, e?)
}

#[allow(unused_variables)]
fn __action20<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, l, _): (usize, NodeResult, usize),
    (_, r, _): (usize, NodeResult, usize),
) -> NodeResult
{
    builder.apply(l?, r?)
}

#[allow(unused_variables)]
fn __action21<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action22<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, Vec<NodeResult>, usize),
) -> Vec<NodeResult>
{
    (__0)
}

#[allow(unused_variables)]
fn __action23<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Vec<NodeResult>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> NodeResult
{
    {
        let mut t = t;
        match t.len() {
	    1 => t.pop().unwrap(),
	    _ => builder.tuple(t)
        }
    }
}

#[allow(unused_variables)]
fn __action24<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Vec<NodeResult>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> NodeResult
{
    builder.array(t)
}

#[allow(unused_variables)]
fn __action25<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action26<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action27<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action28<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action29<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action30<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> NodeResult
{
    builder.decimal(s)
}

#[allow(unused_variables)]
fn __action31<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> NodeResult
{
    builder.decimal_float(s)
}

#[allow(unused_variables)]
fn __action32<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> NodeResult
{
    Ok(builder.var(s))
}

#[allow(unused_variables)]
fn __action33<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
fn __action34<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, Vec<&'input str>, usize),
) -> Vec<&'input str>
{
    (__0)
}

#[allow(unused_variables)]
fn __action35<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, f, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, Vec<&'input str>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, NodeResult, usize),
) -> Result<Command<'input>, Error>
{
    Ok(Command::Define(f, a, e?))
}

#[allow(unused_variables)]
fn __action36<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, NodeResult, usize),
) -> Result<Command<'input>, Error>
{
    Ok(Command::Eval(e?))
}

#[allow(unused_variables)]
fn __action37<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, NodeResult, usize),
) -> Result<Command<'input>, Error>
{
    Ok(Command::Bench(e?))
}

#[allow(unused_variables)]
fn __action38<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, e, _): (usize, NodeResult, usize),
) -> Result<Command<'input>, Error>
{
    Ok(Command::Expr(e?))
}

#[allow(unused_variables)]
fn __action39<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, e, _): (usize, &'input str, usize),
) -> Vec<&'input str>
{
    {
        let mut v = v;
        v.push(e );
        v
    }
}

#[allow(unused_variables)]
fn __action40<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<NodeResult>, usize),
    (_, e, _): (usize, NodeResult, usize),
) -> Vec<NodeResult>
{
    {
        let mut v = v;
        v.push(e );
        v
    }
}

#[allow(unused_variables)]
fn __action41<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<NodeResult>
{
    vec![]
}

#[allow(unused_variables)]
fn __action42<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<NodeResult>, usize),
) -> ::std::vec::Vec<NodeResult>
{
    v
}

#[allow(unused_variables)]
fn __action43<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
    (_, _, _): (usize, &'input str, usize),
) -> NodeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action44<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<&'input str>
{
    vec![]
}

#[allow(unused_variables)]
fn __action45<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> ::std::vec::Vec<&'input str>
{
    v
}

#[allow(unused_variables)]
fn __action46<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
fn __action47<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action48<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, e, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action49<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, __0, _): (usize, NodeResult, usize),
) -> ::std::vec::Vec<NodeResult>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action50<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<NodeResult>, usize),
    (_, e, _): (usize, NodeResult, usize),
) -> ::std::vec::Vec<NodeResult>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action51<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    __0: (usize, NodeResult, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<NodeResult>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action43(
        builder,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        builder,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<NodeResult>, usize),
    __1: (usize, NodeResult, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<NodeResult>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action43(
        builder,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        builder,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action53<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    __0: (usize, NodeResult, usize),
) -> Vec<NodeResult>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action41(
        builder,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        builder,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action54<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<NodeResult>, usize),
    __1: (usize, NodeResult, usize),
) -> Vec<NodeResult>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action42(
        builder,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        builder,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action55<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action46(
        builder,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        builder,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action56<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action46(
        builder,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        builder,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action57<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Vec<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action44(
        builder,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        builder,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action58<
    'input,
    'b,
>(
    builder: &'b Builder,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
) -> Vec<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action45(
        builder,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        builder,
        input,
        __temp0,
        __1,
    )
}

pub trait __ToTriple<'input, 'b, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, 'b, > __ToTriple<'input, 'b, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, 'b, > __ToTriple<'input, 'b, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
