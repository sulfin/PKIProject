use std::{fs, io};
use std::io::{Error, ErrorKind, Read, Result, Write};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::{TempFile, TempFileConfig};
use actix_multipart::form::text::Text;
use actix_web::{App, HttpResponse, HttpServer, middleware, post, Responder, web};
use log::{debug, error};
use openssl::hash::{hash, MessageDigest};
use openssl::nid::Nid;
use openssl::rand::rand_bytes;
use openssl::x509::X509Req;


use serde_json::json;
use ae::config::{AEDatabase, CSRDatabase};


#[derive(Debug, MultipartForm)]
struct CSRFormRequest {
    email: Text<String>,
    csr: TempFile,
}
#[post("/form-email")]
//call mail.rs to send an email with the otp
async fn form_email(
    MultipartForm(form): MultipartForm<CSRFormRequest>
) -> impl Responder {
    debug!("csr request: {:?}", form);
    let mut db = AEDatabase::get();
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
            csr_path: format!("./csr/{csr_name}.csr"),
            otp: otp.to_string(),
        }
    );
    if let Err(e) = res {
        error!("Error while adding csr to database: {}", e);
        return HttpResponse::InternalServerError().json(
            json!({
                "status": "error",
                "message": e.to_string()
            })
        );
    }
    debug!("csr added to database: {:?}", res.unwrap());
    HttpResponse::Ok().json(
        json!({
            "status": "ok",
            "message": "csr added to database"
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
    let mut db = AEDatabase::get();
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

    //send otp to email

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
    let subject_email = subject.entries_by_nid(Nid::COMMONNAME).next();
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

fn generate_otp(len: usize) -> Result<String> {
    let mut buf = vec![0u8; len];
    rand_bytes(&mut buf).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(buf.iter().map(|x| format!("{}", x % 10)).collect::<String>())
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

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(
                web::scope("/api")
                    .service(csr_request)
            )
    })
        .bind(("0.0.0.0", 8740))?
        .workers(2)
        .run()
        .await
}