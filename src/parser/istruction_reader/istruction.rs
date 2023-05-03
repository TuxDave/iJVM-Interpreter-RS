use crate::parser::Istruction::{*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Istruction {
    Bipush(i8),
    Dup,
    Goto(i16),
    IAdd,
    IAnd,
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
    Ldc_W(u16),
    Nop,
    Pop,
    Swap,
    #[allow(non_camel_case_types)]
    Wide_IInc(u16, i8),
    #[allow(non_camel_case_types)]
    Wide_ILoad(u16),
    #[allow(non_camel_case_types)]
    Wide_IStore(u16),
    Halt,
}

impl Istruction {
    pub const BIPUSH: u8 = 0x10;
    pub const DUP: u8 = 0x59;
    pub const GOTO: u8 = 0xA7;
    pub const IADD: u8 = 0x60;
    pub const IAND: u8 = 0x7E;
    pub const IFEQ: u8 = 0x99;
    pub const IFLT: u8 = 0x9B;
    pub const IF_ICMPEQ: u8 = 0x9F;
    pub const IINC: u8 = 0x84;
    pub const ILOAD: u8 = 0x15;
    pub const INVOKEVIRTUAL: u8 = 0xB6;
    pub const IOR: u8 = 0x80;
    pub const IRETURN: u8 = 0xAC;
    pub const ISTORE: u8 = 0x36;
    pub const ISUB: u8 = 0x64;
    pub const LDC_W: u8 = 0x13;
    pub const NOP: u8 = 0x00;
    pub const POP: u8 = 0x57;
    pub const SWAP: u8 = 0x5F;
    pub const WIDE: u8 = 0xC4;
    pub const HALT: u8 = 0xFF;
    pub const OPCODES: [u8; 21] = [Istruction::BIPUSH, Istruction::DUP, Istruction::GOTO, Istruction::IADD,
        Istruction::IAND, Istruction::IFEQ, Istruction::IFLT, Istruction::IF_ICMPEQ, Istruction::IINC,
        Istruction::ILOAD, Istruction::INVOKEVIRTUAL, Istruction::IOR, Istruction::IRETURN,
        Istruction::ISTORE, Istruction::ISUB, Istruction::LDC_W, Istruction::NOP, Istruction::POP,
        Istruction::SWAP, Istruction::WIDE, Istruction::HALT];

    pub fn from_opcode(opcode: u8, params: &[u8]) -> Istruction {
        return match opcode {
            Istruction::BIPUSH => Bipush(params[0] as i8),
            Istruction::DUP => Dup,
            Istruction::GOTO => Goto(i16::from_be_bytes([params[0], params[1]])),
            Istruction::IADD => IAdd,
            Istruction::IAND => IAnd,
            Istruction::IFEQ => IfEq(i16::from_be_bytes([params[0], params[1]])),
            Istruction::IFLT => IfLt(i16::from_be_bytes([params[0], params[1]])),
            Istruction::IF_ICMPEQ => IfICmpEq(i16::from_be_bytes([params[0], params[1]])),
            Istruction::IINC => IInc(params[0], params[1] as i8),
            Istruction::ILOAD => ILoad(params[0]),
            Istruction::INVOKEVIRTUAL => InvokeVirtual(u16::from_be_bytes([params[0], params[1]])),
            Istruction::IOR => IOr,
            Istruction::IRETURN => IReturn,
            Istruction::ISTORE => IStore(params[0]),
            Istruction::ISUB => ISub,
            Istruction::LDC_W => Ldc_W(u16::from_be_bytes([params[0], params[1]])),
            Istruction::NOP => Nop,
            Istruction::POP => Pop,
            Istruction::HALT => Halt,
            _ => Nop
        };
    }

    pub fn from_wide_opcode(little_opcode: u8, params: &[u8]) -> Istruction {
        return match little_opcode {
            Istruction::IINC => Wide_IInc(u16::from_be_bytes([params[0], params[1]]), params[2] as i8),
            Istruction::ILOAD => Wide_ILoad(u16::from_be_bytes([params[0], params[1]])),
            Istruction::ISTORE => Wide_IStore(u16::from_be_bytes([params[0], params[1]])),
            _ => Nop
        };
    }
}

#[derive(PartialEq)]
pub struct ParamType {
    pub bytes_num: u8,
    pub signed: bool,
}

impl ParamType {
    const UBYTE: Self = ParamType { bytes_num: 1, signed: false };
    const IBYTE: Self = ParamType { bytes_num: 1, signed: true };
    const OFFSET: Self = ParamType { bytes_num: 2, signed: true };
    const CONST: Self = ParamType::IBYTE;
    const VARNUM: Self = ParamType::UBYTE;
    const INDEX: Self = ParamType { bytes_num: 2, signed: false };
    const DISP: Self = ParamType::INDEX;
    const WIDE_VARNUM: Self = ParamType::INDEX;
    const NOPARAM: Self = ParamType { bytes_num: 0, signed: false };

    /**
    opcode: 16 bit big endian value with the opcode in the first 8, if wide, the opcode in the second 8
     */
    pub fn from_opcode_wide(opcode: u16) -> (ParamType, Option<ParamType>) {
        let arr: [u8; 2] = opcode.to_be_bytes();
        let wide = arr[0] == Istruction::WIDE;
        let opcode = arr[1];

        return match opcode {
            Istruction::BIPUSH => (ParamType::IBYTE, None),
            Istruction::GOTO | Istruction::IFEQ | Istruction::IFLT | Istruction::IF_ICMPEQ => (ParamType::OFFSET, None),
            Istruction::ILOAD | Istruction::ISTORE => (if !wide { ParamType::VARNUM } else { ParamType::WIDE_VARNUM }, None),
            Istruction::LDC_W => (ParamType::INDEX, None),
            Istruction::IINC => (if !wide { ParamType::VARNUM } else { ParamType::WIDE_VARNUM }, Some(ParamType::CONST)),
            Istruction::INVOKEVIRTUAL => (ParamType::DISP, None),
            _ => (ParamType::NOPARAM, None)
        };
    }

    pub fn from_opcode(opcode: u8) -> (ParamType, Option<ParamType>) {
        let opcode = u16::from_be_bytes([0, opcode]);
        return Self::from_opcode_wide(opcode);
    }
}