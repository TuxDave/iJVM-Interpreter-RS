pub use method_area::MethodArea;

use crate::parser::{Istruction, ParamType};

mod method_area;
mod execute_istr;

#[allow(non_camel_case_types)]
pub struct IJVM<'a> {
    pc: usize,
    // program counter
    ir: Option<Istruction>,
    // istruction "register"
    method_area: MethodArea,
    constant_pool: &'a [i32],
    local_variables: Vec<Vec<i32>>,
    stack: Vec<Vec<i32>>,
    error: bool
}

impl<'a> IJVM<'a> {
    pub fn new(method_area: MethodArea, constant_pool: &[i32]) -> IJVM {
        return IJVM {
            pc: 0,
            ir: None,
            method_area,
            constant_pool,
            local_variables: vec![vec![]],
            stack: vec![vec![]],
            error: false
        };
    }

    fn fetch_decode(&mut self) {
        fn sub_fetch(this: &mut IJVM, put: &mut u8) -> Result<(), String> {
            let fetched = this.method_area.fetch_absolute(this.pc);
            return if let Ok(byte) = fetched {
                this.pc += 1;
                *put = byte;
                Ok(())
            } else {
                Err(fetched.err().unwrap())
            };
        }

        let mut opcode: u8 = 0;
        let mut little: u8 = 0;
        if let Err(msg) = sub_fetch(self, &mut opcode) {
            //TODO: trigger error listener
            self.ir = None;
            eprintln!("{msg}");
            return;
        } else {
            if Istruction::OPCODES.contains(&opcode) {
                let params: (ParamType, Option<ParamType>);
                if opcode == Istruction::WIDE {
                    // let mut little = 0;
                    let wopcode: u16;
                    if let Err(msg) = sub_fetch(self, &mut little) {
                        //TODO: trigger error listener
                        eprintln!("{msg}");
                        return;
                    } else {
                        wopcode = u16::from_be_bytes([opcode, little])
                    }
                    params = ParamType::from_opcode_wide(wopcode);
                } else {
                    params = ParamType::from_opcode(opcode);
                }

                //fetching parameters
                let mut p0 = vec![];
                let mut p1 = None;
                for _ in 0..params.0.bytes_num {
                    let mut temp = 0;
                    if let Err(msg) = sub_fetch(self, &mut temp) {
                        //TODO: trigger error listener
                        eprintln!("{msg}");
                        return;
                    } else {
                        p0.push(temp);
                    }
                }
                if let Some(_) = params.1 {
                    if let Err(msg) = sub_fetch(self, p1.as_mut().unwrap()) {
                        //TODO: trigger error listener
                        eprintln!("{msg}");
                        return;
                    }
                }
                if let Some(v) = p1 {
                    p0.push(v);
                }
                //abbiamo i parametri easy e TECNICAMENTE il pc punta al prossimo opcode
                if opcode == Istruction::WIDE {
                    self.ir = Some(Istruction::from_wide_opcode(little, &p0));
                } else {
                    self.ir = Some(Istruction::from_opcode(opcode, &p0));
                }
            } else {
                //TODO: trigger error listener
                eprintln!("Error in fetching, opcode ({opcode}) non existing");
                return;
            }
        }
    } 
}

#[cfg(test)]
mod t_private_ijvm {
    use std::fs;
    use std::fs::File;

    use crate::ijvm::{IJVM, MethodArea};
    use crate::parser::{Istruction, IstructionReader};

    #[test]
    #[should_panic]
    fn t_fetch() {
        let f = File::create("target/test.txt").expect("Impossibile creare il file");
        let mut fake_method_area = MethodArea {
            istructions: vec![],
            reader: IstructionReader {
                exec: f,
                bytes: 6,
                read: 0,
            },
        };
        fake_method_area.istructions.push(Istruction::ILOAD);
        fake_method_area.istructions.push(0x10);
        fake_method_area.istructions.push(Istruction::WIDE);
        fake_method_area.istructions.push(Istruction::ILOAD);
        fake_method_area.istructions.push(0x1);
        fake_method_area.istructions.push(0x1);
        let cp = vec![];
        let mut ijvm = IJVM::new(fake_method_area, &cp);
        ijvm.fetch_decode();
        println!("{:?}", ijvm.ir.unwrap());
        ijvm.fetch_decode();
        println!("{:?}", ijvm.ir.unwrap());
        ijvm.fetch_decode();
        println!("{:?}", ijvm.ir.unwrap());
        fs::remove_file("target/test.txt").expect("Impossibile rimuovere il file");
    }
}
