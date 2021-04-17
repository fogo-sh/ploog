# Ploog: Configurable Static Site Generator

## Generator Development

### Building

`cargo build --release`

### Running

`cargo run -- posts public`

### Help

```
cargo run -- --help

ploog 0.0
Mitchell H. <me@mitchellhynes.com>
Configurable Static Site Generator.

USAGE:
    ploog [FLAGS] <source> <output>

ARGS:
    <source>    Toml.MD Sources Directory
    <output>    HTML Output Directory

FLAGS:
    -c, --console    Plugin Store and MD editor
    -h, --help       Prints help information
    -a, --altslug    post.md becomes post.html instead of post/index.html
    -s, --serve      Serves your site
    -V, --version    Prints version information
    -w, --watch      Watch source directory for changes
```

## Generator Console Development

### Dev Server

```
cd web
npm run start
```
