use std::fs::File;
use std::rc::Rc;
use method_area::MethodArea;
pub use parser::{Istruction, ParamType};
use crate::ijvm::parser::Parser;

mod method_area;
mod execute_istr;
mod parser;

#[allow(non_camel_case_types)]
pub struct IJVM {
    pc: usize,
    // program counter
    ir: Option<Istruction>,
    // istruction "register"
    method_area: MethodArea,
    constant_pool: Rc<Vec<i32>>,
    local_variables: Vec<Vec<i32>>,
    stack: Vec<Vec<i32>>,
    pub error: bool
}

impl IJVM {

    pub fn new( exec: File) -> Result<IJVM, &'static str> {
        let parser = Parser::new(exec);
        return if let Ok(parser) = parser {
            let (cp, reader) = parser.parse();
            let cp: Rc<Vec<i32>> = Rc::new(cp);
            let ma = MethodArea::new(reader);
            Ok(IJVM {
                pc: 0,
                ir: None,
                method_area: ma,
                constant_pool: Rc::clone(&cp),
                local_variables: vec![vec![]],
                stack: vec![vec![]],
                error: false
            })
        } else {
            Err(parser.err().as_ref().unwrap())
        }
        // return IJVM {
        //     pc: 0,
        //     ir: None,
        //     method_area,
        //     constant_pool,
        //     local_variables: vec![vec![]],
        //     stack: vec![vec![]],
        //     error: false
        // };
    }

    pub fn get_pc(&self) -> usize {
        return self.pc;
    }

    fn get_stack(&self) -> Vec<Vec<i32>>{
        return self.stack.clone();
    }

    fn get_constant_pool(&self) -> Rc<Vec<i32>> {
        return Rc::clone(&self.constant_pool);
    }

    fn get_local_variables(&self) -> Vec<Vec<i32>>{
        return self.local_variables.clone();
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

        self.ir = None;

        let mut opcode: u8 = 0;
        let mut little: u8 = 0;
        if let Err(msg) = sub_fetch(self, &mut opcode) {
            //TODO: trigger error listener
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
                    let tmp = &mut 0;
                    if let Err(msg) = sub_fetch(self, tmp) {
                        //TODO: trigger error listener
                        eprintln!("{msg}");
                        return;
                    } else {
                        p1 = Some(*tmp)
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

    ///## return: (stack, constant_pool, local_variables, istruction_register, program_counter)'s copy
    pub fn get_memory_state(&self) -> (Vec<Vec<i32>>, Rc<Vec<i32>>, Vec<Vec<i32>>, Option<Istruction>, usize) {
        return (self.get_stack(), self.get_constant_pool(), self.get_local_variables(), self.ir.clone(), self.get_pc());
    }

    pub fn step_run(&mut self) -> Option<()> {
        if self.ir.is_none() {
            self.fetch_decode();
        }
        return if let Some(_istr) = self.ir {
            self.execute();
            self.fetch_decode();
            Some(())
        } else {
            None
        }
    }

    pub fn auto_run(&mut self) {
        while !self.error {
            self.fetch_decode();
            if let Some(_istr) = self.ir {
                self.execute();
            } else {
                break;
            }
        }
    }
}
