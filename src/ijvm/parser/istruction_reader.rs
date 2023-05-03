use std::fs::File;
use std::io::Read;

pub use istruction::Istruction;
pub use istruction::ParamType;

pub struct IstructionReader {
    pub(crate) exec: File,
    pub(crate) bytes: u32,
    pub(crate) read: u32,
}

impl IstructionReader {
    pub fn new(mut exec: File) -> IstructionReader {
        let bytes = &mut [0; 4];
        exec.read(bytes); //.expect("Errore in lettura (lunghezza codice eseguibile)");
        let bytes = u32::from_be_bytes(*bytes);
        return IstructionReader {
            exec,
            bytes,
            read: 0,
        };
    }

    pub fn len(&self) -> usize {
        return self.bytes as usize;
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.read == self.bytes {
            return None;
        }
        let ret = &mut [0];
        self.exec.read(ret).expect("Errore in lettura");
        self.read += 1;
        return Some(ret[0]);
    }

    fn read_i8(&mut self) -> Option<i8> {
        let ret = self.read_u8();
        return if ret.is_none() {
            None
        } else {
            Some(ret.unwrap() as i8)
        };
    }

    fn read_u16(&mut self) -> Option<u16> {
        let big = self.read_u8();
        let little = self.read_u8();
        return if big.is_some() && little.is_some() {
            Some(u16::from_be_bytes([big.unwrap(), little.unwrap()]))
        } else {
            None
        };
    }

    fn read_i16(&mut self) -> Option<i16> {
        let ret = self.read_u16();
        return if ret.is_none() {
            None
        } else {
            Some(ret.unwrap() as i16)
        };
    }
}

mod istruction;
