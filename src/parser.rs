use std::fs::File;
use std::io::Read;

const MAGIC: [u8; 4] = [0x1D, 0xEA, 0xDF, 0xAD];

struct Parser {
    exec: File,
    valid: Option<bool>
}

impl Parser {

    pub fn new(exec: File) -> Result<Parser, &'static str> {
        let mut p = Parser{
            exec,
            valid: None,
        };
        return if p.is_valid() {
            p.exec.read(&mut [0; 4]).expect("Errore nella lettura del file al byte 5-8"); //consume i 4 bytes da ignorare
            Ok(p)
        } else {
            Err("File non nel formato eseguibile iJVM.")
        }
    }

    /**
    controlla che il file sia valido, consuma i primi 8 bytes del file
     */
    fn is_valid(&mut self) -> bool {
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
        }
    }

    pub fn get_constants(&mut self) -> Vec<u32> {

        fn arr4_to_i32(arr: [u8; 4]) -> i32 {
            let mut c: i32 = 0;
            for i in (0 ..= 3).rev(){
                c += (arr[3 - i] as i32) << i * 8;
            }
            return c;
        }

        let const_count_buf = &mut [0; 4];
        self.exec.read(const_count_buf).expect("Errore in lettura al byte 9-12");
        let const_count = arr4_to_i32(*const_count_buf);
        let mut consts: Vec<i32> = vec![];
        for i in 0 .. const_count {
            let buff = &mut [0; 4];
            self.exec.read(buff).expect(""); //TODO caccia dentro le costanti e capisci che tipo fare tutto
            consts.push(arr4_to_i32(*buff));
        }
        return ();
    }
}