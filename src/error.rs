#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MerkelTreeError {
   NoRoot,
   NoLeaf,
   FromHexError
}
