#[cfg(test)]
mod common_files {
    use std::error::Error;
    use std::fs::{File, read};
    use std::io::{BufReader, Read, Write};

    #[test]
    pub fn t1() {
        let file = File::create("target/file.txt");
        file.unwrap().write("abcdefghilmn".as_bytes()).unwrap();
        let mut file = File::open("target/file.txt").unwrap();
        // let reader = BufReader::with_capacity(1, file);
        let mut c = &mut [0; 4];
        file.read(c);
        file.read(c);
        println!("{:?}", c)
    }
}