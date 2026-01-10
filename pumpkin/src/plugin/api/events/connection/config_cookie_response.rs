use pumpkin_macros::Event;
use pumpkin_util::resource_location::ResourceLocation;
use std::sync::Arc;

use crate::net::java::JavaClient;

/// An event that occurs when the server receives a cookie response during configuration phase.
///
/// This event allows plugins to access and process cookie data sent by the client.
/// The event is fired for both login and configuration phase cookie responses.
#[derive(Event, Clone)]
pub struct ConfigCookieResponseEvent {
    /// The Java client that sent the cookie response.
    pub client: Arc<JavaClient>,

    /// The cookie key/identifier.
    pub key: ResourceLocation,

    /// Whether the cookie has a payload.
    pub has_payload: bool,

    /// The cookie payload data, if present.
    pub payload: Option<Box<[u8]>>,
}

impl ConfigCookieResponseEvent {
    /// Creates a new instance of `ConfigCookieResponseEvent`.
    ///
    /// # Arguments
    /// - `client`: The Java client that sent the cookie response.
    /// - `key`: The cookie key/identifier.
    /// - `has_payload`: Whether the cookie has a payload.
    /// - `payload`: The cookie payload data, if present.
    ///
    /// # Returns
    /// A new instance of `ConfigCookieResponseEvent`.
    pub fn new(
        client: Arc<JavaClient>,
        key: ResourceLocation,
        has_payload: bool,
        payload: Option<Box<[u8]>>,
    ) -> Self {
        Self {
            client,
            key,
            has_payload,
            payload,
        }
    }
}
