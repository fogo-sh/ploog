use std::fs::{self, read_to_string};
use std::io;
use std::path::{Path, PathBuf};
use toml::Value;

struct TomlMd {
    toml: Toml,
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
    const SOURCES: (&str, &str) = (
        r#"---
[owner]
name = \"Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00 # First class dates
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
}
