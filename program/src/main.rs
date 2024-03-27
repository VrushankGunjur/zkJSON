//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use serde_json::Value;
use sha2::{Sha256, Digest};

// guest program

pub fn main() {

    // read inputs
    let data = sp1_zkvm::io::read::<String>();
    let target_key = sp1_zkvm::io::read::<String>();
    let json_data: Value = serde_json::from_str(&data).unwrap();


    let target_value = &json_data[target_key.clone()];

    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    // write key:value as public inputs as part of the json file
    sp1_zkvm::io::write(&target_key);
    sp1_zkvm::io::write(&target_value);

    // write the hash of the json file to public inputs
    sp1_zkvm::io::write(&format!("{:x}", hash).as_str());
}