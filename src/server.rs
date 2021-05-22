use actix_files::{Files, NamedFile};
use actix_web::Result;
use actix_web::{web, App, HttpRequest, HttpServer};
use std::io;
use std::path::PathBuf;

use crate::Ploog;
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
pub async fn server(ploog: Ploog) -> io::Result<()> {
    let app = ploog.inner.clone();
    if app.serve || app.console {
        let server = HttpServer::new(move || {
            let mut actix = App::new();
            let generated = generate();
            if app.serve {
                actix = actix
                    .route("/{filename:.*}", web::get().to(index))
                    .service(Files::new("/public", ".").show_files_listing());
            }
            if app.console {
                actix = actix.service(ResourceFiles::new("/", generated));
            }
            actix
        });
        server.bind(app.address)?.run().await
    } else {
        Ok(())
    }
}
