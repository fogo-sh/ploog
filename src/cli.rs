use clap::clap_app;

pub fn matches() -> clap::ArgMatches {
    clap_app!(ploog =>
        (version: "0.0")
        (author: "Mitchell H. <me@mitchellhynes.com>")
        (about: "Configurable Static Site Generator.")
        (@arg source: +required "Toml.MD Sources Directory")
        (@arg output: +required "HTML Output Directory")
        (@arg server: -s --serve "Serves your site")
        (@arg watch: -w --watch "Watch source directory for changes")
        (@arg console: -c --console "Plugin Store and MD editor")
        (@arg html_slugs: -a --altslug "post.md becomes post.html instead of post/index.html")
    )
    .get_matches()
}
