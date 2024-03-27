//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use chrono::Utc;
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let data = r#"{
        "favoritecolor": "purple",
        "favoriteseason": "spring",
        "socialsecuritynumber": "123-45-6789"
    }"#;
    let target_key = "favoritecolor";
    stdin.write(&data);
    stdin.write(&target_key);

    let start_time = Utc::now().time();
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
    let end_time = Utc::now().time();     
    let diff = end_time - start_time;
    println!("Proof generation time: {}", diff.num_seconds());

    // Save proof.
    proof
    .save("proof-with-io.json")
    .expect("saving proof failed");

    // Prover-Side Code ^
    /* ---------------- */
    // Verifier-Side Code v

    // can read these 'public' values of the proof from the proof itself (on the verifier side)
    // the rest of the JSON file's contents aren't accessible via the proof

    let target_key = proof.stdout.read::<String>();
    let target_value = proof.stdout.read::<String>();
    println!("Proved [{}: {}]", target_key, target_value);
    // also print the hash of the json file to commit to it
    let hash = proof.stdout.read::<String>();
    println!("Hash of the json file: {}", hash);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    println!("successfully generated and verified proof for the program!")
}