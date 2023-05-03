use crate::ijvm::IJVM;
use crate::parser::Istruction;

impl<'a> IJVM<'a> {
    fn get_last_stack(&mut self) -> &mut Vec<i32> {
        return if self.stack.len() > 0 {
            self.stack.last_mut().unwrap()
        } else {
            self.stack.push(vec![]);
            self.get_last_stack()
        };
    }

    fn wide_get_var_value(&mut self, varnum: u16) -> i32 {
        let last = self.local_variables.last_mut().unwrap();
        let value = last.get(varnum as usize);
        if let Some(value) = value {
            return *value;
        } else {
            while last.len() as isize - 1 != varnum as isize {
                last.push(0);
            }
            return self.wide_get_var_value(varnum);
        }
    }

    fn get_var_value(&mut self, varnum: u8) -> i32 {
        return self.wide_get_var_value(varnum as u16);
    }

    fn wide_store_var_value(&mut self, varnum: u16, new_value: i32) {
        let last = self.local_variables.last_mut().unwrap();
        let value = last.get_mut(varnum as usize);
        if let Some(value) = value {
            *value = new_value;
        } else {
            while last.len() as isize - 1 != varnum as isize {
                last.push(0);
            }
            self.wide_store_var_value(varnum, new_value);
        }
    }

    fn store_var_value(&mut self, varnum: u8, new_value: i32) {
        self.wide_store_var_value(varnum as u16, new_value);
    }

    #[inline]
    fn jump(&mut self, offset: i16) {
        self.pc -= 3;
        if offset >= 0 {
            self.pc += offset as usize;
        } else {
            self.pc -= (-offset) as usize
        }
    }

    pub(super) fn execute(&mut self) {
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
            Istruction::IInc(varnum, value) => {
                let mut val = self.get_var_value(varnum);
                val += value as i32;
                self.store_var_value(varnum, val);
            }
            Istruction::ILoad(varnum) => {
                let val = self.get_var_value(varnum);
                self.get_last_stack().push(val);
            }
            Istruction::InvokeVirtual(disp) => {
                let caller_pc = self.pc; //save the caller's next istr pc
                //aggiunge un layer di variabili, di stack e salta al valore della costante che punta
                //partendo dalla prima istruzione
                if disp as usize >= self.constant_pool.len() {
                    error("INVOKEVIRTUAL: disp out of bound");
                } else {
                    let jump = self.constant_pool[disp as usize];
                    if jump as u32 + 2 >= self.method_area.reader.bytes {
                        error("INVOKEVIRTUAL: point to an out of bound istr");
                    } else {
                        self.pc = jump as usize;
                    }
                }

                let params_count = u16::from_be_bytes([
                    self.method_area.fetch_absolute(self.pc).unwrap(),
                    self.method_area.fetch_absolute(self.pc + 1).unwrap()]
                );
                if params_count as usize > self.get_last_stack().len() {
                    error("INVOKEVIRTUAL: Not enougth param on stack")
                } else {
                    self.pc += 4; // vado alla prima istruzione del metodo
                    let mut params = vec![caller_pc as i32]; //OBJREF (pc chiamante, gia pronto alla next op)
                    {
                        let mut to_rev = vec![];
                        for i in (0..params_count).rev() {
                            if i == 0 {
                                self.get_last_stack().pop(); //rimuovo OBJREF dallo stack corrente
                            } else {
                                //questo unwrap è "safe" perchè abbiamo già controllato che ci siano abbastanza parametri da POPare
                                to_rev.push(self.get_last_stack().pop().unwrap())
                            }
                        }
                        to_rev.reverse();
                        params.append(&mut to_rev);
                    }

                    self.stack.push(vec![]);
                    self.local_variables.push(params);
                }
            }
            Istruction::IOr => {
                let last = self.get_last_stack();
                if last.len() >= 2 {
                    let res = last.pop().unwrap() | last.pop().unwrap();
                    last.push(res);
                } else {
                    error("IRO: Too less values in stack.");
                }
            }
            Istruction::IReturn => {
                if self.stack.len() >= 2 {
                    let ret = self.get_last_stack().pop();
                    if let Some(value) = ret {
                        self.stack.pop();
                        self.pc = self.local_variables.pop().unwrap()[0] as usize; //torno al chiamante (next istr)
                        self.get_last_stack().push(value);
                    } else {
                        error("IRETURN: Need 1 value to be returned.")
                    }
                } else {
                    error("IRETURN: Can only be called in a method.")
                }
            }
            Istruction::IStore(varnum) => {
                let value = self.get_last_stack().pop();
                if let Some(value) = value {
                    self.store_var_value(varnum, value);
                } else {
                    error("ISTORE: Needs at least 1 value on the stack.")
                }
            }
            Istruction::ISub => {
                let last = self.get_last_stack();
                if last.len() >= 2 {
                    let res = last.pop().unwrap() - last.pop().unwrap();
                    last.push(res);
                } else {
                    error("ISUB: Too less values in stack");
                }
            }
            Istruction::Ldc_W(index) => {
                if index > self.constant_pool.len() as u16 {
                    error("LDC_W: Constant index out of bound.");
                } else {
                    let &cp = &self.constant_pool;
                    self.get_last_stack().push(cp[index as usize]);
                }
            }
            Istruction::Nop => {}
            Istruction::Pop => {
                self.get_last_stack().pop();
            }
            Istruction::Swap => {
                let last = self.get_last_stack();
                if last.len() < 2 {
                    error("SWAP: Needs at least 2 values on current stack.");
                } else {
                    let x = last.pop().unwrap();
                    let y = last.pop().unwrap();
                    last.push(x);
                    last.push(y);
                }
            }
            Istruction::Wide_IInc(varnum, inc) => {
                let mut val = self.wide_get_var_value(varnum);
                val += inc as i32;
                self.wide_store_var_value(varnum, val);
            }
            Istruction::Wide_IStore(varnum) => {
                let value = self.get_last_stack().pop();
                if let Some(value) = value {
                    self.wide_store_var_value(varnum, value);
                } else {
                    error("WIDE ISTORE: Needs at least 1 value on the stack.")
                }
            }
            Istruction::Wide_ILoad(varnum) => {
                let val = self.wide_get_var_value(varnum);
                self.get_last_stack().push(val);
            }
            Istruction::Halt => {
                is_error = true; //terminate
            }
        }
        self.error = is_error;
    }
}