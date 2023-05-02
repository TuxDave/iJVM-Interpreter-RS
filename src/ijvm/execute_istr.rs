use crate::ijvm::IJVM;
use crate::parser::Istruction;

impl<'a> IJVM<'a> {
    fn get_last_stack(&mut self) -> &mut Vec<i32> {
        return if self.stack.len() > 0 {
            self.stack.last_mut().unwrap()
        } else {
            self.stack.push(vec![]);
            self.get_last_stack()
        }
    }

    #[inline]
    fn jump(&mut self, offset: i16) {
        self.pc -= 3;
        if offset >= 0 {
            self.pc += offset as usize;
        } else {
            self.pc -= -offset as usize
        }
    }

    fn execute(&mut self) {
        let istr = if let Some(istr) = self.ir {
            self.ir = None;
            istr
        } else {
            return;
        };

        let mut is_error = false;
        let mut error = |msg: &str| {
            //TODO: trigger execution error
            is_error = true;
            eprintln!("{msg}");
        };

        match istr {
            Istruction::Bipush(ibyte) => {
                self.get_last_stack().push(ibyte as i32)
            }
            Istruction::Dup => {
                let tos = self.get_last_stack().last().cloned();
                if let Some(n) = tos {
                    self.get_last_stack().push(n);
                }
            }
            Istruction::Goto(offset) => {
                self.jump(offset);
            }
            Istruction::IAdd => {
                let last = self.get_last_stack();
                if last.len() >= 2 {
                    let res = last.pop().unwrap() + last.pop().unwrap();
                    last.push(res);
                } else {
                    error("IADD: Too less values in stack");
                }
            }
            Istruction::IAnd => {
                let last = self.get_last_stack();
                if last.len() >= 2 {
                    let res = last.pop().unwrap() & last.pop().unwrap();
                    last.push(res);
                } else {
                    error("IAND: Too less values in stack");
                }
            }
            Istruction::IfEq(offset) => {
                let last = self.get_last_stack();
                if let Some(value) = last.pop() {
                    if value == 0 {
                        self.jump(offset);
                    }
                } else {
                    error("IFEQ: Too less values in stack");
                }
            }
            Istruction::IfLt(offset) => {
                let last = self.get_last_stack();
                if let Some(value) = last.pop() {
                    if value < 0 {
                        self.jump(offset);
                    }
                } else {
                    error("IFLT: Too less values in stack");
                }
            }
            Istruction::IfICmpEq(offset) => {
                let last = self.get_last_stack();
                if last.len() >= 2 {
                    if last.pop().unwrap() == last.pop().unwrap() {
                        self.jump(offset);
                    }
                } else {
                    error("IF_ICMPEQ: Too less values in stack");
                }
            }
            //TODO: DO IINC
            _ => {}
        }
        self.error = is_error;
    }
}