use base32::Alphabet;
use ed25519_dalek::VerifyingKey;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct DeviceId(String);

impl Deref for DeviceId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DeviceId {
    pub fn label(&self) -> String {
        self.0.chars().step_by(4).collect()
    }
}

impl From<VerifyingKey> for DeviceId {
    fn from(value: VerifyingKey) -> Self {
        let hash = blake3::hash(value.as_bytes());
        let id_b32 = base32::encode(Alphabet::Crockford, hash.as_bytes());
        Self(id_b32)
    }
}
