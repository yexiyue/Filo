use crate::Result;
use anyhow::anyhow;
use ed25519_dalek::{
    pkcs8::{spki::der::pem::LineEnding, DecodePrivateKey, EncodePrivateKey},
    SigningKey,
};
use rand::rand_core::TryRngCore;
use rand::rngs::OsRng;

use std::{fs, path::Path};

pub struct Identify {
    pub signing_key: SigningKey,
}

impl Identify {
    pub fn generate() -> Identify {
        let mut csprng = OsRng.unwrap_err();
        let signing_key = SigningKey::generate(&mut csprng);
        Self { signing_key }
    }

    pub fn from_pem(pem: &str) -> Result<Identify> {
        let res = SigningKey::from_pkcs8_pem(pem).map_err(|e| anyhow!(e))?;
        Ok(Identify { signing_key: res })
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Identify> {
        let path = path.as_ref();
        if path.exists() {
            let pem = std::fs::read_to_string(path)?;
            Identify::from_pem(&pem)
        } else {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let identify = Identify::generate();

            fs::write(
                path,
                identify
                    .signing_key
                    .to_pkcs8_pem(LineEnding::LF)
                    .map_err(|e| anyhow!("{}", e))?
                    .to_string(),
            )?;
            Ok(identify)
        }
    }
}
