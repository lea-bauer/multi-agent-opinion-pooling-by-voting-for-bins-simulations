pub mod convex_pooling;
pub mod voting_for_bins;
mod epistemic_voting;
mod linear_pooling;
mod logarithmic_pooling;

use rust_decimal::Decimal;
use crate::pooling::logarithmic_pooling::logarithmic_pooling;

pub(crate) fn pooling(voters: f32, alternatives: usize, bins: Vec<[Decimal;2]>, imprecise_beliefs: Vec<[f32;2]>, true_probability: f32, priority: f32, ground_truth_bin: usize) -> (Vec<[f32;2]>,Vec<f32>) {
    let mut aggregated_group_beliefs: Vec<[f32;2]> = vec![];
    let mut epistemic_values: Vec<f32> = vec![];
    let voting_for_bins = voting_for_bins::voting_for_bins(voters, alternatives, bins.clone(), imprecise_beliefs.clone(), true_probability, priority, ground_truth_bin);
    aggregated_group_beliefs.push(voting_for_bins.0);
    epistemic_values.push(voting_for_bins.1);
    let linear_pooling = linear_pooling::linear_pooling(voters, bins.clone(), imprecise_beliefs.clone(), true_probability, priority);
    aggregated_group_beliefs.push(linear_pooling.0);
    epistemic_values.push(linear_pooling.1);
    let logarithmic_pooling = logarithmic_pooling(voters, bins.clone(), imprecise_beliefs.clone(), true_probability, priority);
    aggregated_group_beliefs.push(logarithmic_pooling.0);
    epistemic_values.push(logarithmic_pooling.1);
    let convex_pooling = convex_pooling::convex_pooling(bins.clone(), imprecise_beliefs.clone(), true_probability, priority);
    aggregated_group_beliefs.push(convex_pooling.0);
    epistemic_values.push(convex_pooling.1);
    println!("\nEpistemic values: {:?}", epistemic_values);
    return (aggregated_group_beliefs, epistemic_values);
}

fn compute_ip_scoring_rule(imprecise_belief: [f32;2], true_probability: f32, alpha: f32) -> f32 {
    /*
    Measure properties / epistemic value / utility of IP (Imprecise Probability) distributions by 2 principles:
        1. avoid error:
            - E(K,w)
            - truth value of belief state K at w: 0 ⇔ w ∉ K, 1 ⇔ w ∈ K
        2. seek truth:
            - T(K,w)
            - degree of informativeness: 1 - lebesque_measure(imprecise_belief)
        - K: imprecise belief, state
        - w: precise true probability, world
        - alpha: degree of priority given to the 2 principles 1. avoid error, 2. seek truth
     */
    let mut avoid_error: f32 = 0.0;
    if true_probability >= imprecise_belief[0] && true_probability <= imprecise_belief[1] {
        avoid_error = 1.0;
    }
    let lebesque_measure = imprecise_belief[1] - imprecise_belief[0];
    let seek_truth: f32 = 1.0 - lebesque_measure;
    let mut scoring_rule: f32 = (alpha * avoid_error) + ((1.0 - alpha) * seek_truth);
    if scoring_rule.is_nan() {
        scoring_rule = 0.0;
    }
    return scoring_rule;
}