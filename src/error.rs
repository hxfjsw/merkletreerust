#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MerkleTreeError {
   NoRoot,
   NoLeaf,
   FromHexError
}
