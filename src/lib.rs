extern crate hex;
extern crate tiny_keccak;

pub mod merkletree;
pub mod buffer;
pub mod option;
pub mod error;
pub mod proof;


#[cfg(test)]
mod tests {
    use tiny_keccak::{Hasher, Keccak};
    use crate::buffer::Buffer;
    use crate::merkletree::MerkleTree;
    use crate::option::Options;
    use crate::error::MerkleTreeError;

    #[test]
    fn it_works() -> Result<(), MerkleTreeError> {
        let whitelist_address = vec![
            "0x6dC0c0be4c8B2dFE750156dc7d59FaABFb5B923D".parse::<Buffer>().unwrap(),
            "0xa8d17cc9caf29af964d19267ddeb4dff122697b0".parse::<Buffer>().unwrap(),
            "0x5b38da6a701c568545dcfcb03fcb875f56beddc465794a70626949364d43776962335630496a6f7766513d3d".parse::<Buffer>().unwrap(),
        ];

        let options = Options { duplicate_odd: false, sort_pairs: true, sort_leaves: true, sort: true, hash_leaves: true };
        let hash_fn = |buf: &[u8]| {
            let mut k256 = Keccak::v256();
            let mut result = [0; 32];
            k256.update(buf);
            k256.finalize(&mut result);
            result.to_vec()
        };
        let merkle_tree = MerkleTree::new(whitelist_address, hash_fn, options);

        let root = merkle_tree.get_root()?.to_hex();
        let leaf = "0x5b38da6a701c568545dcfcb03fcb875f56beddc465794a70626949364d43776962335630496a6f7766513d3d";
        let proof = merkle_tree.get_hex_proof(leaf.parse::<Buffer>()?)?;

        println!("root: {:?}", root);
        println!("leaf: {:?}", leaf);
        println!("proof: {:?}", proof);

        Ok(())
    }
}
