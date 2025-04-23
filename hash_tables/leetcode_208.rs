use std::collections::HashMap;
#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

#[derive(Debug, Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            current_node = current_node.children.entry(c).or_insert(TrieNode::default());
            println!("{:#?}", current_node); // Debug print to show the current node
        }
        current_node.is_end_of_word = true;
        println!("{:#?}", current_node); // Debug print to show the end of word node
    }

    fn search(&self, word: String) -> bool {
        let mut current_node = &self.root;
        for c in word.chars() {
            if let Some(node) = current_node.children.get(&c) {
                current_node = node;
            } else {
                return false;
            }
        }
        current_node.is_end_of_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current_node = &self.root;
        for c in prefix.chars() {
            if let Some(node) = current_node.children.get(&c) {
                current_node = node;
            } else {
                return false;
            }
        }
        true
    }
}


fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    println!("{}", trie.search("apple".to_string())); // returns true
    println!("{}", trie.search("app".to_string())); // returns false
    println!("{}", trie.starts_with("app".to_string())); // returns true
    trie.insert("app".to_string());
    println!("{}", trie.search("app".to_string())); // returns true
}