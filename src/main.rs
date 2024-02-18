mod input;
mod setting;
mod pooling;
mod scoring;
mod graphs;

use std::time::Instant;
use crate::input::pause;

fn main() {
    let voters = input::enter_voters();
    let delta_p = input::enter_reliability();
    let p_min = input::enter_pmin();
    let rounds = input::enter_rounds();
    let max_alternatives = setting::bins::bins::compute_number_of_alternatives(voters, delta_p, p_min);
    let percentage_bins = input::enter_percentage_of_bins(max_alternatives);
    let actual_alternatives = setting::bins::bins::compute_share_of_bins(max_alternatives, percentage_bins);
    let precision= setting::bins::bins::compute_precision(actual_alternatives);
    let bins= setting::bins::bins::create_bins(actual_alternatives, precision);
    let true_probability= setting::bins::bins::generate_true_probability();
    let ground_truth_bin= setting::bins::bins::select_ground_truth_bin(actual_alternatives, bins.clone(), true_probability).0;
    let priority = input::enter_priority();

    pause();

    let start = Instant::now();

    let mut voters_beliefs: Vec<Vec<[f32;2]>> = Vec::new();
    let mut ip_scoring: Vec<(Vec<[f32;2]>,Vec<f32>)> = Vec::new();
    let mut beliefs: Vec<Vec<[f32;2]>> = Vec::new();
    let mut epistemic_values: Vec<Vec<f32>> = Vec::new();

    for round in 0..rounds {
        println!("\nRound {}:", round + 1);
        let precise_beliefs= setting::beliefs::precise::generate_precise_beliefs(voters, actual_alternatives, delta_p, ground_truth_bin);
        let imprecise_beliefs = setting::beliefs::imprecise::generate_imprecise_beliefs(voters, actual_alternatives, ground_truth_bin, bins[ground_truth_bin - 1], precise_beliefs, bins.clone());
        voters_beliefs.push(imprecise_beliefs.clone());
        let scoring = pooling::pooling(voters, actual_alternatives, bins.clone(), imprecise_beliefs, true_probability, priority, ground_truth_bin);
        ip_scoring.push(scoring.clone());
        beliefs.push(scoring.clone().0.into());
        epistemic_values.push(scoring.clone().1.into());
    }

    let average_epistemic_values = scoring::compute_average_epistemic_values(epistemic_values, rounds);

    graphs::draw_graphs(ip_scoring.clone(), average_epistemic_values.clone(), rounds, voters_beliefs);

    println!("for each round ([[VfB],[lin],[log],[conv]],[epistemic values]):\n {:?}", ip_scoring);
    println!("\naverage epistemic values:\n {:?}", average_epistemic_values);

    let end = start.elapsed();
    println!("\nRuntime: {:.2?}", end);
}