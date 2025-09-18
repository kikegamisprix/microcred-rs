use crate::crypto::{hash_credential, CryptoKeyPair};
use crate::{Evidence, Issuer, Microcredential, Skill, Subject};
use chrono::{DateTime, Utc};
use serde_json;
use std::error::Error;

pub struct CredentialIssuer {
    issuer_info: Issuer,
    keypair: CryptoKeyPair,
}

impl CredentialIssuer {
    pub fn new(name: String, url: String) -> Self {
        let keypair = CryptoKeyPair::generate();
        let issuer_info = Issuer {
            id: uuid::Uuid::new_v4(),
            name,
            url,
            public_key: keypair.public_key(),
        };

        Self {
            issuer_info,
            keypair,
        }
    }

    pub fn from_existing(
        issuer_info: Issuer,
        secret_key: &[u8],
    ) -> Result<Self, Box<dyn Error>> {
        let keypair = CryptoKeyPair::from_secret_key(secret_key)?;
        Ok(Self {
            issuer_info,
            keypair,
        })
    }

    pub fn issue_credential(
        &self,
        subject: Subject,
        skill: Skill,
        evidence: Vec<Evidence>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<Microcredential, Box<dyn Error>> {
        let mut credential = Microcredential::new(
            self.issuer_info.clone(),
            subject,
            skill,
            evidence,
            expires_at,
        );

        let credential_json = serde_json::to_vec(&credential)?;
        let credential_hash = hash_credential(&credential_json);
        let signature = self.keypair.sign(&credential_hash);

        credential.signature = Some(signature);

        Ok(credential)
    }

    pub fn get_issuer_info(&self) -> &Issuer {
        &self.issuer_info
    }

    pub fn get_public_key(&self) -> Vec<u8> {
        self.keypair.public_key()
    }

    pub fn get_secret_key(&self) -> Vec<u8> {
        self.keypair.secret_key()
    }
}