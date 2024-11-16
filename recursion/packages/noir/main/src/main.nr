// Import the Merkle root calculation function
use dep::std;

global MERKLE_DEPTH = 4;

// The main function of the Noir program
#[recursive]
fn main(
    leaf_index1: Field, // Index of the leaf being updated
    new_leaf1: Field, // New leaf value after update
    new_path1: [Field; MERKLE_DEPTH], // Sibling nodes along the Merkle path for the first leaf
    intermediate_root: pub Field, // Intermediate Merkle root (public input)
) {
    // Compute the new root after updating the sender's leaf
    let intermediate_hash = std::merkle::compute_merkle_root(new_leaf1, leaf_index1, new_path1);

    // Enforce that the computed intermediate root matches the provided intermediate root
    assert(intermediate_hash == intermediate_root);
}
