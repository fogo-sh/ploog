mod parser;
mod server;

use std::path::Path;

use parser::*;
use server::*;

fn main() -> ParserResult<()> {
    let source_list = discover_sources(Path::new("posts"))?;
    let read_sources = read_sources(source_list)?;
    let posts = parse_sources(read_sources)?;
    // TODO: handle true/false via cli
    generate_site(posts, true)?;
    // TODO: handle server via cli
    // TODO: handle error in `ParserResult`
    server();
    Ok(())
}
