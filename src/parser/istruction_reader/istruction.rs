pub enum Istruction {
    Bipush(i8),
    Dup,
    Goto(i16),
    IAdd,
    IfEq(i16),
    IfLt(i16),
    IfICmpEq(i16),
    IInc(u8, i8),
    ILoad(u8),
    InvokeVirtual(u16),
    IOr,
    IReturn,
    IStore(u8),
    ISub,
    #[allow(non_camel_case_types)]
    LDC_W(u16),
    Nop,
    Pop,
    Swap,
    #[allow(non_camel_case_types)]
    Wide_IInc(u16, i8),
    #[allow(non_camel_case_types)]
    Wide_ILoad(u16),
    #[allow(non_camel_case_types)]
    Wide_IStore(u16),
    Halt
}

// pub struct ParamType {
//     bytes_num: u8,
//     signed: bool
// }
// impl ParamType {
//     const UBYTE: Self = ParamType {bytes_num: 1, signed: false};
//     const IBYTE: Self = ParamType {bytes_num: 1, signed: true};
//     const OFFSET: Self = ParamType {bytes_num: 2, signed: true};
//     const CONST: Self = ParamType::IBYTE;
//     const VARNUM: Self = ParamType::UBYTE;
//     const INDEX: Self = ParamType {bytes_num: 2, signed: false};
//     const DISP: Self = ParamType::INDEX;
//     const WIDE_VARNUM: Self = ParamType::INDEX;
// }