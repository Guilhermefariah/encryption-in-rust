use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};

fn main() {
    // The encryption key can be generated randomly:
    // let key = Aes256Gcm::generate_key(OsRng);
    let uuid4_api_key = "".to_string();

    let key = "".to_string();
    let nonce = "".to_string();

    if key.len() != 64 {
        panic!("Invalid length for key",);
    };
    if nonce.len() != 24 {
        panic!("Invalid length for nonce",);
    };
    let key = hex::decode(key).unwrap();
    let nonce = hex::decode(nonce).unwrap();

    let key = Key::<Aes256Gcm>::from_slice(&key);
    let nonce = GenericArray::from_slice(&nonce);

    let cipher = Aes256Gcm::new(&key);
    let ciphertext: Vec<u8> = cipher.encrypt(&nonce, uuid4_api_key.as_ref()).unwrap();
    println!("Ciphertext: {:?}", hex::encode(&ciphertext));

    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    let plaintext_from_utf8 = std::str::from_utf8(&plaintext).unwrap();
    println!("Plaintext from utf8: {:?}", plaintext_from_utf8);
    assert_eq!(std::str::from_utf8(&plaintext).unwrap(), uuid4_api_key);
    println!(
        "Decrypted plaintext: {:?}",
        std::str::from_utf8(&plaintext).unwrap()
    );
}
