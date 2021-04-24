use std::io;

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
