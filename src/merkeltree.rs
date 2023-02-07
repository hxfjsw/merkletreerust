use crate::buffer::Buffer;

pub struct MerkelTree {
    pub sort: bool,
    pub leaves: Vec<Buffer>,
    pub layers: Vec<Buffer>,
}

impl MerkelTree {

    pub fn buffer_to_hex(value: Buffer) -> String
    {
        "".to_string()
    }

    pub fn new() -> Self {
        Self {
            sort: false,
            leaves: vec![],
            layers: vec![]
        }
    }
}
