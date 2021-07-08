#![allow(dead_code, unused_imports)]
extern crate clap;

use std::string;

use chrono::Local;

mod utils;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use thiserror::Error;
fn main() -> Result<(), anyhow::Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("start")
                .about("Start a new task")
                .arg(
                    Arg::with_name("task_name")
                        .index(1)
                        .value_name("task_name")
                        .required(false)
                        .help("Start a new task"),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        ("start", Some(sub_m)) => {
            let today = utils::autogenerate_task_name(Local::today());
            let task_name: String = sub_m.value_of("task_name").unwrap_or(&today).to_string();
            println!("Default {}", task_name)
        }
        _ => panic!("chau!"),
    }
    Ok(())
}
