// Verifies the inclusion of a single leaf.

// Struct representing a hash path in a Merkle tree
struct HashPath {
    path: [Field; 3],
    index: Field
}


fn main(
    root: Field,
    leaf: Field, 
    hash_path: HashPath,
) {
    let merkel_root = 
        std::merkle::compute_merkle_root(leaf, hash_path.index, hash_path.path);
      
// Ensures that the computed Merkle root matches the provided root.
    assert(root == merkel_root);
}
