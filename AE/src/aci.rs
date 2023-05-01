use std::fmt::format;
use std::{fs, io};
use std::io::{Error, Write};
use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::hash::MessageDigest;
use openssl::rand::rand_bytes;
use openssl::x509::{X509, X509ReqRef};
use openssl::x509::extension::{AuthorityKeyIdentifier, ExtendedKeyUsage, KeyUsage, SubjectAlternativeName, SubjectKeyIdentifier};
use crate::config::{AEDatabase, CRTDatabase};

pub fn create_user_crt(email: &str, user_csr: &X509ReqRef) -> Result<CRTDatabase, Error> {
    let user_crt = generate_user_crt(user_csr)?;
    let crt_id = user_crt.digest(MessageDigest::sha256())?;
    let crt_id = crt_id.iter().map(|x| format!("{:02x}", x)).collect::<String>();
    let crt_pem = user_crt.to_pem()?;
    let crt_path = format!("./certs/{}.crt", crt_id);

    fs::File::create(&crt_path)?.write_all(&crt_pem)?;

    Ok(CRTDatabase {
        email: email.to_string(),
        crt_id,
        crt_path,
        otp_revoc: generate_otp(40)?,
    })
}

pub fn generate_otp(len: usize) -> io::Result<String> {
    let mut buf = vec![0u8; len];
    rand_bytes(&mut buf).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(buf.iter().map(|x| format!("{}", x % 10)).collect::<String>())
}

fn generate_user_crt(user_csr: &X509ReqRef) -> Result<X509, Error> {
    let ica_crt = fs::read_to_string("./aci/aci.crt")?;
    let ica_crt = X509::from_pem(ica_crt.as_bytes())?;
    let ica_key = fs::read_to_string("./aci/aci.key")?;
    let ica_key = openssl::pkey::PKey::private_key_from_pem(ica_key.as_bytes())?;
    let mut user_crt = X509::builder()?;

    user_crt.set_version(2)?;
    user_crt.set_subject_name(user_csr.subject_name())?;
    user_crt.set_issuer_name(ica_crt.subject_name())?;
    user_crt.set_pubkey(user_csr.public_key()?.as_ref())?;

    user_crt.set_not_before(
        &Asn1Time::days_from_now(0)?.as_ref()
    )?;
    user_crt.set_not_after(
        &Asn1Time::days_from_now(30 * 3)?.as_ref()
    )?;

    // Random serial number
    let mut serial_buffer = [0u8; 20];
    openssl::rand::rand_bytes(&mut serial_buffer)?;
    user_crt.set_serial_number(
        BigNum::from_slice(&serial_buffer)?.to_asn1_integer()?.as_ref()
    )?;

    // Add extensions
    user_crt.append_extension(
        KeyUsage::new()
            .digital_signature()
            .build()?
    )?;
    user_crt.append_extension(
        ExtendedKeyUsage::new()
            .email_protection()
            .build()?
    )?;

    // user_crt.append_extension(
    //     SubjectKeyIdentifier::new()
    //         .build(&user_crt.x509v3_context(None, None))?
    // )?;
    // user_crt.append_extension(
    //     AuthorityKeyIdentifier::new()
    //         .keyid(true)
    //         .build(&user_crt.x509v3_context(Some(ica_crt.as_ref()), None))?
    // )?;


    user_crt.sign(ica_key.as_ref(), MessageDigest::sha256())?;

    Ok(user_crt.build())
}