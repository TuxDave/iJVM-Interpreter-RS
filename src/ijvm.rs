mod method_area;

pub use method_area::MethodArea;
use crate::parser::{Istruction, ParamType};

#[allow(non_camel_case_types)]
pub struct IJVM {
    pc: usize, // program counter
    ir: Option<Istruction>, // istruction "register"
    method_area: MethodArea,
}
impl IJVM {
    fn fetch (&mut self) {

        fn sub_fetch(this: &mut IJVM, put: &mut u8) -> bool {
            return if let Ok(byte) = this.method_area.fetch_absolute(this.pc) {
                this.pc += 1;
                *put = byte;
                true
            } else {
                false
            }
        }

        let mut opcode: u8 = 0;
        let mut little: u8 = 0;
        if sub_fetch(self, &mut opcode) {
            if Istruction::OPCODES.contains(&opcode) {
                let mut params: (ParamType, Option<ParamType>);
                if opcode == Istruction::WIDE {
                    // let mut little = 0;
                    let mut wopcode: u16;
                    if sub_fetch(self, &mut little) {
                        wopcode = u16::from_be_bytes([opcode, little])
                    } else {
                        //TODO: trigger error listener
                        eprintln!("Error in fetching, bytecode finished.");
                        return;
                    }
                    params = ParamType::from_opcode_wide(wopcode);
                } else {
                    params = ParamType::from_opcode(opcode);
                }

                //fetching parameters
                let mut p0 = vec![];
                let mut p1 = None;
                for _ in 0 .. params.0.bytes_num {
                    let mut temp = 0;
                    if sub_fetch(self, &mut temp) {
                        p0.push(temp);
                    } else {
                        //TODO: trigger error listener
                        eprintln!("Error in fetching, bytecode finished.");
                        return;
                    }
                }
                if let Some(pt) = params.1 {
                    if !sub_fetch(self, p1.as_mut().unwrap()) {
                        //TODO: trigger error listener
                        eprintln!("Error in fetching, bytecode finished.");
                        return;
                    }
                }

                //abbiamo i parametri easy e TECNICAMENTE il pc punta al prossimo opcode
                if opcode == Istruction::WIDE {
                    self.ir = Some(Istruction::from_wide_opcode(little));
                } else {
                    self.ir = Some(Istruction::from_opcode(opcode));
                }
            } else {
                //TODO: trigger error listener
                eprintln!("Error in fetching, opcode ({opcode}) non existing");
                return;
            }
        } else {
            //TODO: trigger error listener
            eprintln!("Error in fetching");
            return;
        }
    }
}