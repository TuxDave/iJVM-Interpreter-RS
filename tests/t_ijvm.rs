#[cfg(test)]
mod t_ijvm{
    use std::fs::File;
    use ijvm_interpreter_rs::ijvm::{IJVM, MethodArea};
    use ijvm_interpreter_rs::parser::Parser;

    #[test]
    fn t_run1() {
        let p = Parser::new(File::open("resources/esempioMetodo.ijvm").unwrap()).unwrap();
        let (cp, reader) = p.parse();
        let mut ijvm = IJVM::new(MethodArea::new(reader), &cp);
        ijvm.run();
    }

    #[test]
    fn t_run2() {
        let p = Parser::new(File::open("resources/esempioGOTO.ijvm").unwrap()).unwrap();
        let (cp, reader) = p.parse();
        let mut ijvm = IJVM::new(MethodArea::new(reader), &cp);
        ijvm.run();
    }
}