pub mod job;
pub mod arrow;

use job::{Job, Step};
// use std::collections::HashMap;
use rand::prelude::*;

pub fn validate_job<S: Step + Clone>(
    job: &mut Job<S>,
    // requirements: &Requirements
) -> bool {
    let mut ok = true;

    // invalid name
    if job.name.len() == 0 {
        let mut rng = rand::thread_rng();
        let id: u16 = rng.gen();
        let name = format!("job_id_{}", id);
        println!(
            "ERROR: Job '{}' has an invalid name; job renamed to '{}'.",
            job.name, name
        );
        job.name = name;
        ok = false;
    }

    // invalid description
    if job.description().len() == 0 {
        println!("ERROR: Job '{}' needs a description.", job.name);
        ok = false;
    }

    // invalid requirements
    // for label in job.requirements() {
    //     if requirements.get(label).is_none() {
    //         println!("ERROR: Requirement '{}' not found in NocoDB.", label);
    //         ok = false;
    //     }
    // }

    ok
}

// pub fn validate_suite(
//     suite: &Suite,
//     // requirements: &Requirements
// ) {
//     // let requirements = nocodb::requirements()

//     let mut ok = true;
//     for job in suite.jobs() {
//         let result = validate_job(
//             job,
//             // &requirements
//         );
//     }
// }
