use std::fs;
use std::io::{Result, Write};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::{TempFile, TempFileConfig};
use actix_multipart::form::text::Text;
use actix_web::{App, HttpResponse, HttpServer, middleware, post, Responder, web};
use log::{debug, error};
use openssl::hash::{hash, MessageDigest};


use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct AEDatabase{
    csrs: Vec<CSRDatabase>,
    crts: Vec<CRTDatabase>,
}
#[derive(Serialize, Deserialize, Debug)]
struct CSRDatabase{
    email: String,
    csr_path: String,
    otp: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct CRTDatabase{
    email: String,
    crt_path: String,
    otp_revoc: String,
}

#[derive(Debug, MultipartForm)]
struct CSRFormRequest {
    email: Text<String>,
    csr: TempFile,
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



    HttpResponse::Ok().json(
        json!({
            "status": "ok",
            "message": "csr valid"
        })
    )
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