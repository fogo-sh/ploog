use actix_files::Files;
use actix_web::{web, App, HttpServer};
use std::io;

mod console;
mod generator;

use crate::Ploog;

#[actix_web::main]
pub async fn server(ploog: Ploog) -> io::Result<()> {
    let app = ploog.inner.clone();
    if app.serve || app.console {
        let server = HttpServer::new(move || {
            let mut actix = App::new();
            if app.serve {
                actix = actix.service(
                    actix_web::web::scope("/preview")
                        .route("/{filename:.*}", web::get().to(generator::index))
                        .service(Files::new("/", ".").show_files_listing()),
                );
            }
            if app.console {
                actix = actix.service(
                    actix_web::web::scope("/console")
                        .service(
                            actix_web::web::scope("/api")
                                .route("/dir", web::get().to(console::dir)),
                        )
                        .service(console::service()),
                );
            }
            actix
        });
        server.bind("127.0.0.1:8080")?.run().await
    } else {
        Ok(())
    }
}
