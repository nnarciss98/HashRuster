use sha2::{Sha256, Digest};
use md5;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn hash_password_sha256(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn hash_password_md5(password: &str) -> String {
    let result = md5::compute(password);
    format!("{:x}", result)
}

fn hash_password(password: &str, hash_type: &str) -> String {
    match hash_type {
        "sha256" => hash_password_sha256(password),
        "md5" => hash_password_md5(password),
        _ => panic!("Unsupported hash type"),
    }
}

fn crack_hash(target_hash: &str, hash_type: &str, wordlist_path: &str) -> Option<String> {
    if let Ok(lines) = read_lines(wordlist_path) {
        for line in lines {
            if let Ok(password) = line {
                let hashed = hash_password(&password, hash_type);
                if hashed == target_hash {
                    return Some(password);
                }
            }
        }
    }
    None
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: cargo run --  <hash_type> <target_hash> <wordlist_path>");
        eprintln!("hash_type: sha256 or md5");
        std::process::exit(1);
    }

    let hash_type = &args[1];
    let target_hash = &args[2];
    let wordlist_path = &args[3];

    match crack_hash(target_hash, hash_type, wordlist_path) {
        Some(password) => println!("Password found: {}", password),
        None => println!("Password not found in wordlist."),
    }
}
