use std::fs::File;
use std::io::Read;

pub use istruction_reader::Istruction;
pub use istruction_reader::IstructionReader;
pub use istruction_reader::ParamType;

mod istruction_reader;

#[allow(dead_code)]
const MAGIC: [u8; 4] = [0x1D, 0xEA, 0xDF, 0xAD];

pub struct Parser {
    exec: File,
    valid: Option<bool>,
    constants: Option<Vec<i32>>,
}

impl Parser {
    pub fn new(exec: File) -> Result<Parser, &'static str> {
        let mut p = Parser {
            exec,
            valid: None,
            constants: None,
        };
        return if p.check_valid() {
            p.exec.read(&mut [0; 4]).expect("Errore nella lettura del file al byte 5-8"); //consume i 4 bytes da ignorare
            Ok(p)
        } else {
            Err("File non nel formato eseguibile iJVM.")
        };
    }

    /**
    controlla che il file sia valido, consuma i primi 8 bytes del file
     */
    fn check_valid(&mut self) -> bool {
        return if self.valid.is_none() {
            let buff = &mut [0; 4];
            let r = self.exec.read(buff);
            if r.is_err() {
                self.valid = Some(false);
            }
            self.valid = Some(*buff == MAGIC);
            self.valid.unwrap()
        } else {
            self.valid.unwrap()
        };
    }

    fn pick_constants(&mut self) {
        if self.constants.is_none() {
            let const_count_buf = &mut [0; 4];
            self.exec.read(const_count_buf).expect("Errore in lettura al byte 9-12");
            let const_count = i32::from_be_bytes(*const_count_buf) / 4;
            let mut consts: Vec<i32> = vec![];
            let mut red_byte = 13;
            for _ in 0..const_count {
                let buff = &mut [0; 4];
                self.exec.read(buff).expect(&*format!("Errore in lettura al byte {red_byte}"));
                red_byte += 4;
                consts.push(i32::from_be_bytes(*buff));
            }
            self.constants = Some(consts);
            self.exec.read(&mut [0; 4]).expect(&*format!("Errore in lettura al byte {red_byte}")); //eat the 4 useless zeros
        }
    }

    pub fn parse(mut self) -> (Vec<i32>, IstructionReader) {
        self.pick_constants();
        return (self.constants.unwrap(), IstructionReader::new(self.exec));
    }
}