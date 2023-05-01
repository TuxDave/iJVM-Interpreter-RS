mod method_area;

pub use method_area::MethodArea;
use crate::parser::Istruction;

#[allow(non_camel_case_types)]
pub struct IJVM {
    pc: usize, // program counter
    ir: Option<Istruction>, // istruction "register"
    method_area: MethodArea,
}
impl IJVM {
    fn fetch (&mut self) {
        self.ir = if let Ok(istr) = self.method_area.fetch_absolute(self.pc) {
            self.pc += 1;
            //todo: fetch all the remaining fields, increment pc as needed and set the istruction
            Some()
        } else {
            //todo: trigger error listener
            println!("Error in fetch");
            None
        }
    }
}