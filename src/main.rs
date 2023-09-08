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

    let key_str = "";
    let key = hex::decode(key_str).expect("Failed to decode key");
    let key = Key::<Aes256Gcm>::from_slice(&key);

    // Generate a 12-byte nonce
    let mut rng = rand::thread_rng();
    let mut nonce_bytes = [0u8; 12];
    rng.fill_bytes(&mut nonce_bytes);
    let nonce = GenericArray::from_slice(&nonce_bytes);

    let cipher = Aes256Gcm::new(&key);
    let ciphertext: Vec<u8> = cipher.encrypt(&nonce, uuid4_api_key.as_ref()).unwrap();

    // Append nonce to ciphertext
    let mut combined = ciphertext.clone();
    combined.extend_from_slice(&nonce);

    println!(
        "Combined (ciphertext + nonce): {:?}",
        hex::encode(&combined)
    );

    // Now let's simulate the decryption side:

    // Split the ciphertext and nonce
    let nonce_pos = combined.len() - 12;
    let received_ciphertext = &combined[..nonce_pos];
    let received_nonce = &combined[nonce_pos..];

    let received_nonce = GenericArray::from_slice(received_nonce);
    let plaintext = cipher.decrypt(received_nonce, received_ciphertext).unwrap();
    let plaintext_from_utf8 = std::str::from_utf8(&plaintext).unwrap();

    println!("Decrypted plaintext: {:?}", plaintext_from_utf8);
    assert_eq!(plaintext_from_utf8, uuid4_api_key);
}
