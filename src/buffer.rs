use std::str::FromStr;
use crate::error::MerkleTreeError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Buffer {
    pub payload: Vec<u8>,
}

impl Buffer {
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }

    pub fn empty() -> Self {
        Self { payload: vec![] }
    }

    #[allow(dead_code)]
    pub fn length(&self) -> usize {
        self.payload.len()
    }

    pub fn to_hex(&self) -> String {
        "0x".to_owned() + &hex::encode(self.payload.clone())
    }

    pub fn concat(combined: Vec<Buffer>) -> Buffer {
        let mut buffer = Buffer::empty();
        for item in combined {
            buffer.payload.extend_from_slice(&item.payload);
        }
        buffer
    }

    pub fn append(&mut self, data :&[u8]) -> &mut Buffer {
        self.payload.extend_from_slice(data);
        self
    }
}

impl FromStr for Buffer {
    type Err = MerkleTreeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s;
        if ss.starts_with("0x") {
            ss = &s[2..];
        }

        let payload = hex::decode(ss);
        match payload {
            Ok(payload) => Ok(Buffer { payload }),
            Err(_e) => Err(MerkleTreeError::FromHexError)
        }
    }
}


impl Default for Buffer {
    fn default() -> Self {
        Self { payload: vec![] }
    }
}

#[test]
pub fn test_parse_buffer_from_str() {
    let a = "6dC0c0be4c8B2dFE750156dc7d59FaABFb5B9231";
    let b: Buffer = a.parse().unwrap();
    println!("{:?}", b);
    println!("{:?}", b.to_hex());
}