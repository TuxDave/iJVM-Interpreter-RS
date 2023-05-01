use std::fs::File;
use std::io::Read;
use std::ptr::read;

pub struct IstructionReader {
    exec: File,
    bytes: u32,
    read: u32
}
impl IstructionReader {
    pub fn new(mut exec: File) -> IstructionReader {
        let bytes = &mut [0; 4];
        exec.read(bytes).expect("Errore in lettura (lunghezza codice eseguibile)");
        let bytes = u32::from_be_bytes(*bytes);
        return IstructionReader{
            exec,
            bytes,
            read: 0
        };
    }

    fn read_ubyte(&mut self) -> Option<u8> {
        if self.read == self.bytes {
            return None
        }
        let ret = &mut [0];
        self.exec.read(ret).expect("Errore in lettura");
        self.read += 1;
        return Some(ret[0]);
    }

    fn read_ibyte(&mut self) -> Option<i8> {
        let ret = self.read_ubyte();
        return if ret.is_none() {
            None
        } else {
            Some(ret.unwrap() as i8)
        }
    }
}