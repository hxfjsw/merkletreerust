mod merkeltree;
mod buffer;
mod option;
mod error;
mod proof;


#[cfg(test)]
mod tests {
    use crate::buffer::Buffer;
    use crate::merkeltree::MerkelTree;
    use crate::option::Options;
    use web3::signing::keccak256;
    use crate::error::MerkelTreeError;

    #[test]
    fn it_works() ->Result<(),MerkelTreeError>{
        let whitelist_address: Vec<Buffer> = vec![
            "0x6dC0c0be4c8B2dFE750156dc7d59FaABFb5B923D".parse::<Buffer>()?,
            "0xa8d17cc9caf29af964d19267ddeb4dff122697b0".parse::<Buffer>()?
        ];

        let options = Options { duplicate_odd: false, sort_pairs: true, sort_leaves: true, sort: true, hash_leaves: true };
        let hash_fn = |buf: &[u8]| { keccak256(buf).to_vec() };
        let merkle_tree = MerkelTree::new(whitelist_address, hash_fn, options);

        let root = merkle_tree.get_root()?.to_hex();
        let leaf = "0x6dC0c0be4c8B2dFE750156dc7d59FaABFb5B923D";
        let proof = merkle_tree.get_hex_proof(leaf.parse::<Buffer>()?)?;

        println!("root: {:?}", root);
        println!("leaf: {:?}", leaf);
        println!("proof: {:?}", proof);

        Ok(())
    }
}
