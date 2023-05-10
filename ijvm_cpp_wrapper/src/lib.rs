// use safer_ffi::vec::{Vec as VecC};
use std::ffi::{c_char, CStr, CString};

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct IJVM_MemoryState {
    pub pc: usize,
    pub stack: *const *const i32,
    pub constant_pool: *const i32,
    pub local_variables: *const *const i32,
    pub istruction_register: *mut c_char,
}
impl IJVM_MemoryState {
    pub fn from(
        from: (Vec<Vec<i32>>,Rc<Vec<i32>>,Vec<Vec<i32>>,Option<Istruction>,usize)
    ) -> IJVM_MemoryState {
        let (
            stack,
            constant_pool,
            local_variables,
            istruction_register,
            pc
        ) = from;

        IJVM_MemoryState {
            stack: stack.iter().map(|el| el.as_ptr()).collect::<Vec<*const i32>>().as_ptr(),
            constant_pool: constant_pool.as_ptr(),
            local_variables: local_variables.iter().map(|el| el.as_ptr()).collect::<Vec<*const i32>>().as_ptr(),
            // stack: VecC::from(old_stack.iter().map(|el: &Vec<i32>| {
            //     let el = el.clone();
            //     VecC::from(el)
            // }).collect::<Vec<VecC<i32>>>()),
            // constant_pool: VecC::from((*old_constant_pool).clone()),
            // local_variables: VecC::from(old_local_variables.iter().map(|el: &Vec<i32>| {
            //     let el = el.clone();
            //     VecC::from(el)
            // }).collect::<Vec<VecC<i32>>>()),

            istruction_register: istruction_register.unwrap_or(Nop).to_string().as_ptr() as *mut c_char,
            pc
        }
    }

    pub fn default_placeholder() -> IJVM_MemoryState {
        Self::from((
            vec![],
            Rc::new(vec![]),
            vec![],
            None,
            0
        ))
    }
}

use std::fs::File;
use std::rc::Rc;
use std::string::ToString;
use ijvm_interpreter_rs::ijvm::{IJVM, Istruction};
use ijvm_interpreter_rs::ijvm::Istruction::Nop;

#[allow(non_upper_case_globals)]
static mut ijvm: Option<IJVM> = None;
static mut current_state: IJVM_MemoryState; //TODO risolvi che rust dealloca dopo che si esce e quindi c rimane con puntatori che puntano al nulla

#[no_mangle]
pub extern "C" fn ijvm_new(path: *mut c_char) -> bool {
    let f = File::open(unsafe { CStr::from_ptr(path.cast_const()) }.to_str().unwrap());
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
pub extern "C" fn get_ijvm_memory_state<'a>() -> IJVM_MemoryState {
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
        if let Some(u_ijvm) = &mut ijvm {
            u_ijvm.step_run().is_some()
        } else { false }
    }
}

#[no_mangle]
pub extern "C" fn auto_run() -> bool {
    unsafe{
        if let Some(u_ijvm) = &mut ijvm {
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
    use std::ffi::{c_char, CString as StringC};

    #[test]
    fn t_dont_panic() {
        let result = ijvm_new("../ijvm_interpreter_rs/resources/esempioGOTO.ijvm\0".as_ptr() as *mut c_char);
        assert_eq!(true, result);
        let mut state = get_ijvm_memory_state();
        assert_eq!(state.pc, 0);
        step_run();
        state = get_ijvm_memory_state();
        assert_eq!(state.pc, 5);
    }
}