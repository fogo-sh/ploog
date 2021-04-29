use actix_web::{web::Json, HttpRequest};
use actix_web_static_files::ResourceFiles;
use serde::{Deserialize, Serialize};

use std::io;
use std::fs::{self, DirEntry};
use std::path::{ Path, PathBuf };

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    path: PathBuf,
}

impl From<DirEntry> for Entry {
    fn from(entry: DirEntry) -> Entry {
        Entry {
            path: entry.path(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dir {
    children: Vec<Dir>,
    posts: Vec<Entry>,
    path: PathBuf,
}

fn visit_dirs(dir: &Path) -> io::Result<Dir> {
    let mut posts: Vec<Entry> = vec![];
    let mut children: Vec<Dir> = vec![];
    let path: PathBuf = dir.to_path_buf();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                children.push(visit_dirs(&path)?);
            } else {
                posts.push(entry.into());
            }
        }
    }
    Ok(Dir {
        posts,
        children,
        path
    })
}

pub fn service() -> ResourceFiles {
    let generated = generate();
    ResourceFiles::new("/", generated)
}

pub async fn dir(req: HttpRequest) -> actix_web::Result<Json<Dir>> {
    Ok(Json(visit_dirs(Path::new("./public"))?))
}
