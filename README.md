# Encryption in Rust 🛡️

This project demonstrates how to use the aes-gcm crate in Rust to encrypt and decrypt data. Specifically, it shows how to generate a UUID, encrypt it using AES-256-GCM, and then decrypt it back to its original form.

## Table of Contents

* Installation
* Usage
* Environment Variables
* Dependencies
* License

## Installation

To get started, clone this repository and navigate to the project directory

```sh
git clone https://github.com/yourusername/encryption-in-rust.git

cd encryption-in-rust
```



Ensure you have <strong>Rust</strong> installed. If not, you can install it using <strong>rustup</strong>.

## Usage

Before running the program, make sure to set the <strong>ENCRYPTED_KEYM</strong> environment variable with a 256-bit key encoded in hexadecimal format.

```sh
export ENCRYPTED_KEY=your_256_bit_key_in_hex

```
To run the program, use the following command

```sh
cargo run
```

The program will:

* Generate a new UUID.
* Encrypt the UUID using AES-256-GCM.
* Print the combined output of nonce, ciphertext, and authentication tag.
* Decrypt the combined output back to the original UUID.

## Environment Variables
* <strong>ENCRYPTED_KEY</strong>: A 256-bit key for AES encryption, encoded in hexadecimal format. This key must be set before running the program.

## Dependencies
This project uses the following dependencies:

* aes-gcm: Provides the AES-GCM encryption scheme.
* hex: For encoding and decoding hexadecimal strings.
* uuid: For generating UUIDs.
* rand: For generating random numbers.

## License

This project is licensed under the MIT License. See the <strong>LICENSE</strong> file for more details.