use clap::{crate_name, crate_version, crate_authors, crate_description};

pub fn parse_args<'a>() -> clap::ArgMatches<'a> {
    clap::App::new(crate_name!())
        .version(crate_version!())      
        .author(crate_authors!())       
        .about(crate_description!())
        .get_matches()
}