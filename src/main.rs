use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::fs::{self, read_to_string};
use std::io;

use std::path::{Path, PathBuf};

fn to_html(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[derive(Deserialize, Debug, PartialEq)]
struct Metadata {
    title: String,
}

impl Metadata {
    fn new(title: &str) -> Metadata {
        Metadata {
            title: title.to_string(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
struct TomlMd {
    metadata: Option<Metadata>,
    post_html: String,
}

impl TomlMd {
    fn new(metadata: Option<Metadata>, post_html: &str) -> TomlMd {
        TomlMd {
            metadata,
            post_html: post_html.to_string(),
        }
    }

    fn parse(string: &String) -> Result<TomlMd, toml::de::Error> {
        let first_line = string.lines().next();
        let mut sections = string.split("---").into_iter();
        let metadata: Option<Metadata> = if let Some(line) = first_line {
            if "---" == line {
                if let Some(toml) = sections.nth(1) {
                    Some(toml::from_str(toml)?)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        let post_html = {
            if let Some(md) = sections.nth(0) {
                to_html(md)
            } else {
                to_html(string)
            }
        };
        Ok(TomlMd {
            metadata,
            post_html,
        })
    }
}

fn parse_sources(sources: Vec<String>) -> Result<Vec<TomlMd>, toml::de::Error> {
    sources.iter().map(|string| TomlMd::parse(string)).collect()
}

fn read_sources(sources: Vec<PathBuf>) -> io::Result<Vec<String>> {
    sources.iter().map(|path| read_to_string(path)).collect()
}

fn discover_sources(path: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    Ok(entries)
}

fn main() -> io::Result<()> {
    let source_list = discover_sources(Path::new("./posts"))?;
    let read_sources = read_sources(source_list);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, Write};
    use std::writeln;
    use tempfile::{tempdir, TempDir};

    const SOURCES_COUNT: usize = 2;
    const SOURCES_HTML: (&str, &str) = (
        "<h1>I</h1>\n<h2>am</h2>\n<h3>Ploog</h3>\n<p>~Based~</p>\n",
        "<h1>I</h1>\n<h2>am</h2>\n<h3>Ploog</h3>\n<p>~Based~</p>\n",
    );
    const SOURCES: (&str, &str) = (
        r#"---
title = 'Hello world.'
---
# I
## am
### Ploog
~Based~"#,
        r#"
# I
## am
### Ploog
~Based~"#,
    );
    const INVALID_SOURCE: &str = r#"---
title 'Hello world.'
---"#;

    fn expected_sources_parsed() -> (TomlMd, TomlMd) {
        let (source1_html, source2_html) = SOURCES_HTML;
        (
            TomlMd::new(Some(Metadata::new("Hello world.")), source1_html),
            TomlMd::new(None, source2_html),
        )
    }

    fn example_posts() -> io::Result<(TempDir, File, File)> {
        let (source1, source2) = SOURCES;

        let dir = tempdir()?;
        let file_path1 = dir.path().join("./post1.md");
        let mut post1 = File::create(file_path1)?;
        writeln!(post1, "{}", source1)?;

        let file_path2 = dir.path().join("./post2.md");
        let mut post2 = File::create(file_path2)?;
        writeln!(post2, "{}", source2)?;
        Ok((dir, post1, post2))
    }

    fn example_load_posts() -> io::Result<Vec<String>> {
        let (dir, _post1, _post2) = example_posts()?;
        let sources = discover_sources(dir.path())?;
        read_sources(sources)
    }

    #[test]
    fn test_load_markdown() -> io::Result<()> {
        let results = example_load_posts()?;
        let (source1, source2) = SOURCES;
        assert_eq!(&results.len(), &SOURCES_COUNT);
        assert_eq!(&results[0].trim_end(), &source1);
        assert_eq!(&results[1].trim_end(), &source2);
        Ok(())
    }

    #[test]
    fn test_to_tomlmd() -> Result<(), toml::de::Error> {
        let (source1, source2) = SOURCES;
        let (obj1, obj2) = expected_sources_parsed();

        assert_eq!(obj1, TomlMd::parse(&source1.to_string())?,);

        assert_eq!(obj2, TomlMd::parse(&source2.to_string())?,);
        Ok(())
    }

    #[test]
    fn test_invalid_toml() -> Result<(), toml::de::Error> {
        let source = INVALID_SOURCE;

        match TomlMd::parse(&source.to_string()) {
            Ok(_) => assert!(false, "Invalid toml was parsed as Ok()"),
            _ => assert!(true),
        }

        Ok(())
    }

    #[test]
    fn test_parse_tomlmd() -> io::Result<()> {
        let (dir, _post1, _post2) = example_posts()?;
        let sources = discover_sources(dir.path())?;
        let sources = read_sources(sources).expect("Reading failed.");
        let sources = parse_sources(sources).expect("Parse failed.");
        let (obj1, obj2) = expected_sources_parsed();

        assert_eq!(vec![obj1, obj2], sources);

        Ok(())
    }
}
