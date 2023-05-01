use std::{fs, io};
use std::io::{Error, ErrorKind, Read, Result, Write};
use actix_cors::Cors;

use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::{TempFile, TempFileConfig};
use actix_multipart::form::text::Text;
use actix_web::{App, HttpResponse, HttpServer, middleware, post, Responder, web};
use log::{debug, error};
use openssl::hash::{hash, MessageDigest};
use openssl::nid::Nid;
use openssl::rand::rand_bytes;
use openssl::x509::X509Req;
use serde::Deserialize;
use serde_json::json;
use ae::aci;
use ae::aci::generate_otp;

use ae::config::{AEDatabase, CSRDatabase};
use ae::mail;
use crate::mail::mail;
#[derive(Debug, MultipartForm)]
struct CSRFormRequest {
    email: Text<String>,
    csr: TempFile,
}

#[derive(Debug, Deserialize)]
struct CSRFormValidation {
    otp: String,
    email: String,
}

#[post("/csr/validation")]
async fn csr_validation(
    form: web::Json<CSRFormValidation>
) -> impl Responder {
    let form = form.into_inner();
    debug!("csr validation: {:?}", form);

    let db = AEDatabase::get();
    if let Err(e) = db {
        error!("Error while getting database: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let mut db = db.unwrap();
    let csr_db = db.get_csr(&form.email);
    if let None = csr_db {
        error!("Error while getting csr: {}", "csr not found");
        return HttpResponse::BadRequest().json(
            json!({
                "status": "error",
                "message": "csr not found"
            })
        );
    }
    let csr_db = csr_db.unwrap();
    if csr_db.otp != form.otp {
        error!("Error while getting csr: {}", "otp is not valid");
        return HttpResponse::BadRequest().json(
            json!({
                "status": "error",
                "message": "otp is not valid"
            })
        );
    }
    // load csr
    let csr = fs::read_to_string(&csr_db.csr_path);
    if let Err(e) = csr {
        error!("Error while reading csr: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let csr = csr.unwrap();
    let csr = X509Req::from_pem(csr.as_bytes());
    if let Err(e) = csr {
        error!("Error while parsing csr: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let csr = csr.unwrap();

    // Create crt
    let crt = aci::create_user_crt(&form.email, &csr, &db);
    if let Err(e) = crt {
        error!("Error while creating crt: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let crt = crt.unwrap();
    let res = db.clone().add_crt(&crt);
    if let Err(e) = res {
        error!("Error while adding crt: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    // Remove crs
    let res = fs::remove_file(&csr_db.csr_path);
    if let Err(e) = res {
        error!("Error while removing csr: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let res = db.remove_csr(&form.email);
    if let Err(e) = res {
        error!("Error while removing csr: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }


    HttpResponse::Ok().json(
        json!({
            "status": "ok",
            "message": "otp is valid",
            "otp_revok": &crt.otp_revoc,
            "crt_id": &crt.crt_id,
        })
    )
}

#[post("/csr/request")]
async fn csr_request(
    MultipartForm(form): MultipartForm<CSRFormRequest>
) -> impl Responder {
    debug!("csr request: {:?}", form);

    let csr_name = hash(MessageDigest::sha256(), form.email.as_bytes());
    if let Err(e) = csr_name {
        error!("Error while hashing email: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let csr_name = csr_name.unwrap();
    let csr_name = csr_name.iter().map(|x| format!("{:02x}", x)).collect::<String>();
    //validate csr
    let csr_path = form.csr.file.path();
    let bytes = fs::read_to_string(csr_path);
    if let Err(e) = bytes {
        error!("Error while reading csr: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let csr = X509Req::from_pem(bytes.unwrap().as_bytes());
    if let Err(e) = csr {
        error!("Error while parsing csr: {}", e);
        return HttpResponse::BadRequest().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let csr = csr.unwrap();
    let csr_verif_res = verify_csr(&form.email, &csr);
    if let Err(e) = csr_verif_res {
        error!("Error while verifying csr: {}", e);
        return HttpResponse::BadRequest().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let csr_verif_res = csr_verif_res.unwrap();
    if !csr_verif_res {
        error!("Error while verifying csr: {}", "csr verification failed");
        return HttpResponse::BadRequest().json(
            json!({
                "status": "error",
                "message": "csr verification failed"
            })
        );
    }

    //save csr to file
    let res = form.csr.file.persist(format!("./csr/{csr_name}.csr"));
    if let Err(e) = res {
        error!("Error while saving csr: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    debug!("csr saved to file: ./csr/{}.csr", csr_name);

    //save csr to database
    let db = AEDatabase::get();
    if let Err(e) = db {
        error!("Error while getting database: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let mut db = db.unwrap();
    let otp = generate_otp(6);
    if let Err(e) = otp {
        error!("Error while generating otp: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    let otp = otp.unwrap();
    let res = db.add_csr(
        CSRDatabase {
            email: form.email.to_string(),
            csr_path: format!("./csr/{csr_name}.csr", csr_name = csr_name),
            otp: otp.to_string(),
        }
    );
    if let Err(e) = res {
        error!("Error while saving csr to database: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }

    //send otp to email using mail.rs
    mail(&**form.email, otp.to_string()).await;



    HttpResponse::Ok().json(
        json!({
            "status": "ok",
            "message": "csr valid"
        })
    )
}

fn verify_csr(email: &str, csr: &X509Req) -> Result<bool> {
    let public_key = csr.public_key()?;
    let ec_public_key = public_key.ec_key();
    if ec_public_key.is_err() {
        return Ok(false);
    }
    let ec_public_key = ec_public_key.unwrap();
    // Verify key group
    let curve_name = ec_public_key.group().curve_name().ok_or(Error::new(ErrorKind::Other, "curve name not found"))?;
    if curve_name != Nid::BRAINPOOL_P256R1 {
        return Ok(false);
    }

    // verify csr signature
    if !csr.verify(&public_key)? {
        return Ok(false);
    }

    // verify email
    let subject = csr.subject_name();
    let subject_email = subject.entries_by_nid(Nid::PKCS9_EMAILADDRESS).next();
    if subject_email.is_none() {
        return Ok(false);
    }
    let subject_email = subject_email.unwrap();
    let subject_email = subject_email.data().as_utf8()?;
    if subject_email.to_string() != email {
        return Ok(false);
    }
    Ok(true)
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("creating temporary upload directory");
    fs::create_dir_all("./tmp")?;

    if fs::metadata("./csr").is_err() {
        log::info!("creating csr directory");
        fs::create_dir_all("./csr")?;
    }
    if fs::metadata("./certs").is_err() {
        log::info!("creating certs directory");
        fs::create_dir_all("./certs")?;
    }

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(
                web::scope("/api")
                    .service(csr_request)
                    .service(csr_validation)
            )
    })
        .bind(("0.0.0.0", 8740))?
        .workers(2)
        .run()
        .await
}