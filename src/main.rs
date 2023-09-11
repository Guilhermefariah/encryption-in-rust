use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, Key, KeyInit,
};
use rand::RngCore;
use uuid::Uuid;

fn main() {
    // Generate a new UUID
    let uuid4_api_key = Uuid::new_v4().to_string();
    println!("Generated UUID: {}", uuid4_api_key);

    let key_str = std::env::var("ENCRYPTED_KEY").expect("ENCRYPTED_KEY not set");
    let key = hex::decode(key_str).expect("Failed to decode key");
    let key = Key::<Aes256Gcm>::from_slice(&key);

    // Generate a 12-byte nonce
    let mut rng = rand::thread_rng();
    let mut nonce_bytes = [0u8; 12];
    rng.fill_bytes(&mut nonce_bytes);
    let nonce = GenericArray::from_slice(&nonce_bytes);

    let cipher = Aes256Gcm::new(&key);
    let ciphertext: Vec<u8> = cipher.encrypt(&nonce, uuid4_api_key.as_ref()).unwrap();

    // Construct the combined layout: nonce + ciphertext + auth tag
    let mut combined = Vec::new();
    combined.extend_from_slice(&nonce);
    combined.extend_from_slice(&ciphertext);

    println!(
        "Combined (nonce + ciphertext + auth tag): {:?}",
        hex::encode(&combined)
    );

    // Now let's simulate the decryption side:

    // Split the nonce, ciphertext, and auth tag
    let received_nonce = &combined[..12];
    let received_ciphertext_with_tag = &combined[12..];

    let received_nonce = GenericArray::from_slice(received_nonce);
    let plaintext = cipher.decrypt(received_nonce, received_ciphertext_with_tag).unwrap();
    let plaintext_from_utf8 = std::str::from_utf8(&plaintext).unwrap();

    println!("Decrypted plaintext: {:?}", plaintext_from_utf8);
    assert_eq!(plaintext_from_utf8, uuid4_api_key);
}
