use crate::crypto::{hash_credential, verify_signature};
use crate::{Issuer, Microcredential};
use serde_json;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VerificationError {
    SerializationError(String),
    InvalidSignature,
    ExpiredCredential,
    MissingSignature,
    TrustedIssuerNotFound,
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerificationError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            VerificationError::InvalidSignature => write!(f, "Invalid signature"),
            VerificationError::ExpiredCredential => write!(f, "Credential has expired"),
            VerificationError::MissingSignature => write!(f, "Credential is not signed"),
            VerificationError::TrustedIssuerNotFound => {
                write!(f, "Issuer is not in the trusted list")
            }
        }
    }
}

impl Error for VerificationError {}

pub struct CredentialVerifier {
    trusted_issuers: Vec<Issuer>,
}

impl CredentialVerifier {
    pub fn new() -> Self {
        Self {
            trusted_issuers: Vec::new(),
        }
    }

    pub fn add_trusted_issuer(&mut self, issuer: Issuer) {
        self.trusted_issuers.push(issuer);
    }

    pub fn remove_trusted_issuer(&mut self, issuer_id: &uuid::Uuid) {
        self.trusted_issuers.retain(|issuer| issuer.id != *issuer_id);
    }

    pub fn get_trusted_issuers(&self) -> &[Issuer] {
        &self.trusted_issuers
    }

    pub fn verify_credential(
        &self,
        credential: &Microcredential,
    ) -> Result<bool, VerificationError> {
        if credential.is_expired() {
            return Err(VerificationError::ExpiredCredential);
        }

        let signature = credential
            .signature
            .as_ref()
            .ok_or(VerificationError::MissingSignature)?;

        let trusted_issuer = self
            .trusted_issuers
            .iter()
            .find(|issuer| issuer.id == credential.issuer.id)
            .ok_or(VerificationError::TrustedIssuerNotFound)?;

        let mut credential_for_hash = credential.clone();
        credential_for_hash.signature = None;

        let credential_json = serde_json::to_vec(&credential_for_hash)
            .map_err(|e| VerificationError::SerializationError(e.to_string()))?;

        let credential_hash = hash_credential(&credential_json);

        let is_valid = verify_signature(&trusted_issuer.public_key, &credential_hash, signature)
            .map_err(|_| VerificationError::InvalidSignature)?;

        if !is_valid {
            return Err(VerificationError::InvalidSignature);
        }

        Ok(true)
    }

    pub fn verify_credential_chain(
        &self,
        credentials: &[Microcredential],
    ) -> Result<Vec<bool>, VerificationError> {
        credentials
            .iter()
            .map(|credential| self.verify_credential(credential))
            .collect()
    }
}

impl Default for CredentialVerifier {
    fn default() -> Self {
        Self::new()
    }
}