mod parser;
use std::path::Path;

use parser::*;

fn main() -> ParserResult<()> {
    let source_list = discover_sources(Path::new("posts"))?;
    let read_sources = read_sources(source_list)?;
    let posts = parse_sources(read_sources)?;
    generate_site(posts)?;
    Ok(())
}
