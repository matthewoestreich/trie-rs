# trie_rs

Messing around with trie in Rust...

# Quick Start

- Clone the repo
- Modify `bin/main.rs`
- Run `cargo run`

# Example

```rust
use trie_rs::Trie;

let mut trie = Trie::new();

trie.insert("hello");
trie.insert("help");
trie.insert("helicopter")

let words_hel = trie.find_all_by_prefix("hel");
println!("{words_hel:?}"); // ["hello", "help", "helicopter"]

let words_heli = trie.find_all_by_prefix("heli");
println!("{words_heli:?}"); // ["helicopter"]
```
