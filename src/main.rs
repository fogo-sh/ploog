mod cli;
mod parser;
mod server;

use std::path::Path;

use parser::*;
use server::*;

fn main() -> ParserResult<()> {
    let matches = cli::matches();

    let source_list = discover_sources(Path::new("posts"))?;
    let read_sources = read_sources(source_list)?;
    let posts = parse_sources(read_sources)?;
    let html_style = matches.is_present("html_slugs");
    generate_site(&posts, Some("public".into()), !html_style)?;
    server(matches.is_present("serve"), matches.is_present("console"))?;
    Ok(())
}
