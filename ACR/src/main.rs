use std::ops::Deref;

use actix_web::{App, HttpServer, post, Responder, web};
use log::{debug, LevelFilter};
use simple_logger::SimpleLogger;

use acr::acrconfig::AcrConfig;

#[post("/generate-acr")]
async fn generate_acr(config: web::Data<AcrConfig>) -> impl Responder {
    let mut ret = "ACR generated";
    acr::acr::generate_acr(config.into_inner().deref()).unwrap_or_else(|e| {
        ret = "ACR generation failed";
        println!("Error: {}", e);
    });
    ret
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().with_level(LevelFilter::Info).env().init().unwrap();
    debug!("Debug logging enabled");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AcrConfig::default()))
            .service(generate_acr)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
