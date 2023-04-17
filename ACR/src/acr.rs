use std::fs;
use std::io::Write;

use openssl::asn1::{Asn1Integer, Asn1Time};
use openssl::bn::BigNum;
use openssl::ec::{EcGroup, EcKey};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{PKey, Private};
use openssl::x509::{X509, X509Builder, X509NameBuilder};
use openssl::x509::extension::{AuthorityKeyIdentifier, BasicConstraints, KeyUsage, SubjectKeyIdentifier};

use crate::acrconfig::AcrConfig;

pub fn generate_acr(config: &AcrConfig) -> Result<(), String> {
    // Don't generate certificate if it already exists
    if fs::metadata("ca-root.crt").is_ok() {
        return Err("Certificate already exists".to_string());
    }

    // Generate key pair
    println!("Generating key pair");
    let key = generate_key()?;
    // Generate Selfsigned certificate
    println!("Generating selfsigned certificate");
    let certificate = generate_selfsigned_certificate(&config, &key)?;
    // Save certificate to file
    println!("Saving certificate to file");
    let pem_certificate = certificate.to_pem().map_err(|e| e.to_string())?;
    fs::File::create("ca-root.crt").map_err(|e| e.to_string())?
        .write_all(&pem_certificate).map_err(|e| e.to_string())?;
    // Save key to file
    println!("Saving key to file");
    let pem_key = key.private_key_to_pem_pkcs8().map_err(|e| e.to_string())?;
    fs::File::create("ca-root.key.pem").map_err(|e| e.to_string())?
        .write_all(&pem_key).map_err(|e| e.to_string())?;
    Ok(())
}

fn generate_key() -> Result<PKey<Private>, String> {
    let group = EcGroup::from_curve_name(Nid::BRAINPOOL_P384R1).map_err(|e| e.to_string())?;
    let key = EcKey::generate(&group).map_err(|e| e.to_string())?;
    PKey::from_ec_key(key).map_err(|e| e.to_string())
}

fn generate_selfsigned_certificate(config: &AcrConfig, key: &PKey<Private>) -> Result<X509, String> {
    let mut x509_name = X509NameBuilder::new().map_err(|e| e.to_string())?;
    x509_name.append_entry_by_text("C", &config.country).map_err(|e| e.to_string())?;
    x509_name.append_entry_by_text("CN", &config.common_name).map_err(|e| e.to_string())?;
    let x509_name = x509_name.build();

    let mut builder = X509Builder::new().map_err(|e| e.to_string())?;
    builder.set_version(2).map_err(|e| e.to_string())?;
    builder.set_issuer_name(&x509_name).map_err(|e| e.to_string())?;
    builder.set_subject_name(&x509_name).map_err(|e| e.to_string())?;

    builder.set_not_before(
        &&Asn1Time::days_from_now(0).map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;
    builder.set_not_after(
        &&Asn1Time::days_from_now(config.valid_time).map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;

    builder.set_pubkey(key.as_ref()).map_err(|e| e.to_string())?;



    builder.append_extension(
        BasicConstraints::new()
            .critical()
            .ca()
            .pathlen(config.path_len)
            .build().map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;

    builder.append_extension(
        KeyUsage::new()
            .digital_signature()
            .crl_sign()
            .key_cert_sign()
            .build().map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;

    builder.set_serial_number(
        &&Asn1Integer::from_bn(
            &&BigNum::from_u32(1).map_err(|e| e.to_string())?
        ).map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;

    builder.append_extension(
        SubjectKeyIdentifier::new()
            .build(&builder.x509v3_context(None, None)
            ).map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;

    builder.append_extension(
        AuthorityKeyIdentifier::new()
            .keyid(true)
            .build(
                &builder.x509v3_context(None, None)
            ).map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;

    builder.sign(
        key.as_ref(),
        MessageDigest::sha384(),
    ).map_err(|e| e.to_string())?;

    Ok(builder.build())
}