use std::str::FromStr;
use crate::error::MerkelTreeError;

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

    #[allow(dead_code)]
    pub fn concat(combined: Vec<Buffer>) -> Buffer {
        let mut payload = vec![];
        for c in combined.iter() {
            payload.extend(c.payload.iter());
        }
        Buffer { payload }
    }
}

impl FromStr for Buffer {
    type Err = MerkelTreeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s;
        if ss.starts_with("0x") {
            ss = &s[2..];
        }

        let payload = hex::decode(ss);
        match payload {
            Ok(payload) => Ok(Buffer { payload }),
            Err(_e) => Err(MerkelTreeError::FromHexError)
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