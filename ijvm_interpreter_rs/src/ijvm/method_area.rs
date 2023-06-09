use crate::ijvm::parser::IstructionReader;

pub struct MethodArea {
    pub istructions: Vec<u8>,
    pub(crate) reader: IstructionReader,
}

impl MethodArea {
    pub fn new(reader: IstructionReader) -> MethodArea {
        return MethodArea {
            istructions: vec![],
            reader,
        };
    }

    pub fn fetch_absolute(&mut self, pc: usize) -> Result<u8, String> {
        return if pc <= self.reader.len() - 1 {
            let istr = self.istructions.get(pc);
            if let Some(istr) = istr {
                Ok(*istr)
            } else {
                let to_read = (pc as isize - (self.istructions.len() as isize - 1)) as usize;
                if to_read == 0 && pc != 0 {
                    return Err(format!("L'istruzione richiesta ({pc}) è out of bound ({})... \
                            (file compilato termimato)", self.reader.len()));
                }
                for _ in 0..to_read {
                    let istr = self.reader.read_u8();
                    if let Some(istr) = istr {
                        self.istructions.push(istr);
                    } else {
                        return Err(format!("L'istruzione richiesta ({pc}) è out of bound ({})... \
                            (file compilato non corretto)", self.reader.len()));
                    }
                }
                self.fetch_absolute(pc)
            }
        } else {
            Err(format!("L'istruzione richiesta ({pc}) è out of bound ({})... file compilato terminato", self.reader.len()))
        };
    }

    pub fn fetch_by_offset(&mut self, current: usize, offset: i16) -> Result<u8, String> {
        let pc = current as isize + offset as isize;
        if pc < 0 {
            return Err(format!("L'istruzione richiesta ({pc}) è out of bound ({})... \
                            (file compilato non corretto)", self.reader.len()));
        }
        return self.fetch_absolute(pc as usize);
    }
}