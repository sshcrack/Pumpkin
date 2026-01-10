use std::{path::Path, sync::LazyLock};

use pumpkin_config::op;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{LoadJSONConfiguration, SaveJSONConfiguration};

pub static OPERATOR_CONFIG: LazyLock<tokio::sync::RwLock<OperatorConfig>> =
    LazyLock::new(|| tokio::sync::RwLock::new(OperatorConfig::load()));

#[derive(Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct OperatorConfig {
    pub ops: Vec<op::Op>,
}

impl OperatorConfig {
    #[must_use]
    pub fn get_entry(&self, uuid: &Uuid) -> Option<&op::Op> {
        self.ops.iter().find(|entry| entry.uuid.eq(uuid))
    }
}

impl LoadJSONConfiguration for OperatorConfig {
    fn get_path() -> &'static Path {
        Path::new("ops.json")
    }
    fn validate(&self) {
        use std::collections::HashSet;
        
        if self.ops.is_empty() {
            return;
        }
        
        let mut seen_uuids = HashSet::new();
        let mut seen_names = HashSet::new();
        let mut duplicates_found = false;
        
        for (index, op) in self.ops.iter().enumerate() {
            // Check for duplicate UUIDs
            if !seen_uuids.insert(&op.uuid) {
                log::warn!(
                    "Duplicate UUID in ops.json at index {}: {} ({}). This entry will still be used but may cause issues.",
                    index, op.uuid, op.name
                );
                duplicates_found = true;
            }
            
            // Check for duplicate names (warning only, as names can theoretically be reused)
            if !seen_names.insert(&op.name) {
                log::warn!(
                    "Duplicate name in ops.json at index {}: '{}'. Multiple operators with the same name may cause confusion.",
                    index, op.name
                );
            }
            
            // Validate UUID is not nil
            if op.uuid.is_nil() {
                log::warn!(
                    "Nil UUID found in ops.json at index {} for name '{}'. This entry may not work correctly.",
                    index, op.name
                );
            }
            
            // Validate name is not empty
            if op.name.is_empty() {
                log::warn!(
                    "Empty name found in ops.json at index {} for UUID {}. This entry may not work correctly.",
                    index, op.uuid
                );
            }
        }
        
        if duplicates_found {
            log::warn!("Operator list contains duplicate entries. Consider cleaning up ops.json.");
        }
        
        log::debug!("Validated {} operator entries", self.ops.len());
    }
}

impl SaveJSONConfiguration for OperatorConfig {}
