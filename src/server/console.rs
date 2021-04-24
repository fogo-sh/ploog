use actix_web::{web::Json, HttpRequest};
use actix_web_static_files::ResourceFiles;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;

use crate::TomlMd;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Dir {
    children: Vec<Dir>,
    posts: Vec<TomlMd>,
    name: String,
}

pub fn service() -> ResourceFiles {
    let generated = generate();
    ResourceFiles::new("/", generated)
}

pub async fn dir(req: HttpRequest) -> actix_web::Result<Json<Dir>> {
    Ok(Json(Dir {
        children: vec![],
        posts: vec![],
        name: OsString::new().to_str().unwrap().to_string(),
    }))
}
