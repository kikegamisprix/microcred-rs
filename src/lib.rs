use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issuer {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: SkillLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub url: String,
    pub evidence_type: EvidenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    Project,
    Assessment,
    Portfolio,
    Certification,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Microcredential {
    pub id: Uuid,
    pub issuer: Issuer,
    pub subject: Subject,
    pub skill: Skill,
    pub evidence: Vec<Evidence>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
    pub signature: Option<Vec<u8>>,
}

impl Microcredential {
    pub fn new(
        issuer: Issuer,
        subject: Subject,
        skill: Skill,
        evidence: Vec<Evidence>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            issuer,
            subject,
            skill,
            evidence,
            issued_at: Utc::now(),
            expires_at,
            metadata: HashMap::new(),
            signature: None,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired() && self.signature.is_some()
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

pub mod crypto;
pub mod issuer;
pub mod verifier;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::issuer::CredentialIssuer;
    use crate::verifier::CredentialVerifier;
    use chrono::Duration;

    #[test]
    fn test_credential_creation() {
        let issuer_service = CredentialIssuer::new(
            "Test University".to_string(),
            "https://test.edu".to_string(),
        );

        let subject = Subject {
            id: Uuid::new_v4(),
            name: "Test Student".to_string(),
            email: "test@example.com".to_string(),
        };

        let skill = Skill {
            id: "test-skill".to_string(),
            name: "Test Skill".to_string(),
            description: "A test skill".to_string(),
            level: SkillLevel::Intermediate,
        };

        let evidence = vec![Evidence {
            id: Uuid::new_v4(),
            name: "Test Evidence".to_string(),
            description: "Test evidence description".to_string(),
            url: "https://example.com/evidence".to_string(),
            evidence_type: EvidenceType::Project,
        }];

        let credential = issuer_service
            .issue_credential(subject, skill, evidence, None)
            .unwrap();

        assert!(credential.is_valid());
        assert!(!credential.is_expired());
        assert!(credential.signature.is_some());
    }

    #[test]
    fn test_credential_verification() {
        let issuer_service = CredentialIssuer::new(
            "Test University".to_string(),
            "https://test.edu".to_string(),
        );

        let subject = Subject {
            id: Uuid::new_v4(),
            name: "Test Student".to_string(),
            email: "test@example.com".to_string(),
        };

        let skill = Skill {
            id: "test-skill".to_string(),
            name: "Test Skill".to_string(),
            description: "A test skill".to_string(),
            level: SkillLevel::Advanced,
        };

        let evidence = vec![Evidence {
            id: Uuid::new_v4(),
            name: "Test Evidence".to_string(),
            description: "Test evidence description".to_string(),
            url: "https://example.com/evidence".to_string(),
            evidence_type: EvidenceType::Assessment,
        }];

        let credential = issuer_service
            .issue_credential(subject, skill, evidence, None)
            .unwrap();

        let mut verifier = CredentialVerifier::new();
        verifier.add_trusted_issuer(issuer_service.get_issuer_info().clone());

        let verification_result = verifier.verify_credential(&credential);
        assert!(verification_result.is_ok());
        assert!(verification_result.unwrap());
    }

    #[test]
    fn test_expired_credential() {
        let issuer_service = CredentialIssuer::new(
            "Test University".to_string(),
            "https://test.edu".to_string(),
        );

        let subject = Subject {
            id: Uuid::new_v4(),
            name: "Test Student".to_string(),
            email: "test@example.com".to_string(),
        };

        let skill = Skill {
            id: "test-skill".to_string(),
            name: "Test Skill".to_string(),
            description: "A test skill".to_string(),
            level: SkillLevel::Expert,
        };

        let evidence = vec![Evidence {
            id: Uuid::new_v4(),
            name: "Test Evidence".to_string(),
            description: "Test evidence description".to_string(),
            url: "https://example.com/evidence".to_string(),
            evidence_type: EvidenceType::Certification,
        }];

        let past_time = Utc::now() - Duration::days(1);
        let credential = issuer_service
            .issue_credential(subject, skill, evidence, Some(past_time))
            .unwrap();

        assert!(credential.is_expired());
        assert!(!credential.is_valid());

        let mut verifier = CredentialVerifier::new();
        verifier.add_trusted_issuer(issuer_service.get_issuer_info().clone());

        let verification_result = verifier.verify_credential(&credential);
        assert!(verification_result.is_err());
    }
}