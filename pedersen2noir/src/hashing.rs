// This module provides functionality to hash two Fq elements using nargo,
// generating a Prover.toml file and executing the necessary nargo commands.
// You must have nargo installed in order to use it.

use crate::Error;
use ark_bn254::Fr as Fq; // Fr (scalar field) of BN254 is the Fq (base field) of Babyjubjub
use ark_std::str::FromStr; // import to use from_str in structs
use num::{BigUint, Num};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

/// Hashes two Fq elements using nargo execution.
pub fn hash_two_fq(input1: Fq, input2: Fq) -> Result<Fq, Error> {
    // Step 1: Create the Prover.toml file
    create_prover_toml(input1, input2)?;

    // Step 2: Execute nargo and retrieve the hash
    let hash_str_hex = execute_nargo_and_get_hash()?;

    // Step 3: transfer the string hash to Fq elemenet
    let hash = hex_to_fq(&hash_str_hex)?;

    Ok(hash)
}

/// Creates a Prover.toml file with the given Fq inputs to hash.
fn create_prover_toml(input1: Fq, input2: Fq) -> io::Result<()> {
    // Convert Fq elements to strings
    let input1_str = input1.to_string();
    let input2_str = input2.to_string();

    // Create and write the Prover.toml file
    let mut file = File::create("noir_pedersen/Prover.toml")?;
    writeln!(file, "input1 = \"{}\"", input1_str)?;
    writeln!(file, "input2 = \"{}\"", input2_str)?;

    Ok(())
}

/// Executes the nargo command and retrieves the hash output.
fn execute_nargo_and_get_hash() -> io::Result<String> {
    // Execute the "nargo execute" command
    let output = Command::new("nargo")
        .arg("execute")
        .current_dir("noir_pedersen") // Set the working directory
        .stdout(Stdio::piped()) // Capture stdout to avoid printing to terminal
        .stderr(Stdio::null()) // Suppress stderr output
        .spawn()? // Spawn the command
        .stdout
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to capture stdout"))?;

    // Read the first line from the command's output
    let reader = BufReader::new(output);
    if let Some(first_line) = reader.lines().next() {
        return first_line;
    }

    Err(io::Error::new(io::ErrorKind::Other, "No output from nargo"))
}

