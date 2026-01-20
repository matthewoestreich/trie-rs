use trie_rs::Trie;

fn main() {
    let mut t = Trie::new();
    t.insert("hello");
    t.insert("help");
    t.insert("helicopter");
    let suggestions = t.find_all_by_prefix("hel");
    println!("{suggestions:?}");
    let check_hel = t.contains("he");
    println!("contains `hel`? {check_hel}");
}
