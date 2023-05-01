mod method_area;

pub use method_area::MethodArea;

#[allow(non_camel_case_types)]
pub struct IJVM {
    pc: usize // program counter
}