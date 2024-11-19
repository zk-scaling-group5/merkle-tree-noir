# Merkle Tree Transaction Simulator

This project simulates a single transaction on a Merkle tree. The transaction involves a sender represented by the Merkle tree leaf at index 2 and a receiver at leaf index 10. It showcases how to use Pedersen hashing in Noir circuits to construct a Merkle tree and validate a transaction using zero-knowledge proofs.

## Project Overview

The code generates a Merkle tree with 16 leaves, where each leaf is a Pedersen hash of the integers 1 through 16. Noir's Pedersen hashing functions are used to compute the hashes and construct the Merkle tree, enabling a proof of transaction that adheres to zk-SNARK principles.

### Key Components

- **Merkle Tree Construction**: Located in the `merkletreerust` directory, this component builds the Merkle tree from the hashed values of each leaf.
- **Pedersen Hash Functions**:
  - `pedersen1noir`: Generates a Pedersen hash of a single field element.
  - `pedersen2noir`: Generates a Pedersen hash of two field elements, used for hashing internal nodes.
- **Proof Circuit**: The circuit for proving the transaction is defined in the `merkletreenoir` directory.

## Prerequisites

Ensure the following tools are installed on your system:

- [Noir](https://noir-lang.org/)
- [Rust](https://www.rust-lang.org/)
- `bb` tools

## Running the Simulation

1. **Navigate to the `merkletreerust` directory**:

   ```bash
   cd merkletreerust
   ```

2. **Run the simulation**:

   ```bash
   cargo run
   ```

   This command will:
   - Build the Merkle tree with 16 leaves.
   - Use Noir's Pedersen hash functions to hash each leaf.
   - Generate the witness in `merkletreenoir/Prover.toml`.
   - Generate the proof file in `merkletreenoir/target/proof`.

## Output

The simulation will produce the following files:

- **Witness File**: Located at `merkletreenoir/Prover.toml`.
- **Proof File**: Located at `merkletreenoir/target/proof`.

## Generate a Solidity Verifier
Inside the folder merkletreenoir, write the following commands:

```bb write_vk -b ./target/merkletreenoir.json```

```bb contract```

A new Solidity contract file contract.sol is generated along with the verification key vk. It can be deployed to any EVM blockchain acting as a verifier smart contract.
Follow the steps here to [Noir](https://noir-lang.org/)[compile](https://noir-lang.org/docs/how_to/how-to-solidity-verifier#step-2---compiling), [deploy](https://noir-lang.org/docs/how_to/how-to-solidity-verifier#step-3---deploying) and [verify](https://noir-lang.org/docs/how_to/how-to-solidity-verifier#step-4---verifying) the solidity contract on-chain on any EVM blockchain (for example Sepolia testnet).

### Verify the proof 
#### Step 1: Convert the Proof to Hexadecimal
The previous generated Proof is in Binary and in order to use it in Remix in the ```verifyProof()``` function, 
the proof needs to be converted to hexadecimal format. 
Use the following command:

```HEX_PROOF=$(od -An -v -t x1 ./proof | tr -d ' \n')```
This reads the binary proof file and converts it into a continuous hexadecimal string.
```echo "0x$HEX_PROOF"```

This 0x$HEX_PROOF will be passed as the _proof parameter.

#### Step 2: Determine the Number of Public Inputs
Public inputs are explicitly defined in your Noir program and passed separately to the verifier.
In this example, there are 3 public inputs: old_root, intermediate_root, new_root.

Extract and Convert Public Inputs to Hexadecimal: Use the following command:


``` HEX_PUBLIC_INPUTS=$(head -c $PUBLIC_INPUT_BYTES ./target/proof | od -An -v -t x1 | tr -d ' \n')```
This extracts the first $PUBLIC_INPUT_BYTES from the proof file and converts them into a single hexadecimal string.

echo $HEX_PUBLIC_INPUTS 
This will output:

[
    "0x219acb6c87119c9d195e825922fb6f81e7636616d27f29cf1239e022fa36bf10",
    "0x12be21783d2eceb6968e86d597ff5d8939cd7955816500ea128793a35e1ff220",
    "0x1637d3277170f716400649d7bdbd0b47042a377a57671e662527cce2c79af5da"
]

#### Step 3: Verify in Remix
Deploy the Verifier Contract:

Ensure the verifier corresponds to the circuit used to generate the proof.
Call the verifyProof Function: In Remix, use the following format:

```
verifyProof(
    "0x219acb6c87119c9d195e825922fb6f81e7636616d27f29cf1239e022fa36bf10...", // HEX_PROOF
    [
        "0x219acb6c87119c9d195e825922fb6f81e7636616d27f29cf1239e022fa36bf10", // Public Input 1
        "0x12be21783d2eceb6968e86d597ff5d8939cd7955816500ea128793a35e1ff220", // Public Input 2
        "0x1637d3277170f716400649d7bdbd0b47042a377a57671e662527cce2c79af5da"  // Public Input 3
    ]
);
```


## File Structure

- **merkletreerust**: Contains the main Rust code to build and simulate the Merkle tree transaction.
- **merkletreenoir**: Contains the Noir circuit files for proving the transaction using zero-knowledge proofs.
- **pedersen1noir**: Noir function for hashing a single field element.
- **pedersen2noir**: Noir function for hashing two field elements.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

