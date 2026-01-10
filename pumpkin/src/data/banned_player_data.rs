use std::{path::Path, sync::LazyLock};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::net::GameProfile;

use super::{LoadJSONConfiguration, SaveJSONConfiguration, banlist_serializer::BannedPlayerEntry};

pub static BANNED_PLAYER_LIST: LazyLock<tokio::sync::RwLock<BannedPlayerList>> =
    LazyLock::new(|| tokio::sync::RwLock::new(BannedPlayerList::load()));

#[derive(Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct BannedPlayerList {
    pub banned_players: Vec<BannedPlayerEntry>,
}

impl BannedPlayerList {
    #[must_use]
    pub fn get_entry(&mut self, profile: &GameProfile) -> Option<&BannedPlayerEntry> {
        self.remove_invalid_entries();
        self.banned_players
            .iter()
            .find(|entry| entry.name == profile.name && entry.uuid == profile.id)
    }

    fn remove_invalid_entries(&mut self) {
        let original_len = self.banned_players.len();

        self.banned_players.retain(|entry| {
            entry
                .expires
                .is_none_or(|expires| expires >= OffsetDateTime::now_utc())
        });

        if original_len != self.banned_players.len() {
            self.save();
        }
    }
}

impl LoadJSONConfiguration for BannedPlayerList {
    fn get_path() -> &'static Path {
        Path::new("banned-players.json")
    }
    fn validate(&self) {
        use std::collections::HashSet;

        if self.banned_players.is_empty() {
            return;
        }

        let mut seen_uuids = HashSet::new();
        let mut seen_names = HashSet::new();
        let mut issues_found = false;

        for (index, entry) in self.banned_players.iter().enumerate() {
            // Check for duplicate UUIDs
            if !seen_uuids.insert(&entry.uuid) {
                log::warn!(
                    "Duplicate UUID in banned-players.json at index {}: {} ({}). This may cause unexpected behavior.",
                    index,
                    entry.uuid,
                    entry.name
                );
                issues_found = true;
            }

            // Check for duplicate names
            if !seen_names.insert(&entry.name) {
                log::warn!(
                    "Duplicate name in banned-players.json at index {}: '{}'. Consider reviewing this entry.",
                    index,
                    entry.name
                );
            }

            // Validate UUID is not nil
            if entry.uuid.is_nil() {
                log::warn!(
                    "Nil UUID found in banned-players.json at index {} for name '{}'. This entry may not work correctly.",
                    index,
                    entry.name
                );
                issues_found = true;
            }

            // Validate name is not empty
            if entry.name.is_empty() {
                log::warn!(
                    "Empty name found in banned-players.json at index {} for UUID {}. This entry may not work correctly.",
                    index,
                    entry.uuid
                );
                issues_found = true;
            }

            // Validate expiration date is in the future if present
            if let Some(expires) = entry.expires
                && expires <= OffsetDateTime::now_utc()
            {
                log::debug!(
                    "Expired ban found in banned-players.json at index {} for '{}' ({}). This will be removed automatically.",
                    index,
                    entry.name,
                    entry.uuid
                );
            }
        }

        if issues_found {
            log::warn!(
                "Banned players list contains issues. Consider reviewing banned-players.json."
            );
        }

        log::debug!(
            "Validated {} banned player entries",
            self.banned_players.len()
        );
    }
}

impl SaveJSONConfiguration for BannedPlayerList {}
