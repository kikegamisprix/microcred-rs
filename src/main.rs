use microcred_rs::{
    issuer::CredentialIssuer, verifier::CredentialVerifier, Evidence, EvidenceType, Skill,
    SkillLevel, Subject,
};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Microcredential System Demo ===\n");

    let issuer = CredentialIssuer::new(
        "Rust University".to_string(),
        "https://rust-university.edu".to_string(),
    );

    println!("Created issuer: {}", issuer.get_issuer_info().name);

    let subject = Subject {
        id: Uuid::new_v4(),
        name: "Alice Developer".to_string(),
        email: "alice@example.com".to_string(),
    };

    let skill = Skill {
        id: "rust-programming".to_string(),
        name: "Rust Programming".to_string(),
        description: "Proficiency in Rust programming language".to_string(),
        level: SkillLevel::Advanced,
    };

    let evidence = vec![
        Evidence {
            id: Uuid::new_v4(),
            name: "Web Server Project".to_string(),
            description: "Built a high-performance web server using Tokio".to_string(),
            url: "https://github.com/alice/rust-webserver".to_string(),
            evidence_type: EvidenceType::Project,
        },
        Evidence {
            id: Uuid::new_v4(),
            name: "Rust Certification Assessment".to_string(),
            description: "Passed advanced Rust programming assessment".to_string(),
            url: "https://assessments.rust-university.edu/alice/cert-123".to_string(),
            evidence_type: EvidenceType::Assessment,
        },
    ];

    let credential = issuer.issue_credential(subject, skill, evidence, None)?;

    println!("Issued credential for: {}", credential.subject.name);
    println!("Skill: {} (Level: {:?})", credential.skill.name, credential.skill.level);
    println!("Evidence count: {}", credential.evidence.len());
    println!("Credential ID: {}", credential.id);
    println!("Is valid: {}", credential.is_valid());

    let mut verifier = CredentialVerifier::new();
    verifier.add_trusted_issuer(issuer.get_issuer_info().clone());

    match verifier.verify_credential(&credential) {
        Ok(is_valid) => {
            println!("\n=== Verification Result ===");
            println!("Credential verification: {}", if is_valid { "VALID" } else { "INVALID" });
        }
        Err(e) => {
            println!("Verification failed: {}", e);
        }
    }

    println!("\n=== Credential JSON ===");
    let json = serde_json::to_string_pretty(&credential)?;
    println!("{}", json);

    Ok(())
}
