mod cli;
mod live;
mod parser;
mod server;

use std::{path::PathBuf, sync::Arc};

use clap::ArgMatches;
use live::Watch;
use parser::*;
use server::*;

#[derive(Clone)]
pub(crate) struct PloogInner {
    output_path: PathBuf,
    source_path: PathBuf,
    watch: bool,
    serve: bool,
    console: bool,
    html_style: bool,
}

pub struct Ploog {
    inner: Arc<PloogInner>,
}

impl From<ArgMatches> for Ploog {
    fn from(args: ArgMatches) -> Self {
        Ploog {
            inner: Arc::new(PloogInner {
                output_path: PathBuf::from(&args.value_of("output").expect("No output dir.")),
                source_path: PathBuf::from(&args.value_of("source").expect("No source dir.")),
                watch: args.is_present("watch"),
                serve: args.is_present("server"),
                console: args.is_present("console"),
                html_style: args.is_present("html_slugs"),
            }),
        }
    }
}

fn main() -> ParserResult<()> {
    let matches = cli::matches();

    // TODO: live reload webpack dev server
    // TODO: preact impl
    // TODO: logos impl?
    // TODO: fontawesome impl
    let app: Ploog = matches.into();

    // Removing this let binding causes hot reloading to not work.
    #[allow(unused_variables)]
    let watch = app.watch().unwrap();

    server(app)?;
    Ok(())
}
