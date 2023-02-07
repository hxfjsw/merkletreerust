use crate::buffer::Buffer;
use crate::error::MerkleTreeError;
use crate::option::Options;
use crate::proof::{Proof, ProofPosition};

pub struct MerkleTree<F> {
    leaves: Vec<Buffer>,
    layers: Vec<Vec<Buffer>>,
    sort_leaves: bool,
    sort_pairs: bool,
    sort: bool,
    hash_leaves: bool,
    duplicate_odd: bool,
    hash_fn: F,
}

impl<F> MerkleTree<F> {
    pub fn new(leaves: Vec<Buffer>, hash_fn: F, options: Options) -> MerkleTree<F>
        where F: Fn(&[u8]) -> Vec<u8> {
        let mut this = Self {
            sort: options.sort,
            leaves: vec![],
            layers: vec![],
            sort_leaves: options.sort_leaves,
            sort_pairs: options.sort_pairs,
            hash_leaves:  options.hash_leaves,
            duplicate_odd: options.duplicate_odd,
            hash_fn,
        };
        if this.sort {
            this.sort_leaves = true;
            this.sort_pairs = true;
        }
        this.process_leaves(leaves);
        this
    }

    pub fn get_root(&self) -> Result<&Buffer, MerkleTreeError> {
        if self.layers.len() == 0 {
            return Err(MerkleTreeError::NoRoot);
        }
        let i = self.layers.len() - 1;
        let a = self.layers.get(i).unwrap().get(0);
        if let Some(a) = a {
            Ok(a)
        } else {
            return Err(MerkleTreeError::NoRoot);
        }
    }

    pub fn process_leaves(&mut self, leaves: Vec<Buffer>)
        where F: Fn(&[u8]) -> Vec<u8> {
        let mut hashed_leaves = vec![];
        if self.hash_leaves {
            hashed_leaves = leaves.into_iter().map(|leave| {
                let hash = (self.hash_fn)(&leave.payload[..]);
                return Buffer::new(hash);
            }).collect();
        }

        self.leaves = hashed_leaves;
        if self.sort_leaves {
            self.leaves.sort();
        }
        self.layers.push(self.leaves.clone());
        self.create_hashes(self.leaves.clone());
    }

    fn create_hashes(&mut self, nodes: Vec<Buffer>)
        where F: Fn(&[u8]) -> Vec<u8> {
        let mut nodes = nodes;
        while nodes.len() > 1 {
            let layer_index = self.layers.len();
            self.layers.push(vec![]);
            let mut i = 0;
            while i < nodes.len() {
                if i + 1 == nodes.len() {
                    // let data = nodes.get(nodes.len() - 1).unwrap();
                    // self.layers.get(layer_index).unwrap().push(nodes.get(i).unwrap().clone());
                    let data = nodes.get(nodes.len() - 1).unwrap();
                    self.layers.push(vec![data.clone()]);
                }
                let left = nodes.get(i).unwrap();
                let right = if i + 1 == nodes.len() { left } else { nodes.get(i + 1).unwrap() };
                let mut combined = vec![left.clone(), right.clone()];
                if self.sort_pairs {
                    combined.sort();
                }
                let data = Buffer::concat(combined);
                let hash = (self.hash_fn)(&data.payload[..]);
                let a = self.layers.get_mut(layer_index).unwrap();
                a.push(Buffer::new(hash));

                i += 2;
            }
            nodes = self.layers.get(layer_index).unwrap().clone();
        }
    }

    pub fn get_proof(&self, _leaf: Buffer) -> Result<Vec<Proof>, MerkleTreeError>
        where F: Fn(&[u8]) -> Vec<u8> {
        if self.leaves.len() == 0 {
            return Err(MerkleTreeError::NoLeaf);
        }

        let mut leaf = Buffer::empty();
        if self.hash_leaves {
            let hash = (self.hash_fn)(&_leaf.payload[..]);
            leaf = Buffer::new(hash);
        }

        let mut proof: Vec<Proof> = vec![];

        let mut index: i64 = -1;
        for i in 0..self.leaves.len() {
            if leaf.payload == self.leaves.get(i).unwrap().payload {
                index = i as i64;
            }
        }

        if index == -1 {
            return Ok(proof);
        }

        for i in 0..self.layers.len() {
            let layer = self.layers.get(i).unwrap();
            let is_right_node = index % 2;
            let pair_index;
            if is_right_node == 1 {
                pair_index = (index - 1) as usize;
            } else {
                pair_index = (index + 1) as usize;
            }

            if pair_index < layer.len() {
                proof.push(Proof {
                    position: if is_right_node == 1 { ProofPosition::Left } else { ProofPosition::Right },
                    data: layer.get(pair_index).unwrap().clone(),
                });
            }

            // set index to parent index
            index = (index / 2) | 0;
        }

        return Ok(proof);
    }

    pub fn get_hex_proof(&self, leaf: Buffer) -> Result<Vec<String>, MerkleTreeError>
        where F: Fn(&[u8]) -> Vec<u8> {
        let result: Vec<String> = self.get_proof(leaf)?.into_iter().map(|i| { i.data.to_hex() }).collect();
        Ok(result)
    }


    pub fn sort_leaves(&self) -> bool {
        self.sort_leaves
    }
    pub fn sort_pairs(&self) -> bool {
        self.sort_pairs
    }
    pub fn sort(&self) -> bool {
        self.sort
    }
    pub fn hash_leaves(&self) -> bool {
        self.hash_leaves
    }
    pub fn duplicate_odd(&self) -> bool {
        self.duplicate_odd
    }
    pub fn hash_fn(&self) -> &F {
        &self.hash_fn
    }

    pub fn set_sort_leaves(&mut self, sort_leaves: bool) {
        self.sort_leaves = sort_leaves;
    }
    pub fn set_sort_pairs(&mut self, sort_pairs: bool) {
        self.sort_pairs = sort_pairs;
    }
    pub fn set_sort(&mut self, sort: bool) {
        self.sort = sort;
    }
    pub fn set_hash_leaves(&mut self, hash_leaves: bool) {
        self.hash_leaves = hash_leaves;
    }
    pub fn set_duplicate_odd(&mut self, duplicate_odd: bool) {
        self.duplicate_odd = duplicate_odd;
    }
    pub fn set_hash_fn(&mut self, hash_fn: F) {
        self.hash_fn = hash_fn;
    }

    pub fn leaves(&self) -> &Vec<Buffer> {
        &self.leaves
    }
    pub fn layers(&self) -> &Vec<Vec<Buffer>> {
        &self.layers
    }

    pub fn set_leaves(&mut self, leaves: Vec<Buffer>) {
        self.leaves = leaves;
    }
    pub fn set_layers(&mut self, layers: Vec<Vec<Buffer>>) {
        self.layers = layers;
    }
}
