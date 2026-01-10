use std::{net::IpAddr, path::Path, sync::LazyLock};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{LoadJSONConfiguration, SaveJSONConfiguration, banlist_serializer::BannedIpEntry};

pub static BANNED_IP_LIST: LazyLock<tokio::sync::RwLock<BannedIpList>> =
    LazyLock::new(|| tokio::sync::RwLock::new(BannedIpList::load()));

#[derive(Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct BannedIpList {
    pub banned_ips: Vec<BannedIpEntry>,
}

impl BannedIpList {
    #[must_use]
    pub fn get_entry(&mut self, ip: &IpAddr) -> Option<&BannedIpEntry> {
        self.remove_invalid_entries();
        self.banned_ips.iter().find(|entry| entry.ip == *ip)
    }

    fn remove_invalid_entries(&mut self) {
        let original_len = self.banned_ips.len();

        self.banned_ips.retain(|entry| {
            entry
                .expires
                .is_none_or(|expires| expires >= OffsetDateTime::now_utc())
        });

        if original_len != self.banned_ips.len() {
            self.save();
        }
    }
}

impl LoadJSONConfiguration for BannedIpList {
    fn get_path() -> &'static Path {
        Path::new("banned-ips.json")
    }
    fn validate(&self) {
        use std::collections::HashSet;

        if self.banned_ips.is_empty() {
            return;
        }

        let mut seen_ips = HashSet::new();
        let mut issues_found = false;

        for (index, entry) in self.banned_ips.iter().enumerate() {
            // Check for duplicate IPs
            if !seen_ips.insert(&entry.ip) {
                log::warn!(
                    "Duplicate IP in banned-ips.json at index {}: {}. This may cause unexpected behavior.",
                    index,
                    entry.ip
                );
                issues_found = true;
            }

            // IpAddr type guarantees the IP is valid, but we can check for special cases
            if entry.ip.is_unspecified() {
                log::warn!(
                    "Unspecified IP address (0.0.0.0 or ::) found in banned-ips.json at index {}. This may ban all players.",
                    index
                );
                issues_found = true;
            }

            if entry.ip.is_loopback() {
                log::warn!(
                    "Loopback IP address found in banned-ips.json at index {}: {}. This only affects local connections.",
                    index,
                    entry.ip
                );
            }

            // Validate expiration date is in the future if present
            if let Some(expires) = entry.expires
                && expires <= OffsetDateTime::now_utc()
            {
                log::debug!(
                    "Expired IP ban found in banned-ips.json at index {} for {}. This will be removed automatically.",
                    index,
                    entry.ip
                );
            }
        }

        if issues_found {
            log::warn!("Banned IPs list contains issues. Consider reviewing banned-ips.json.");
        }

        log::debug!("Validated {} banned IP entries", self.banned_ips.len());
    }
}

impl SaveJSONConfiguration for BannedIpList {}
