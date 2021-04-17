use pulldown_cmark::{html, Options, Parser};
use std::fs::{self, create_dir, read_to_string, File};
use std::io::{self, Write};

use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::PloogInner;
use std::cmp::Ordering;

macro_rules! ploog_template {
    () => {
        "<!DOCTYPE html lang=\"en\"><html><head><meta charset=\"UTF-8\"></head><body>{}<body><html>"
    };
}

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
pub struct Metadata {
    pub title: String,
    pub slug: String,
}

impl Metadata {
    pub fn new(title: &str, slug: &str) -> Metadata {
        Metadata {
            title: title.to_string(),
            slug: slug.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum InvalidFileNameKind {
    NotUTF8,
    NoExtension,
}

#[derive(Debug)]
pub enum ParsingErrorKind {
    InvalidFileName(InvalidFileNameKind),
    MissingMetadata,
}

#[derive(Debug)]
pub struct ParsingError {
    inner: ParsingErrorKind,
}

impl ParsingError {
    pub fn new(inner: ParsingErrorKind) -> ParsingError {
        ParsingError { inner }
    }
}

pub type ParserResult<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub struct ParserError {
    toml_error: Option<toml::de::Error>,
    io_error: Option<io::Error>,
    parsing_error: Option<ParsingError>,
}

impl From<io::Error> for ParserError {
    fn from(io_error: io::Error) -> ParserError {
        ParserError {
            toml_error: None,
            io_error: Some(io_error),
            parsing_error: None,
        }
    }
}

impl From<toml::de::Error> for ParserError {
    fn from(toml_error: toml::de::Error) -> ParserError {
        ParserError {
            toml_error: Some(toml_error),
            io_error: None,
            parsing_error: None,
        }
    }
}

impl From<ParsingError> for ParserError {
    fn from(parsing_error: ParsingError) -> ParserError {
        ParserError {
            toml_error: None,
            io_error: None,
            parsing_error: Some(parsing_error),
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "failed to parse")
    }
}

#[derive(Eq, Deserialize, Debug, PartialEq)]
pub struct ParserInput {
    pub input_path: Option<PathBuf>,
    pub string: String,
}

impl Ord for ParserInput {
    fn cmp(&self, other: &Self) -> Ordering {
        self.input_path.cmp(&other.input_path)
    }
}

impl PartialOrd for ParserInput {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ParserInput {
    pub fn new(input_path: Option<PathBuf>, string: String) -> ParserInput {
        ParserInput { input_path, string }
    }
}

fn get_metadata(input_path: Option<PathBuf>, string: &str) -> ParserResult<Metadata> {
    let default_metadata = input_path.map(|p| {
        let out = p
            .file_stem()
            .ok_or_else(|| {
                ParsingError::new(ParsingErrorKind::InvalidFileName(
                    InvalidFileNameKind::NoExtension,
                ))
            })?
            .to_str()
            .ok_or_else(|| {
                ParsingError::new(ParsingErrorKind::InvalidFileName(
                    InvalidFileNameKind::NotUTF8,
                ))
            })?;
        Ok(Metadata::new(out, out))
    });
    let first_line = string.lines().next();
    let mut sections = string.split("---");
    if let Some(line) = first_line {
        if "---" == line {
            if let Some(toml) = sections.nth(1) {
                let toml: Metadata = toml::from_str(toml)?;
                return Ok(toml);
            }
        }
    }

    default_metadata.ok_or_else(|| ParsingError::new(ParsingErrorKind::MissingMetadata))?
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TomlMd {
    pub metadata: Metadata,
    pub post_html: String,
}

impl TomlMd {
    #[allow(dead_code)]
    pub fn new(metadata: Metadata, post_html: &str) -> TomlMd {
        TomlMd {
            metadata,
            post_html: post_html.to_string(),
        }
    }

    pub fn parse(input: ParserInput) -> ParserResult<TomlMd> {
        let ParserInput { input_path, string } = input;
        let metadata: ParserResult<Metadata> = get_metadata(input_path, &string);
        let mut sections = string.split("---");
        let post_html = {
            if let Some(md) = sections.nth(2) {
                to_html(md)
            } else {
                to_html(&string)
            }
        };
        Ok(TomlMd {
            metadata: metadata?,
            post_html: format!(ploog_template!(), post_html),
        })
    }
}

// TODO: Convert these to types producing eachother.
// let generated_site = discover_sources().read().parse()?

pub fn parse_sources(sources: Vec<ParserInput>) -> ParserResult<Vec<TomlMd>> {
    sources.into_iter().map(TomlMd::parse).collect()
}

pub fn read_sources(sources: Vec<PathBuf>) -> ParserResult<Vec<ParserInput>> {
    sources
        .into_iter()
        .map(|path| {
            let read = read_to_string(&path);
            Ok(ParserInput::new(Some(path), read?))
        })
        .collect()
}

pub fn discover_sources(path: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    Ok(entries)
}

pub fn generate_site<'a>(
    posts: &'a [TomlMd],
    dir: Option<&Path>,
    index_style: bool,
) -> ParserResult<&'a [TomlMd]> {
    let path = match dir {
        Some(present) => present,
        None => Path::new("public"),
    };

    match create_dir(&path) {
        Ok(_) => {}
        Err(error) => {
            if error.kind() != io::ErrorKind::AlreadyExists {
                return Err(error.into());
            }
        }
    }

    for TomlMd {
        metadata,
        post_html,
    } in posts
    {
        let mut post = if index_style {
            match create_dir(format!(
                "{}/{}",
                path.to_str().ok_or_else(|| ParsingError::new(
                    ParsingErrorKind::InvalidFileName(InvalidFileNameKind::NotUTF8,)
                ))?,
                &metadata.slug
            )) {
                Ok(_) => {}
                Err(error) => {
                    if error.kind() != io::ErrorKind::AlreadyExists {
                        return Err(error.into());
                    }
                }
            }
            File::create(format!(
                "{}/{}/index.html",
                path.to_str().ok_or_else(|| ParsingError::new(
                    ParsingErrorKind::InvalidFileName(InvalidFileNameKind::NotUTF8,)
                ))?,
                metadata.slug
            ))?
        } else {
            File::create(format!(
                "{}/{}.html",
                path.to_str().ok_or_else(|| ParsingError::new(
                    ParsingErrorKind::InvalidFileName(InvalidFileNameKind::NotUTF8,)
                ))?,
                metadata.slug
            ))?
        };
        writeln!(post, "{}", post_html)?;
    }

    Ok(posts)
}

pub trait GenerateSite {
    fn generate(&self) -> ParserResult<Vec<TomlMd>>;
}

impl GenerateSite for PloogInner {
    fn generate(&self) -> ParserResult<Vec<TomlMd>> {
        let source_list = discover_sources(&self.source_path)?;
        let read_sources = read_sources(source_list)?;
        let posts = parse_sources(read_sources)?;
        let generated = generate_site(&posts, Some(&self.output_path), !self.html_style)?;
        for tomlmd in generated {
            println!(
                "emitted: {} at /{}",
                tomlmd.metadata.title, tomlmd.metadata.slug
            );
        }
        Ok(posts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::sorted;
    use std::fs::File;
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
slug = 'hello'
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
            TomlMd::new(
                Metadata::new("Hello world.", "hello"),
                &format!(ploog_template!(), source1_html),
            ),
            TomlMd::new(
                Metadata::new("post2", "post2"),
                &format!(ploog_template!(), source2_html),
            ),
        )
    }

    fn example_posts() -> io::Result<(TempDir, File, File)> {
        let (source1, source2) = SOURCES;

        let dir = tempdir()?;
        let file_path1 = dir.path().join("post1.md");
        let mut post1 = File::create(file_path1)?;
        writeln!(post1, "{}", source1)?;

        let file_path2 = dir.path().join("post2.md");
        let mut post2 = File::create(file_path2)?;
        writeln!(post2, "{}", source2)?;
        Ok((dir, post1, post2))
    }

    fn example_load_posts() -> ParserResult<Vec<ParserInput>> {
        let (dir, _post1, _post2) = example_posts()?;
        read_sources(discover_sources(dir.path())?)
    }

    fn invalid_post() -> io::Result<(TempDir, File)> {
        let source = INVALID_SOURCE;

        let dir = tempdir()?;

        let file_path = dir.path().join("post2.md");
        let mut post = File::create(file_path)?;
        writeln!(post, "{}", source)?;
        Ok((dir, post))
    }

    fn invalid_load_post() -> ParserResult<Vec<ParserInput>> {
        let (dir, _post1) = invalid_post()?;
        let sources: Vec<PathBuf> = discover_sources(dir.path())?;
        read_sources(sources)
    }

    fn generate_example_site(index_style: bool) -> ParserResult<Vec<fs::DirEntry>> {
        let results = example_load_posts()?;
        let results: Vec<ParserInput> = sorted(results).collect();
        let sources = parse_sources(results)?;
        let dir = tempdir()?;
        generate_site(&sources, Some(dir.path()), index_style)?;
        Ok(fs::read_dir(dir.path())?.collect::<io::Result<Vec<fs::DirEntry>>>()?)
    }

    #[test]
    fn test_load_markdown() -> ParserResult<()> {
        let results = example_load_posts()?;
        let results: Vec<ParserInput> = sorted(results).collect();
        let (source1, source2) = SOURCES;
        assert_eq!(&results.len(), &SOURCES_COUNT);
        assert_eq!(&results[0].string.trim_end(), &source1);
        assert_eq!(&results[1].string.trim_end(), &source2);
        Ok(())
    }

    #[test]
    fn test_generate_html_index_style() -> ParserResult<()> {
        let generate_index_style = generate_example_site(true)?;
        assert_eq!(generate_index_style.len(), 2);
        assert!(generate_index_style
            .iter()
            .map(|e| Ok(e.file_type()?))
            .collect::<ParserResult<Vec<_>>>()?
            .iter()
            .all(|e| e.is_dir()));

        Ok(())
    }

    #[test]
    fn test_generate_html_file_name_style() -> ParserResult<()> {
        let generate_index_style = generate_example_site(false)?;
        assert_eq!(generate_index_style.len(), 2);
        assert!(!generate_index_style
            .iter()
            .map(|e| Ok(e.file_type()?))
            .collect::<ParserResult<Vec<_>>>()?
            .iter()
            .any(|e| e.is_dir()));

        Ok(())
    }

    #[test]
    fn test_invalid_toml() -> ParserResult<()> {
        if TomlMd::parse(
            invalid_load_post()?
                .into_iter()
                .next()
                .expect("File didn't load."),
        )
        .is_ok()
        {
            panic!(r#"Invalid toml was parsed as Ok()"#)
        }

        Ok(())
    }

    #[test]
    fn test_parse_tomlmd() -> io::Result<()> {
        let (dir, _post1, _post2) = example_posts()?;
        let sources = discover_sources(dir.path())?;
        let sources = read_sources(sources).expect("Reading failed.");
        let sources: Vec<ParserInput> = sorted(sources).collect();
        let sources = parse_sources(sources).expect("Parse failed.");
        let (obj1, obj2) = expected_sources_parsed();

        assert_eq!(vec![obj1, obj2], sources);

        Ok(())
    }
}
