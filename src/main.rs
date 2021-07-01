#![allow(dead_code, unused_imports)]

extern crate clap;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
fn main() {
    let _matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();
}
