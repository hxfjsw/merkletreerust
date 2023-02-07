<h3 align="center">
  <br />
  <img src="https://user-images.githubusercontent.com/168240/83951171-85f48c80-a7e4-11ea-896e-529c28ffa18e.png" alt="merkletree.js logo" width="600" />
  <br />
  <br />
  <br />
</h3>

# MerkleTreeRust

Install 

```bash
# Cargo.toml

[dependencies]
merkletreerust = "*"
```

## Getting started

Construct tree, generate proof

```rust
use crate::buffer::Buffer;
use crate::merkeltree::MerkelTree;
use crate::option::Options;
use tiny_keccak::{Hasher, Keccak};

let whitelist_address: Vec<Buffer> = vec![
    "0x6dC0c0be4c8B2dFE750156dc7d59FaABFb5B923D".parse::<Buffer>()?,
    "0xa8d17cc9caf29af964d19267ddeb4dff122697b0".parse::<Buffer>()?
];

let options = Options { duplicate_odd: false, sort_pairs: true, sort_leaves: true, sort: true, hash_leaves: true };
let hash_fn = |buf: &[u8]| {
    let mut k256 = Keccak::v256();
    let mut result = [0; 32];
    k256.update(buf);
    k256.finalize(&mut result);
    result.to_vec()
};
let merkle_tree = MerkelTree::new(whitelist_address, hash_fn, options);

let root = merkle_tree.get_root()?.to_hex();
let leaf = "0x6dC0c0be4c8B2dFE750156dc7d59FaABFb5B923D";
let proof = merkle_tree.get_hex_proof(leaf.parse::<Buffer>()?)?;

println!("root: {:?}", root);
println!("leaf: {:?}", leaf);
println!("proof: {:?}", proof);
```


Output:

```bash
root:0xdb44a1f32851683f64d15a563ecd3686b67de2075821b6196dbaf7d25604592f
leaf:0x6dC0c0be4c8B2dFE750156dc7d59FaABFb5B923D
proof:["0x7fa4f9a213fc25511745e0fe7627ab0d7145664238bd854fb781559c2ddbf9c4"]
```


## Diagrams

▾ Visualization of Merkle Tree

<img src="https://user-images.githubusercontent.com/168240/43616375-15330c32-9671-11e8-9057-6e61c312c856.png" alt="Merkle Tree" width="500">

▾ Visualization of Merkle Tree Proof

<img src="https://user-images.githubusercontent.com/168240/43616387-27ec860a-9671-11e8-9f3f-0b871a6581a6.png" alt="Merkle Tree Proof" width="420">

▾ Visualization of Invalid Merkle Tree Proofs

<img src="https://user-images.githubusercontent.com/168240/43616398-33e20584-9671-11e8-9f62-9f48ce412898.png" alt="Merkle Tree Proof" width="420">

▾ Visualization of Bitcoin Merkle Tree

<img src="https://user-images.githubusercontent.com/168240/43616417-46d3293e-9671-11e8-81c3-8cdf7f8ddd77.png" alt="Merkle Tree Proof" width="420">


