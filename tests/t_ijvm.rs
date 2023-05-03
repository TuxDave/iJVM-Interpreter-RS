#[cfg(test)]
mod t_ijvm{
    use std::fs::File;
    use ijvm_interpreter_rs::ijvm::IJVM;

    #[test]
    fn t_run1() {
        let mut ijvm = IJVM::new(File::open("resources/esempioMetodo.ijvm").unwrap()).expect("ERRORE");
        ijvm.run();
    }

    #[test]
    fn t_run2() {
        let mut ijvm = IJVM::new(File::open("resources/esempioGOTO.ijvm").expect("ERRORE PARSE")).expect("ERRORE");
        ijvm.run();
    }
}