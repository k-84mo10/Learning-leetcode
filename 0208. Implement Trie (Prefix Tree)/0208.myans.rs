use std::collections::HashMap;

#[derive(Default, Debug)]
struct Node {
    children: HashMap<char, Box<Node>>,
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
        }
    }

    fn create_child(&mut self, ch: char) -> &mut Self {
        self.children
            .entry(ch)
            .or_insert_with(|| Box::new(Node::new()))
            .as_mut()
    }

    fn get_child(&self, ch: char) -> Option<&Self> {
        self.children.get(&ch).map(|b| b.as_ref())
    }
}

#[derive(Default, Debug)]
struct Trie {
    root: Node,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self {
            root: Node::new(),
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.create_child(ch);
        }
        node.is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        match self.traverse(&word){
            Some(node) if node.is_end => true,
            _ => false, 
        }
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        self.traverse(&prefix).is_some()
    }

    fn traverse(&self, prefix: &String) -> Option<&Node> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            node = node.get_child(ch)?;
        }
        Some(node)
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */