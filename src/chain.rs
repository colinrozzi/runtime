use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainEntry {
    parent: Option<String>,  // None only for genesis block
    data: Value,
}

pub struct HashChain {
    entries: HashMap<String, ChainEntry>,
    head: Option<String>,
}

impl HashChain {
    pub fn new() -> Self {
        HashChain {
            entries: HashMap::new(),
            head: None,
        }
    }

    // Initialize chain with component hash
    pub fn initialize(&mut self, component_hash: &str) {
        let genesis = ChainEntry {
            parent: None,
            data: serde_json::json!({
                "component_hash": component_hash
            }),
        };

        let hash = self.hash_entry(&genesis);
        self.entries.insert(hash.clone(), genesis);
        self.head = Some(hash);
    }

    // Hash a chain entry
    fn hash_entry(&self, entry: &ChainEntry) -> String {
        let serialized = serde_json::to_string(entry).expect("Failed to serialize entry");
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    // Add new data to the chain
    pub fn add(&mut self, data: Value) -> String {
        let entry = ChainEntry {
            parent: self.head.clone(),
            data,
        };

        let hash = self.hash_entry(&entry);
        self.entries.insert(hash.clone(), entry);
        self.head = Some(hash.clone());
        hash
    }

    // Get entry by hash
    pub fn get(&self, hash: &str) -> Option<&ChainEntry> {
        self.entries.get(hash)
    }

    // Get current head
    pub fn get_head(&self) -> Option<&str> {
        self.head.as_deref()
    }

    // Verify chain integrity
    pub fn verify(&self) -> bool {
        let mut current = self.head.as_ref();
        
        while let Some(hash) = current {
            if let Some(entry) = self.entries.get(hash) {
                // Verify hash matches entry
                if self.hash_entry(entry) != *hash {
                    return false;
                }
                current = entry.parent.as_ref();
            } else {
                return false;
            }
        }
        true
    }
}
