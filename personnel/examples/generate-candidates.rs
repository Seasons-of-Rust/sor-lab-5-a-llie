use std::{fs::File, io::Write};

use personnel::{AstronautJob, Candidate};

fn main() {
    // Generate 1000 unique candidates
    let mut candidates = Vec::new();

    // Collect the jobs in an array
    let jobs = [
        AstronautJob::Biogeochemist,
        AstronautJob::Biologist,
        AstronautJob::Engineer,
        AstronautJob::Geologist,
        AstronautJob::Mechanic,
        AstronautJob::Medic,
        AstronautJob::RoverOp,
        AstronautJob::Scientist,
    ];

    // Iterate over each job for the primaries
    for primary_job in jobs.iter() {
        // Iterate over each job for the secondaries
        for secondary_job in jobs.iter() {
            // Iterate over some ages and health values
            for age in (16..93).step_by(3) {
                for health in (28..82).step_by(3) {
                    // Create a new candidate
                    candidates.push(Candidate {
                        primary_job: primary_job.clone(),
                        secondary_job: Some(secondary_job.clone()),
                        age,
                        health,
                    });
                }
            }
        }
    }

    // Open a file to write to
    let mut file = File::create("candidates.txt").expect("Failed to create candidates.txt");

    // Iterate over the candidates, writing them to the file
    for candidate in candidates.iter() {
        let line = format!(
            "{:?},{},{},{}\n",
            candidate.primary_job,
            match &candidate.secondary_job {
                Some(job) => format!("{:?}", job),
                None => "None".to_string(),
            },
            candidate.age,
            candidate.health
        );

        file.write_all(line.as_bytes())
            .expect("Failed to write candidate to file");
    }
}
