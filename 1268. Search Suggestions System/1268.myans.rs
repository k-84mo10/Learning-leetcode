use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    suggestions: Vec<String>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            suggestions: Vec::new(),
        }
    }

    fn create_child(&mut self, ch: char) -> &mut Self {
        self.children
            .entry(ch)
            .or_insert_with(|| Box::new(TrieNode::new()))
            .as_mut()
    } 

    fn get_child(&self, ch: char) -> Option<&Self> {
        self.children.get(&ch).map(|b| b.as_ref())
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort();

        let mut root = TrieNode::new();

        for s in &products {
            let mut node = &mut root;
            for ch in s.chars() {
                node = node.create_child(ch);
                if node.suggestions.len() < 3 {
                    node.suggestions.push(s.clone());
                }
            }
        }

        let mut result = Vec::new();
        let mut node_opt: Option<&TrieNode> = Some(&root);

        for ch in search_word.chars() {
            if let Some(node) = node_opt.and_then(|n| n.get_child(ch)) {
                result.push(node.suggestions.clone());
                node_opt = Some(&node);
            } else {
                result.push(Vec::new());
                node_opt = None;
            }
        }

        result
    }
}