// FILE: models/coffee.rs
// ___________________________________________________________

use serde::{Deserialize, Serialize};
// ___________________________________________________________


/// Coffee struct represents a type of coffee with an id and a count.
/// It implements ordering based on the count of coffee.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coffee {
    pub id: i32,
    pub count: i32,
}
// ___________________________________________________________

// Custom Ord, & PartialOrd needed for the BTreeMap to sort by count
impl Ord for Coffee {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}
// ___________________________________________________________

impl PartialOrd for Coffee {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.count.cmp(&other.count))
    }
}
// ___________________________________________________________

impl Coffee {
    /// Constructs a new Coffee.
    pub fn new(id: i32, count: i32) -> Self {
        Self { id, count }
    }

    /// Serializes the Coffee instance to a JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .unwrap_or_else(
                |err: serde_json::Error| {
                    format!("Failed to serialize coffee: {}", err)
                }
            )
    }
}
// ___________________________________________________________
