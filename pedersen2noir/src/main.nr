// Import the Pedersen hash function
use dep::std;

fn hash_and_print(input: [Field; 2]) {
    let result: Field = std::hash::pedersen_hash(input);
    println(result);
}

fn main(input1: Field, input2: Field) {
    let input: [Field; 2] = [input1, input2];

    // Return the hash as a public output
    hash_and_print(input);
}
