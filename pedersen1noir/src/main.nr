// Import the Pedersen hash function
use dep::std;

fn hash_and_print(input: [Field; 1]) {
    let result: Field = std::hash::pedersen_hash(input);
    println(result);
}

fn main(input1: Field) {
    let input: [Field; 1] = [input1];

    // Return the hash as a public output
    hash_and_print(input);
}
