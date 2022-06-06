use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Clone, Debug)]
pub enum AstronautJob {
    Biogeochemist,
    Biologist,
    Engineer,
    Geologist,
    Mechanic,
    Medic,
    RoverOp,
    Scientist,
}

impl FromStr for AstronautJob {
    type Err = String;

    /// Converts a string to an AstronautJob
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Biogeochemist" => Ok(AstronautJob::Biogeochemist),
            "Biologist" => Ok(AstronautJob::Biologist),
            "Engineer" => Ok(AstronautJob::Engineer),
            "Geologist" => Ok(AstronautJob::Geologist),
            "Mechanic" => Ok(AstronautJob::Mechanic),
            "Medic" => Ok(AstronautJob::Medic),
            "RoverOp" => Ok(AstronautJob::RoverOp),
            "Scientist" => Ok(AstronautJob::Scientist),
            _ => Err(format!("Unknown job: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Candidate {
    pub primary_job: AstronautJob,
    pub secondary_job: Option<AstronautJob>,
    pub age: u8,
    pub health: u8,
}

impl Candidate {
    /// Loads the candidates from the candidates.txt file into a vector
    pub fn load_candidate_file() -> Vec<Candidate> {
        // Open the file candidates.txt
        let file = File::open("candidates.txt").expect("Failed to open candidates.txt");

        io::BufReader::new(file) // Create a buffered reader
            .lines() // Iterate over the lines
            .map(|line| {
                let line = line.unwrap(); // Get the line
                let mut parts = line.split(','); // Split the line on commas

                // Parse the candidate's data
                let primary_job = parts.next().unwrap();
                let secondary_job = parts.next().unwrap();
                let age = parts.next().unwrap();
                let health = parts.next().unwrap();

                // Return this candidate to be added to the list
                Candidate {
                    primary_job: primary_job.parse().unwrap(),
                    secondary_job: if secondary_job == "None" {
                        None
                    } else {
                        Some(secondary_job.parse().unwrap())
                    },
                    age: age.parse().unwrap(),
                    health: health.parse().unwrap(),
                }
            })
            .collect() // Collect the candidates into a vector
    }
}
