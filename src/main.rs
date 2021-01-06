extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use std::vec::Vec;

fn main() {
    let start_proof = Instant::now();
    let mut sha256 = Sha256::new();
    let mut hashes = Vec::new();
    sha256.input_str("GENISIS_STRING");
    hashes.push(sha256.result_str());
    println!("{:?}", hashes[0]);

    for i in 1..1000000 {
        sha256 = Sha256::new();
        sha256.input_str(hashes[i - 1].as_str());
        hashes.push(sha256.result_str());
        println!("proof N°{:?}: {:?}", i, hashes[i]);
    }
    let duration_proof = start_proof.elapsed();
    println!("time to compute proof: {:?}", duration_proof);

    let mut children = vec![];
    let proofs = Arc::new(hashes);
    let start_verify = Instant::now();

    for i in 0..10 {
        let val = proofs.clone();
        children.push(thread::spawn(move || {
            for j in i * 100000..i * 100000 + 100000 {
                let proof_str = if j == 0 {
                    "GENISIS_STRING"
                } else {
                    val[j - 1].as_str()
                };
                let mut hasher = Sha256::new();
                hasher.input_str(proof_str);

                let proof = val[j].as_str();
                let output = hasher.result_str();
                println!("output N°{:?}: {:?}", j, output);
                println!("proof N°{:?}: {:?}", j, proof);

                if proof != output {
                    panic!("hello panic");
                }
            }
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    let duration_verify = start_verify.elapsed();
    println!("time to verify proof: {:?}", duration_verify);
    println!(
        "producing proof {:?} VS verifying {:?}",
        duration_proof, duration_verify
    );
    println!("This demo only use 10 core (if your CPU has it)");
    println!("Now imagine levearging your GPU's core (thousand of them)");
}
