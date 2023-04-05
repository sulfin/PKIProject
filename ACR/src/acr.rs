use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::pkey::Private;
use openssl::x509::{X509, X509Builder, X509NameBuilder};

pub fn generate_acr() -> Result<(), String> {
    // Generate key pair
    let key = generate_key()?;
    // Generate Selfsigned certificate

    // Save certificate to file
}

fn generate_key() -> Result<EcKey<Private>, String> {
    let nid = Nid::SECP384R1;
    let group = EcGroup::from_curve_name(nid).map_err(|e| e.to_string())?;
    EcKey::generate(&group).map_err(|e| e.to_string())
}

fn generate_selfsigned_certificate(key: EcKey<Private>) -> Result<X509, String> {
    let mut x509_name = X509NameBuilder::new().map_err(|e| e.to_string())?;
    x509_name.append_entry_by_text("C", "FR").map_err(|e| e.to_string())?;
    x509_name.append_entry_by_text("CN", "Pas Un Virus Sign").map_err(|e| e.to_string())?;
    let x509_name = x509_name.build();

    let mut builder = X509Builder::new().map_err(|e| e.to_string())?;
    builder.set_issuer_name(&x509_name).map_err(|e| e.to_string())?;

    Ok(builder.build())
}