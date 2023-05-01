
pub fn crl_check(cert_id:i32, user: String) -> Result<bool, String> {

    while(1){
        if RevokedCert::find_by_id(cert_id).is_err() {
            return Err("Certificat non trouvé".to_string());
        }
        let revoked_cert = RevokedCert::find_by_id(cert_id).unwrap();
        if revoked_cert.revoked {
            return Err("Certificat révoqué".to_string());
        }
        if revoked_cert.user != user {
            return Err("Certificat ne correspond pas à l'utilisateur".to_string());
        }
        else {
            return Ok(true);
        }
    }
}