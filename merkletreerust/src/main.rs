use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::process::Stdio;

struct Witness {
    intermediate_root: String,
    leaf_index1: String,
    leaf_index2: String,
    new_leaf1: String,
    new_leaf2: String,
    new_path1: Vec<String>,
    new_path2: Vec<String>,
    new_root: String,
    old_leaf1: String,
    old_leaf2: String,
    old_path1: Vec<String>,
    old_path2: Vec<String>,
    old_root: String,
}

impl Witness {
    fn new(
        intermediate_root: String,
        leaf_index1: String,
        leaf_index2: String,
        new_leaf1: String,
        new_leaf2: String,
        new_path1: Vec<String>,
        new_path2: Vec<String>,
        new_root: String,
        old_leaf1: String,
        old_leaf2: String,
        old_path1: Vec<String>,
        old_path2: Vec<String>,
        old_root: String,
    ) -> Self {
        Self {
            intermediate_root,
            leaf_index1,
            leaf_index2,
            new_leaf1,
            new_leaf2,
            new_path1,
            new_path2,
            new_root,
            old_leaf1,
            old_leaf2,
            old_path1,
            old_path2,
            old_root,
        }
    }

    fn to_toml_string(&self) -> String {
        format!(
            r#"
intermediate_root = "{intermediate_root}"
leaf_index1 = "{leaf_index1}"
leaf_index2 = "{leaf_index2}"
new_leaf1 = "{new_leaf1}"
new_leaf2 = "{new_leaf2}"
new_path1 = {new_path1:?}
new_path2 = {new_path2:?}
new_root = "{new_root}"
old_leaf1 = "{old_leaf1}"
old_leaf2 = "{old_leaf2}"
old_path1 = {old_path1:?}
old_path2 = {old_path2:?}
old_root = "{old_root}"
"#,
            intermediate_root = self.intermediate_root,
            leaf_index1 = self.leaf_index1,
            leaf_index2 = self.leaf_index2,
            new_leaf1 = self.new_leaf1,
            new_leaf2 = self.new_leaf2,
            new_path1 = self.new_path1,
            new_path2 = self.new_path2,
            new_root = self.new_root,
            old_leaf1 = self.old_leaf1,
            old_leaf2 = self.old_leaf2,
            old_path1 = self.old_path1,
            old_path2 = self.old_path2,
            old_root = self.old_root
        )
        .replace("[\"", "[\"")
        .replace("\"]", "\"]")
        .replace(", ", ", ")
    }

    fn write_to_toml_file(&self, file_path: &str) -> io::Result<()> {
        let toml_content = self.to_toml_string();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // This ensures that the file is truncated if it already exists
            .open(file_path)?;
        file.write_all(toml_content.as_bytes())?;
        Ok(())
    }
}

fn generate_proof() -> io::Result<()> {
    let mut command = Command::new("bb")
        .arg("prove")
        .arg("-b")
        .arg("../merkletreenoir/target/merkletreenoir.json")
        .arg("-w")
        .arg("../merkletreenoir/target/merkletreenoir.gz")
        .arg("-o")
        .arg("../merkletreenoir/target/proof")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(l) => println!("Standard Output: {}", l),
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    }

    if let Some(stderr) = command.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(l) => eprintln!("Standard Output (from stderr): {}", l),
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    }

    let status = command.wait()?;
    if status.success() {
        println!("Proof generated successfully.");
    } else {
        eprintln!("Error generating proof.");
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct MerkleTree {
    leaf_nodes: Vec<String>, // Stores hash of each leaf node
    tree: Vec<Vec<String>>,  // Stores the entire Merkle tree layers
}

impl MerkleTree {
    // Creates a new Merkle tree from the given leaf data
    fn new(leaf_data: Vec<String>) -> Self {
        let leaf_nodes: Vec<String> = leaf_data;

        let mut tree = vec![leaf_nodes.clone()];

        // Build the tree level by level
        while tree.last().unwrap().len() > 1 {
            let prev_level = tree.last().unwrap();
            let mut new_level = Vec::new();

            // Hash pairs of nodes from the previous level to create the current level
            for pair in prev_level.chunks(2) {
                let hash = if pair.len() == 2 {
                    compute_pedersen_hash2(&pair[0], &pair[1]).expect("Hashing failed")
                } else {
                    pair[0].clone() // Odd node case
                };
                new_level.push(hash);
            }

            tree.push(new_level);
        }

        Self { leaf_nodes, tree }
    }

    // Retrieves the Merkle root of the tree
    fn root(&self) -> &String {
        self.tree.last().unwrap().first().unwrap()
    }

    // Retrieves the Merkle path proof for a given leaf index
    fn merkle_path(&self, index: usize) -> Vec<String> {
        let mut path = Vec::new();
        let mut idx = index;

        for level in &self.tree[..self.tree.len() - 1] {
            // Exclude root level
            let sibling_index = if idx % 2 == 0 { idx + 1 } else { idx - 1 };

            if sibling_index < level.len() {
                path.push(level[sibling_index].clone());
            }
            idx /= 2;
        }

        path
    }

    // Updates a leaf node and recomputes the affected nodes up to the root
    fn update_leaf(&mut self, index: usize, new_data: &str) {
        // Update the leaf node with the new hash
        self.leaf_nodes[index] = new_data.to_string();
        self.tree[0][index] = self.leaf_nodes[index].clone();

        // Recompute hashes up the tree
        let mut idx = index;
        for i in 0..self.tree.len() - 1 {
            let parent_idx = idx / 2;
            let left_child = &self.tree[i][parent_idx * 2];
            let right_child = if parent_idx * 2 + 1 < self.tree[i].len() {
                &self.tree[i][parent_idx * 2 + 1]
            } else {
                left_child
            };
            self.tree[i + 1][parent_idx] =
                compute_pedersen_hash2(left_child, right_child).expect("Hashing failed");
            idx = parent_idx;
        }
    }

    fn print_tree(&self) {
        for level in &self.tree {
            let truncated_level: Vec<String> = level
                .iter()
                .map(|hash| {
                    if hash.len() > 8 {
                        hash[hash.len() - 8..].to_string() // Take the last 8 characters
                    } else {
                        hash.clone()
                    }
                })
                .collect();
            println!("{:?}", truncated_level);
        }
    }

    fn _print_tree_full_hash(&self) {
        for level in &self.tree {
            println!("{:?}", level);
        }
    }
}
fn compute_pedersen_hash1(field_element1: &str) -> Result<String, String> {
    // Step 1: Populate Prover.toml
    let toml_content = format!("\ninput1 = \"{}\"", field_element1);

    let mut file =
        File::create("../pedersen1noir/Prover.toml").expect("Unable to create Prover.toml");
    file.write_all(toml_content.as_bytes())
        .expect("Unable to write to Prover.toml");

    // Step 2: Run Noir's `nargo` command and capture output
    let output = Command::new("nargo")
        .args(&[
            "execute",
            "-p",
            "Prover",
            "--program-dir",
            "../pedersen1noir/",
        ])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output() // Using output() to capture stdout and stderr directly
        .expect("Failed to execute Noir program");

    // Check if nargo command executed successfully
    if !output.status.success() {
        return Err(format!(
            "nargo command failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Step 3: Read and parse Noir's output to extract hash
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("0x") {
            return Ok(line.trim().to_string()); // Return the hash if found
        }
    }

    Err("Hash output not found".to_string())
}

fn compute_pedersen_hash2(field_element1: &str, field_element2: &str) -> Result<String, String> {
    // Step 1: Populate Prover.toml
    let toml_content = format!(
        "\ninput1 = \"{}\"\ninput2 = \"{}\"",
        field_element1, field_element2
    );

    let mut file =
        File::create("../pedersen2noir/Prover.toml").expect("Unable to create Prover.toml");
    file.write_all(toml_content.as_bytes())
        .expect("Unable to write to Prover.toml");

    // Step 2: Run Noir's `nargo` command and capture output
    let output = Command::new("nargo")
        .args(&[
            "execute",
            "-p",
            "Prover",
            "--program-dir",
            "../pedersen2noir/",
        ])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output() // Using output() to capture stdout and stderr directly
        .expect("Failed to execute Noir program");

    // Check if nargo command executed successfully
    if !output.status.success() {
        return Err(format!(
            "nargo command failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Step 3: Read and parse Noir's output to extract hash
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("0x") {
            return Ok(line.trim().to_string()); // Return the hash if found
        }
    }

    Err("Hash output not found".to_string())
}

fn main() {
    // Example usage
    let leaves = vec![
        compute_pedersen_hash1("1").unwrap(),
        compute_pedersen_hash1("2").unwrap(),
        compute_pedersen_hash1("3").unwrap(),
        compute_pedersen_hash1("4").unwrap(),
        compute_pedersen_hash1("5").unwrap(),
        compute_pedersen_hash1("6").unwrap(),
        compute_pedersen_hash1("7").unwrap(),
        compute_pedersen_hash1("8").unwrap(),
        compute_pedersen_hash1("9").unwrap(),
        compute_pedersen_hash1("10").unwrap(),
        compute_pedersen_hash1("11").unwrap(),
        compute_pedersen_hash1("12").unwrap(),
        compute_pedersen_hash1("13").unwrap(),
        compute_pedersen_hash1("14").unwrap(),
        compute_pedersen_hash1("15").unwrap(),
        compute_pedersen_hash1("16").unwrap(),
    ];

    // Create a Merkle tree
    let mut merkle_tree = MerkleTree::new(leaves);

    println!("Initial Merkle Tree:");
    // Print the Merkle tree
    merkle_tree.print_tree();

    // Print the Merkle root
    let merkle_tree_root1 = merkle_tree.root().clone();
    println!("Merkle Root: {}", merkle_tree_root1);

    println!("----------------------------------------------------");
    println!("A new transaction from node index 2 to node index 10");
    println!("----------------------------------------------------");

    let merkle_path_sender_before = merkle_tree.merkle_path(2);
    let merkle_path_receiver_before = merkle_tree.merkle_path(10);
    let sender_hash_before = merkle_tree.leaf_nodes[2].clone();
    let receiver_hash_before = merkle_tree.leaf_nodes[10].clone();

    println!("Update in node index 2");
    // Update a leaf and recompute the tree
    merkle_tree.update_leaf(2, compute_pedersen_hash1("100").unwrap().as_str());
    merkle_tree.print_tree();
    let merkle_tree_root2 = merkle_tree.root().clone();
    println!("Updated Merkle Root: {}", merkle_tree_root2);

    // Get Merkle path for the first leaf
    let merkle_path_sender_after = merkle_tree.merkle_path(2);
    println!(
        "Merkle Path proof for leaf 2: {:?}",
        merkle_path_sender_after
    );
    println!("----------------------------------------------------");
    println!("Update in node index 10");
    // Update a leaf and recompute the tree
    merkle_tree.update_leaf(10, compute_pedersen_hash1("200").unwrap().as_str());
    merkle_tree.print_tree();
    let merkle_tree_root3 = merkle_tree.root().clone();
    println!("Updated Merkle Root: {}", merkle_tree_root3);

    // Get Merkle path for the first leaf
    let merkle_path_receiver_after = merkle_tree.merkle_path(10);
    println!(
        "Merkle Path proof for leaf 10: {:?}",
        merkle_path_receiver_after
    );
    println!("----------------------------------------------------");
    println!("----------------------------------------------------");
    println!("Summary of state transitions:");
    println!("Merkle root before any updates: {:?}", merkle_tree_root1);
    println!(
        "Merkle root after updating node index 2: {:?}",
        merkle_tree_root2
    );
    println!(
        "Merkle root after updating node index 10: {:?}",
        merkle_tree_root3
    );
    println!("----------------------------------------------------");
    println!("Merkle hash leaf 2 before update: {:?}", sender_hash_before);
    println!(
        "Merkle path proof for leaf 2 before update: {:?}",
        merkle_path_sender_before
    );
    println!(
        "Merkle hash leaf 10 before update: {:?}",
        receiver_hash_before
    );
    println!(
        "Merkle path proof for leaf 10 before update: {:?}",
        merkle_path_receiver_before
    );
    println!("----------------------------------------------------");
    println!(
        "Merkle hash leaf 2 after update: {:?}",
        merkle_tree.leaf_nodes[2].clone()
    );
    println!(
        "Merkle path proof for leaf 2 after update: {:?}",
        merkle_path_sender_after
    );
    println!(
        "Merkle hash leaf 10 after update: {:?}",
        merkle_tree.leaf_nodes[10].clone()
    );
    println!(
        "Merkle path proof for leaf 10 after update: {:?}",
        merkle_path_receiver_after
    );
    println!("----------------------------------------------------");

    // Create a witness
    let witness = Witness::new(
        merkle_tree_root2,
        "0x2".to_string(),
        "0xa".to_string(),
        merkle_tree.leaf_nodes[2].clone(),
        merkle_tree.leaf_nodes[10].clone(),
        merkle_path_sender_after,
        merkle_path_receiver_after,
        merkle_tree_root3,
        sender_hash_before,
        receiver_hash_before,
        merkle_path_sender_before,
        merkle_path_receiver_before,
        merkle_tree_root1,
    );
    witness
        .write_to_toml_file("../merkletreenoir/Prover.toml")
        .expect("Unable to write to Prover.toml");

    // Generate proof
    if let Err(e) = generate_proof() {
        eprintln!("Failed to execute command: {}", e);
    }
}
