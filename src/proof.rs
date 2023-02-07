use crate::buffer::Buffer;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProofPosition {
    Right,
    Left,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Proof {
    pub position: ProofPosition,
    pub data: Buffer,
}