extern crate crypto;

use std::io;
use crypto::digest::Digest;
use crypto::md5::Md5;

// Function takes a mutable String reference
fn get_username(buffer: &mut String) -> () {
    loop {
        println!("Please enter a username:");
        
        // Since the String is already borrowed as a mutable reference
        // we don't need to borrow it again to pass it in to io::stdin().read_line
        match io::stdin().read_line(buffer) {
            Ok(_) => break,
            Err(_) => continue,
        };
    }
}

fn md5_hash(buffer: &str) -> String {
    // Hash the string using the Md5 algorithm
    let mut hasher = Md5::new();
    hasher.input_str(buffer);
    hasher.result_str()
}

fn mirror_row(row: &[u8]) -> [u8; 5] {
    match row {
        &[a, b, c] => [a, b, c, b, a],
                _  => panic!()
    }
}

fn mirror_grid(chunked_digest: &mut std::slice::Chunks<'_, u8>) -> [[u8; 5]; 5] {
    [
        mirror_row(chunked_digest.next().unwrap()),
        mirror_row(chunked_digest.next().unwrap()),
        mirror_row(chunked_digest.next().unwrap()),
        mirror_row(chunked_digest.next().unwrap()),
        mirror_row(chunked_digest.next().unwrap())
    ]
}

fn main() {
    // Create a mutable String
    let mut buffer: String = String::new();
    // Borrow buffer as a mutable String reference
    get_username(&mut buffer);

    // Get the digest hash as byte array
    let digest: String = md5_hash(&buffer);
    let digest: &[u8] = digest.as_bytes();
    // Chunk the hash into blocks of 3
    let mut chunked_digest:  std::slice::Chunks<'_, u8> 
        = digest[..].chunks(3);
    // Convert the first 5 chunks into a mirrored grid
    let grid: [[u8; 5]; 5] = mirror_grid(&mut chunked_digest);
    // Grab the first 3 values of the digest for the color
    let _color: &[u8] = &digest[..3];

    println!("{:?}", grid)
}