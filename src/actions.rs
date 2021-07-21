// use chrono::Local;
// extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    name: String,
    created: String,
    description: String,
    log_time: i64,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Description must be a string of 256 characters")]
    DescriptionValError(String),
    #[error("Unknown error related to tasks")]
    Unknown,
}

pub fn create_task(
    tnam: String,
    tdesc: String,
    tcreated: chrono::Date<chrono::Local>,
) -> Result<Task> {
    // serialization
    let ntask = Task {
        name: tnam,
        description: tdesc,
        created: tcreated.to_string().to_owned(),
        log_time: 0,
    };

    Ok(ntask)
    // // current_date.format("task_%Y%m%d").to_string()
    // serde_json::to_string(&ntask)?;
    // Ok(tnam)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     // use chrono::Local;
//     // use chrono::TimeZone;

//     #[test]
//     fn test_create_task_description_length() {
//         Task {
//             name: "t1",
//             description: "",
//         }
//     }
// }
