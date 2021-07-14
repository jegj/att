extern crate clap;

use chrono::Local;

mod utils;

mod actions;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
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
                        .help("Task name"),
                )
                .arg(
                    Arg::with_name("task_description")
                        .short("d")
                        .long("description")
                        .takes_value(true)
                        .required(false)
                        .help("Set a custom description to the task"),
                )
                .arg(
                    Arg::with_name("start_time")
                        .short("s")
                        .long("start-time")
                        .takes_value(true)
                        .required(false)
                        .help("Set start time to the task"),
                )
                .arg(
                    Arg::with_name("end_time")
                        .short("e")
                        .long("end-time")
                        .takes_value(true)
                        .required(false)
                        .help("Set end time on the task"),
                )
                .arg(
                    Arg::with_name("only_create")
                        .short("o")
                        .long("only-create")
                        .takes_value(false)
                        .required(false)
                        .help("No start timer on this task"),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        ("start", Some(sub_m)) => {
            let today = utils::autogenerate_task_name(Local::today());
            let task_name: String = sub_m.value_of("task_name").unwrap_or(&today).to_string();
            let task_description = sub_m.value_of("task_description").unwrap_or("").to_string();
            let end_time = sub_m.value_of("end_time").unwrap_or("").to_string();
            let start_time = sub_m.value_of("start_time").unwrap_or("").to_string();
            // let task_description: String = sub_m
            //     .value_of("task_description")
            println!(
                "Default: {} {} {} {}",
                task_name, task_description, start_time, end_time
            )
        }
        _ => panic!("chau!"),
    }
    Ok(())
}
