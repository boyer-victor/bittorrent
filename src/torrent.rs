#![allow(dead_code)]

#[derive(Debug)]
pub struct Torrent {
    pub(crate) announce: String,
    pub(crate) info: Info,
}

impl Torrent {
    fn new() -> Torrent {
        return Torrent{ announce: "".to_string(), info: Info::new() };
    }

    pub fn length(self) -> u64 {
        return self.info.length;
    }

    pub fn from_file(file_name: &str) -> Torrent {
        todo!("parse from files {}", file_name)
        // let encoded_value = fs::read(file_name);

    }
}

#[derive(Debug)]
struct Info {
    piece_length: Vec<u8>,
    pieces: Vec<u8>,
    name: Vec<u8>,
    pub(crate) length: u64,
}

impl Info {
    fn new() -> Info {
        return Info {
            piece_length: vec![],
            pieces: vec![],
            name: vec![],
            length: 0,
        }
    }
}

pub fn read_file(file_name: &str) -> Torrent {
todo!("{}", file_name);
}