use std::ffi::{c_char, CStr};
use std::fs::File;
use std::ops::Index;
use std::rc::Rc;
use std::string::ToString;
use ijvm_interpreter_rs::ijvm::{IJVM, Istruction};
use ijvm_interpreter_rs::ijvm::Istruction::Nop;

#[allow(non_upper_case_globals)]
static mut ijvm: Option<IJVM> = None;

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
pub extern "C" fn get_pc() -> usize {
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_pc()
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn get_stacks_num() -> usize {
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_stack().len()
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn get_stack_size(i_stack: usize) -> usize{
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_stack().get(i_stack).unwrap_or(&vec![]).len()
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn get_stack_value(i_stack: usize, i_pos: usize) -> i32 {
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_stack()[i_stack][i_pos]
        } else { i32::MIN }
    }
}

#[no_mangle]
pub extern "C" fn get_constant_pool_size() -> usize {
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_constant_pool().len()
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn get_constant(i: usize) -> i32 {
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_constant_pool()[i]
        } else { i32::MIN }
    }
}

#[no_mangle]
pub extern "C" fn get_lvs_num() -> usize {
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_local_variables().len()
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn get_lv_size(i_lv: usize) -> usize{
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_local_variables().get(i_lv).unwrap_or(&vec![]).len()
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn get_lv_value(i_lv: usize, i_pos: usize) -> i32 {
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_local_variables()[i_lv][i_pos]
        } else { i32::MIN }
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

#[no_mangle]
pub extern "C" fn get_method_area_size() -> usize {
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_method_area().len()
        } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn get_method(i: usize) -> u8 {
    unsafe {
        if let Some(uijvm) = &ijvm {
            uijvm.get_method_area()[i]
        } else { -1 as i8 as u8 }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;
    use crate::{auto_run, get_constant, get_lv_value, get_lvs_num, get_pc, ijvm_new, step_run};

    #[test]
    fn t_dont_panic() {
        let result = ijvm_new("../ijvm_interpreter_rs/resources/esempioMetodo.ijvm\0".as_ptr() as *mut c_char);
        assert_eq!(true, result);
        assert_eq!(get_pc(), 0);
        auto_run();
        assert_eq!(get_pc(), 13);
        assert_eq!(get_constant(0), 64 as i32);
        assert_eq!(get_lvs_num(), 1);
        assert_eq!(get_lv_value(0,0), 1);
    }
}