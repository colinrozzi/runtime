use std::hash::{Hash, Hasher};

pub struct HashChain {
    chain: Vec<String>,
}

impl HashChain {
    pub fn new() -> Self {
        HashChain { chain: vec![] }
    }

    pub fn add(&mut self, data: String) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        data.hash(&mut hasher);
        let hash = format!("{:x}", hasher.finish());
        let print_hash = hash.clone();
        self.chain.push(hash);
        println!("Added to chain: {}: {}", print_hash, data);
    }

    pub fn get(&self) -> Vec<String> {
        self.chain.clone()
    }

    pub fn get_last(&self) -> String {
        self.chain.last().unwrap().clone()
    }
}
