use std::fmt::Debug;
use safer_ffi::vec::{Vec as VecC};
use safer_ffi::string::String as StringC;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct IJVM_MemoryState {
    pub pc: usize,
    pub stack: VecC<VecC<i32>>,
    pub constant_pool: VecC<i32>,
    pub local_variables: VecC<VecC<i32>>,
    pub istruction_register: StringC,
}
impl IJVM_MemoryState {
    pub fn from(
        from: (Vec<Vec<i32>>,Rc<Vec<i32>>,Vec<Vec<i32>>,Option<Istruction>,usize)
    ) -> IJVM_MemoryState {
        let (
            mut old_stack,
            mut old_constant_pool,
            mut old_local_variables,
            mut old_istruction_register,
            mut pc
        ) = from;

        return IJVM_MemoryState {
            stack: VecC::from(old_stack.iter().map( |el: &Vec<i32>| {
                let el = el.clone();
                VecC::from(el)
            }).collect::<Vec<VecC<i32>>>()),
            constant_pool: VecC::from((*old_constant_pool).clone()),
            local_variables: VecC::from(old_local_variables.iter().map( |el: &Vec<i32>| {
                let el = el.clone();
                VecC::from(el)
            }).collect::<Vec<VecC<i32>>>()),
            istruction_register: StringC::from(format!("{}", old_istruction_register.unwrap_or_else(|| Nop))),
            pc
        }
    }

    pub fn default_placeholder() -> IJVM_MemoryState {
        return Self::from((
            vec![],
            Rc::new(vec![]),
            vec![],
            None,
            0
        ));
    }
}

use std::fs::File;
use std::rc::Rc;
use std::string::ToString;
use ijvm_interpreter_rs::ijvm::{IJVM, Istruction};
use ijvm_interpreter_rs::ijvm::Istruction::Nop;

#[allow(non_upper_case_globals)]
static mut ijvm: Option<IJVM> = None;

#[no_mangle]
pub extern "C" fn ijvm_new(path: StringC) -> bool {
    let string = path;
    let f = File::open(string.to_string());
    let l_ijvm;
    if let Ok(file) = f {
        l_ijvm = IJVM::new(file);
    } else {
        eprintln!("File not found");
        return false;
    }
    if let Ok(l_ijvm) = l_ijvm {
        unsafe { ijvm = Some(l_ijvm); }
    } else {
        return false;
    };
    return true;
}

#[no_mangle]
pub extern "C" fn get_ijvm_memory_state() -> IJVM_MemoryState {
    unsafe {
        return if ijvm.is_some() {
            IJVM_MemoryState::from(ijvm.as_ref().unwrap().get_memory_state())
        } else {
            IJVM_MemoryState::default_placeholder()
        }
    }
}

#[no_mangle]
pub extern "C" fn step_run() -> bool{
    unsafe {
        return if let Some(u_ijvm) = &mut ijvm {
            u_ijvm.step_run().is_some()
        } else { false }
    }
}

#[no_mangle]
pub extern "C" fn auto_run() -> bool {
    unsafe{
        return if let Some(u_ijvm) = &mut ijvm {
            u_ijvm.auto_run();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_ijvm_memory_state, ijvm_new, step_run};
    use safer_ffi::vec::{Vec as VecC};
    use safer_ffi::string::String as StringC;

    #[test]
    fn t_dont_panic() {
        let result = ijvm_new(StringC::from("../ijvm_interpreter_rs/resources/esempioGOTO.ijvm".to_string()));
        assert_eq!(true, result);
        let mut state = get_ijvm_memory_state();
        assert_eq!(state.pc, 0);
        step_run();
        state = get_ijvm_memory_state();
        assert_eq!(state.pc, 5);
        assert_eq!(state.stack[0].len(), 1)
    }
}