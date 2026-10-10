use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VectorClock {
    pub timestamps: HashMap<String, u64>,
}

impl VectorClock {
    pub fn increment(&mut self, node_id: &str) {
        let count = self.timestamps.entry(node_id.to_string()).or_insert(0);
        *count += 1;
    }

    pub fn merge(&mut self, other: &VectorClock) {
        for (node, ts) in &other.timestamps {
            let count = self.timestamps.entry(node.clone()).or_insert(0);
            *count = (*count).max(*ts);
        }
    }

    pub fn partial_cmp(&self, other: &VectorClock) -> Option<std::cmp::Ordering> {
        let mut greater = false;
        let mut less = false;

        let all_keys: std::collections::HashSet<_> = self.timestamps.keys()
            .chain(other.timestamps.keys())
            .collect();

        for key in all_keys {
            let v1 = self.timestamps.get(key).unwrap_or(&0);
            let v2 = other.timestamps.get(key).unwrap_or(&0);
            if v1 > v2 { greater = true; }
            if v1 < v2 { less = true; }
        }

        if greater && !less { Some(std::cmp::Ordering::Greater) }
        else if !greater && less { Some(std::cmp::Ordering::Less) }
        else if !greater && !less { Some(std::cmp::Ordering::Equal) }
        else { None } // Concurrent/Conflict
    }
}
