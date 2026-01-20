#[derive(Default, Debug)]
struct Node {
    value: char,
    children: Vec<Node>,
    word: Option<String>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for ch in word.chars() {
            let found_node = current_node
                .children
                .iter()
                .enumerate()
                .find(|(_, n)| n.value == ch);

            match found_node {
                Some((index, _)) => current_node = &mut current_node.children[index],
                None => {
                    current_node.children.push(Node {
                        value: ch,
                        children: Vec::new(),
                        word: None,
                    });
                    current_node = current_node.children.last_mut().unwrap();
                }
            }
        }

        current_node.word = Some(word.to_string());
    }

    /// Finds all words that start with prefix
    pub fn find_all_by_prefix(&self, prefix: &str) -> Vec<String> {
        let mut cn = &self.root;

        for ch in prefix.chars() {
            if let Some((index, _)) = cn.children.iter().enumerate().find(|(_, n)| n.value == ch) {
                cn = &cn.children[index];
            } else {
                return vec![];
            }
        }

        Self::extract_words(cn)
    }

    /// Extracts all words starting from a given node.
    fn extract_words(root: &Node) -> Vec<String> {
        let mut extracted = vec![];

        if let Some(word) = root.word.as_ref() {
            extracted.push(word.to_string());
        }

        for child_node in root.children.iter() {
            let mut child_words = Self::extract_words(child_node);
            extracted.append(&mut child_words);
        }

        extracted
    }
}
