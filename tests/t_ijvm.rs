#[cfg(test)]
mod t_ijvm{
    use std::fs::File;
    use ijvm_interpreter_rs::ijvm::IJVM;

    #[test]
    fn t_run1() {
        let mut ijvm = IJVM::new(File::open("resources/esempioMetodo.ijvm").unwrap()).expect("ERRORE");
        ijvm.auto_run();
    }

    #[test]
    fn t_run2() {
        let mut ijvm = IJVM::new(File::open("resources/esempioGOTO.ijvm").expect("ERRORE PARSE")).expect("ERRORE");
        ijvm.auto_run();
    }

    #[test]
    fn t_step1() {
        let mut ijvm = IJVM::new(File::open("resources/esempioGOTO.ijvm").expect("ERRORE PARSE")).expect("ERRORE");
        let mut res = ijvm.step_run();
        while res.is_some() {
            res = ijvm.step_run();
        }
    }
}