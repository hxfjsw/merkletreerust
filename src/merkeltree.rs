use crate::buffer::Buffer;
use crate::error::MerkelTreeError;
use crate::option::Options;
use crate::proof::{Proof, ProofPosition};

pub struct MerkelTree<F> {
    pub leaves: Vec<Buffer>,
    pub layers: Vec<Vec<Buffer>>,
    sort_leaves: bool,
    sort_pairs: bool,
    sort: bool,
    hash_leaves: bool,
    duplicate_odd: bool,
    hash_fn: F,
}

impl<F> MerkelTree<F> {
    pub fn new(leaves: Vec<Buffer>, hash_fn: F, options: Options) -> MerkelTree<F>
        where F: Fn(&[u8]) -> Vec<u8> {
        let mut this = Self {
            sort: false,
            leaves: vec![],
            layers: vec![],
            sort_leaves: false,
            sort_pairs: false,
            hash_leaves: false,
            duplicate_odd: false,
            hash_fn,
        };

        this.hash_leaves = options.hash_leaves;
        this.sort_leaves = options.sort_leaves;
        this.sort_pairs = options.sort_pairs;
        this.sort = options.sort;
        if this.sort {
            this.sort_leaves = true;
            this.sort_pairs = true;
        }
        this.duplicate_odd = options.duplicate_odd;
        this.process_leaves(leaves);

        this
    }

    pub fn get_root(&self) -> Result<&Buffer, MerkelTreeError> {
        if self.layers.len() == 0 {
            return Err(MerkelTreeError::NoRoot);
        }
        let i = self.layers.len() - 1;
        let a = self.layers.get(i).unwrap().get(0);
        if let Some(a) = a {
            Ok(a)
        } else {
            return Err(MerkelTreeError::NoRoot);
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

    pub fn get_proof(&self, _leaf: Buffer) -> Result<Vec<Proof>, MerkelTreeError>
        where F: Fn(&[u8]) -> Vec<u8> {
        if self.leaves.len() == 0 {
            return Err(MerkelTreeError::NoLeaf);
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


            proof.push(Proof {
                position: if is_right_node == 1 { ProofPosition::Left } else { ProofPosition::Right },
                data: layer.get(pair_index).unwrap().clone(),
            })
        }

        return Ok(proof);
    }

    pub fn get_hex_proof(&self, leaf: Buffer) -> Result<Vec<String>, MerkelTreeError>
        where F: Fn(&[u8]) -> Vec<u8> {
        let result: Vec<String> = self.get_proof(leaf)?.into_iter().map(|i| { i.data.to_hex() }).collect();
        Ok(result)
    }
}
