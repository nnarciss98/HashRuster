# HashRuster

Hash cracker made in rust.
This tool can crack sha256 or md5 hashes

## How to use it

Choose the type of hash you want to crack, then you can use any wordlist of your choice to crack the hash.
```
cargo run -- <sha256> <hash> <wordlist.txt>
cargo run -- <md5> <hash> <wordlist.txt>
```