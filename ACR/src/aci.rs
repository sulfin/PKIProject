use std::fs;
use std::io::Write;
use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::ec::{EcGroup, EcKey};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{PKey, Private};
use openssl::x509::{X509, X509NameBuilder};
use openssl::x509::extension::{AuthorityKeyIdentifier, BasicConstraints, KeyUsage, SubjectKeyIdentifier};

pub fn generate_aci() -> Result<(), String> {
    let aci_key = generate_aci_key()?;
    let root_key = load_root_key()?;
    let root_certificate = load_root_certificate()?;

    println!("Generating aci certificate");
    let aci_certificate = generate_aci_certificate(&aci_key, &root_key, &root_certificate)?;

    //save aci certificate
    println!("Saving aci certificate to file");
    let pem_certificate = aci_certificate.to_pem().map_err(|e| e.to_string())?;
    fs::File::create("aci.crt").map_err(|e| e.to_string())?
        .write_all(&pem_certificate).map_err(|e| e.to_string())?;
    //save key to file
    println!("Saving aci key to file");
    let pem_key = aci_key.private_key_to_pem_pkcs8().map_err(|e| e.to_string())?;
    fs::File::create("aci.key").map_err(|e| e.to_string())?
        .write_all(&pem_key).map_err(|e| e.to_string())?;

    Ok(())
}

fn generate_aci_key() -> Result<PKey<Private>, String> {
    let group = EcGroup::from_curve_name(Nid::BRAINPOOL_P256R1).map_err(|e| e.to_string())?;
    let key = EcKey::generate(&group).map_err(|e| e.to_string())?;
    PKey::from_ec_key(key).map_err(|e| e.to_string())
}

fn load_root_key() -> Result<PKey<Private>, String> {
    let pem = fs::read_to_string("ca-root.key.pem").map_err(|e| e.to_string())?;
    let key = PKey::private_key_from_pem(&pem.as_bytes()).map_err(|e| e.to_string())?;
    Ok(key)
}

fn load_root_certificate() -> Result<X509, String> {
    let pem = fs::read_to_string("ca-root.crt").map_err(|e| e.to_string())?;
    let certificate = X509::from_pem(&pem.as_bytes()).map_err(|e| e.to_string())?;
    Ok(certificate)
}

fn generate_aci_certificate(aci_key: &PKey<Private>, root_key: &PKey<Private>, root_certificate: &X509) -> Result<X509, String> {
    let mut subj_name = X509NameBuilder::new().map_err(|e| e.to_string())?;
    subj_name.append_entry_by_text("C", "FR").map_err(|e| e.to_string())?;
    subj_name.append_entry_by_text("O", "ISEN").map_err(|e| e.to_string())?;
    subj_name.append_entry_by_text("CN", "Pas Un Virus Intermediate 1").map_err(|e| e.to_string())?;
    let subj_name = subj_name.build();

    let mut aci_certificate = X509::builder().map_err(|e| e.to_string())?;
    aci_certificate.set_version(2).map_err(|e| e.to_string())?;
    aci_certificate.set_subject_name(&subj_name).map_err(|e| e.to_string())?;
    aci_certificate.set_issuer_name(root_certificate.subject_name()).map_err(|e| e.to_string())?;
    aci_certificate.set_serial_number(
        BigNum::from_u32(1).map_err(|e| e.to_string())?.to_asn1_integer().map_err(|e| e.to_string())?.as_ref()
    ).map_err(|e| e.to_string())?;

    aci_certificate.set_not_before(
        Asn1Time::days_from_now(0).map_err(|e| e.to_string())?.as_ref()
    ).map_err(|e| e.to_string())?;
    aci_certificate.set_not_after(
        Asn1Time::days_from_now(365 * 4).map_err(|e| e.to_string())?.as_ref()
    ).map_err(|e| e.to_string())?;

    aci_certificate.set_pubkey(aci_key).map_err(|e| e.to_string())?;

    aci_certificate.append_extension(
        BasicConstraints::new()
            .critical()
            .ca()
            .pathlen(0)
            .build()
            .map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;

    aci_certificate.append_extension(
        KeyUsage::new()
            .critical()
            .digital_signature()
            .key_cert_sign()
            .crl_sign()
            .build()
            .map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;

    aci_certificate.append_extension(
        SubjectKeyIdentifier::new()
            .build(&aci_certificate.x509v3_context(None, None))
            .map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;

    aci_certificate.append_extension(
        AuthorityKeyIdentifier::new()
            .keyid(true)
            .issuer(true)
            .build(&aci_certificate.x509v3_context(Some(root_certificate), None))
            .map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())?;

    aci_certificate.sign(
        root_key.as_ref(),
        MessageDigest::sha256()).map_err(|e| e.to_string())?;

    Ok(aci_certificate.build())
}