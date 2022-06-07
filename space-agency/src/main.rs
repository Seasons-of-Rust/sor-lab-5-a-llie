use personnel::{AstronautJob, Candidate};

pub fn categorize(duty: &AstronautJob) -> u32 {
    match duty {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}

pub fn job_score(score: &Candidate) -> u32 {
    let job_value = categorize(&score.primary_job);
    let job_calc = match &score.secondary_job {
        None => categorize(&score.primary_job),
        Some(value) => categorize(value),
    };
    (job_value * job_calc) % 576
}

pub fn calculate_score(score: &Candidate) -> u32 {
    let cand_score = (job_score(score) + (score.health as u32)) * (score.age as u32);
    cand_score % 3928
}

pub fn rank_candidates(ranks: &mut [Candidate]) {
    ranks.sort_by(|a, b| (&calculate_score(b)).cmp(&calculate_score(a)));
}

fn main() {
    let mut candidates = Candidate::load_candidate_file();
    rank_candidates(&mut candidates);
    println!("{:?}", candidates)
}

#[test]
fn test_job_score() {
    let double_job = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Geologist),
        health: 10,
        age: 22,
    };

    let single_job = Candidate {
        primary_job: AstronautJob::Mechanic,
        secondary_job: None,
        health: 10,
        age: 22,
    };

    assert_eq!(job_score(&double_job), 127);
    assert_eq!(job_score(&single_job), 289);
}

#[test]
fn test_calculate_score() {
    let double_job = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Geologist),
        health: 10,
        age: 22,
    };

    let single_job = Candidate {
        primary_job: AstronautJob::Mechanic,
        secondary_job: None,
        health: 10,
        age: 22,
    };

    assert_eq!(calculate_score(&double_job), 3014);
    assert_eq!(calculate_score(&single_job), 2650);
}

#[test]
fn test_rank_candidates() {
    let mut candidates = vec![
        Candidate {
            primary_job: AstronautJob::Mechanic,
            secondary_job: None,
            health: 10,
            age: 22,
        },
        Candidate {
            primary_job: AstronautJob::Biogeochemist,
            secondary_job: Some(AstronautJob::Geologist),
            health: 10,
            age: 22,
        },
    ];

    assert_eq!(calculate_score(&candidates[0]), 2650);
    assert_eq!(calculate_score(&candidates[1]), 3014);

    rank_candidates(&mut candidates);

    assert_eq!(calculate_score(&candidates[0]), 3014);
    assert_eq!(calculate_score(&candidates[1]), 2650);
}
