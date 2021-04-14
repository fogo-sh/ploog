use actix_web::{web, App, HttpRequest, HttpServer};
use actix_files::NamedFile;
use actix_web::{Result};
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
