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

#[cfg(test)]
mod test {

    use crate::identify::Identify;

    use super::*;

    #[test]
    fn test_device_id() {
        let identify = Identify::from_pem(
            "-----BEGIN PRIVATE KEY-----
MFECAQEwBQYDK2VwBCIEIBAa0Ut5N/mC6ssT7J7z156s3oXCMRet63BMC+dIbeXg
gSEATLA6Bz/CL2XjgRMXT4GyIGHJ4nbQMaP6hi1LAq0pUkw=
-----END PRIVATE KEY-----",
        )
        .unwrap();
        let device_id = DeviceId::from(identify.signing_key.verifying_key());
        println!(
            "device id: {device_id:?}---{} \n {}",
            device_id.0.len(),
            device_id.label()
        );
    }
}
