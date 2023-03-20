use ed25519_compact::KeyPair;
use ed25519_compact::Seed;
use ed25519_compact::Noise;
use std::time::Instant;

fn main() {
    // A message to sign and verify.
    let message = b"hello";

    // Generates a new key pair using a random seed.
    // A given seed will always produce the same key pair.
    let key_pair = KeyPair::from_seed(Seed::default());

    // Measures the time taken for signature.
    let start = Instant::now();
    let signature = key_pair.sk.sign(message, Some(Noise::default()));
    let signature_time = start.elapsed().as_micros();

    // Measures the time taken for verification.
    let start = Instant::now();
    key_pair
        .pk
        .verify(message, &signature)
        .expect("Signature didn't verify");
    let verification_time = start.elapsed().as_micros();

    // Prints the time taken for signature and verification.
    println!("Time taken for signature: {} μs", signature_time);
    println!("Time taken for verification: {} μs", verification_time);
}

