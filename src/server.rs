use actix_files::NamedFile;
use actix_web::Result;
use actix_web::{web, App, HttpRequest, HttpServer};
use std::io;
use std::path::PathBuf;

use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
pub async fn server(serve: bool, console: bool) -> io::Result<()> {
    if serve || console {
        let server = HttpServer::new(move || {  
            let mut app = App::new();
            let generated = generate();

            if serve {
                app = app.route("/{filename:.*}", web::get().to(index));
            }
            if console {
                app = app.service(ResourceFiles::new("/", generated));
            }
            app
        });
        server.bind("127.0.0.1:8080")?.run().await
    } else {
        Ok(())
    }
}
