// Import the Merkle root calculation function
use dep::std;

global MERKLE_DEPTH = 4;

// The main function of the Noir program
fn main(
    leaf_index1: Field, // Index of the leaf being updated
    leaf_index2: Field, // Index of the leaf being updated
    old_leaf1: Field, // Old leaf value before update
    new_leaf1: Field, // New leaf value after update
    old_leaf2: Field, // Old leaf value before update
    new_leaf2: Field, // New leaf value after update
    old_path1: [Field; MERKLE_DEPTH], // Sibling nodes along the Merkle path for the first leaf
    new_path1: [Field; MERKLE_DEPTH], // Sibling nodes along the Merkle path for the first leaf
    old_path2: [Field; MERKLE_DEPTH], // Sibling nodes along the Merkle path for the second leaf
    new_path2: [Field; MERKLE_DEPTH], // Sibling nodes along the Merkle path for the second leaf
    old_root: pub Field, // Old Merkle root (public input)
    intermediate_root: pub Field, // Intermediate Merkle root (public input)
    new_root: pub Field, // New Merkle root (public input)
) {
    // Sender's leaf belongs to the tree with the old root
    let old_hash1 = std::merkle::compute_merkle_root(old_leaf1, leaf_index1, old_path1);
    // Receiver's leaf belongs to the tree with the old root
    let old_hash2 = std::merkle::compute_merkle_root(old_leaf2, leaf_index2, old_path2);

    // Compute the new root after updating the sender's leaf
    let intermediate_hash = std::merkle::compute_merkle_root(new_leaf1, leaf_index1, new_path1);

    // Compute the new root after updating the receiver's leaf
    let new_hash = std::merkle::compute_merkle_root(new_leaf2, leaf_index2, new_path2);

    // Enforce that the computed old roots match the provided old roots
    assert(old_hash1 == old_root);
    assert(old_hash2 == old_root);

    // Enforce that the computed intermediate root matches the provided intermediate root
    assert(intermediate_hash == intermediate_root);

    // Enforce that the computed new root matches the provided new root
    assert(new_hash == new_root);
}
