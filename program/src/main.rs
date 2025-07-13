//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use eth_binary_tree::tree::{random_key, random_value, verify_proof, BinaryTree};

pub fn main() {
    let leaf_count = sp1_zkvm::io::read::<u32>();

    let mut tree = BinaryTree::new();
    let mut keys = vec![];
    let mut values = vec![];

    for n in 0..leaf_count {
        let key = random_key();
        keys.push(key);
        let value = random_value();
        values.push(value);
        tree.insert(key, value);
    }

    let root_hash = tree.merkelize();
    let mut all_proofs_verified = true;
    for n in 0..leaf_count {
        let key = keys.get(n as usize).unwrap();
        let proof = tree.get_proof(*key).unwrap();

        if !verify_proof(&proof, root_hash, *key) {
            all_proofs_verified = false;
        }
    }

    sp1_zkvm::io::commit(&all_proofs_verified);
}
