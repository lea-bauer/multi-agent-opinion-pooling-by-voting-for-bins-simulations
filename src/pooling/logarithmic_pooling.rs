/*
- in general:
  separately for lower, upper beliefs calculate (product of all beliefs^weight)/((product of all beliefs^weight)+(product of (1-belief)^weight) s.t. sum of all weights is 1
*/

use rust_decimal::Decimal;
use crate::pooling;
//use crate::voting;

pub(crate) fn logarithmic_pooling(voters: f32, _bins: Vec<[Decimal;2]>, imprecise_beliefs: Vec<[f32;2]>, true_probability: f32, priority: f32) -> ([f32; 2],f32) {
    println!("-- Logarithmic pooling:");
    //aggregation
    let mut logarithmic_pooling_beliefs: Vec<[f32;2]> = vec![];
    //aggregation with uniform weight distribution
    let group_belief = logarithmic_pooling_with_equal_weights(voters, imprecise_beliefs.clone());
    logarithmic_pooling_beliefs.push(group_belief);
    println!("Aggregated group belief is: {:?}", group_belief);
    //scoring
    let scoring_value: f32 = pooling::compute_ip_scoring_rule(group_belief, true_probability, priority);
    println!("Scoring value: {}", scoring_value);
    return (group_belief, scoring_value);
}

fn logarithmic_pooling_with_equal_weights(voters: f32, imprecise_beliefs: Vec<[f32;2]>) -> [f32;2] {
    let weight: f32  = 1.0 / voters;
    let mut lower_probabilities: Vec<f32> = vec![];
    let mut upper_probabilities: Vec<f32> = vec![];
    for belief in imprecise_beliefs {
        lower_probabilities.push(belief[0]);
        upper_probabilities.push(belief[1]);
    }
    let min_group_belief = product_beliefs_times_weight(lower_probabilities.clone(), weight) / (product_beliefs_times_weight(lower_probabilities.clone(), weight) + product_inverse_beliefs_times_weight(lower_probabilities.clone(), weight));
    let max_group_belief = product_beliefs_times_weight(upper_probabilities.clone(), weight) / (product_beliefs_times_weight(upper_probabilities.clone(), weight) + product_inverse_beliefs_times_weight(upper_probabilities.clone(), weight));
    let group_belief: [f32;2] = [min_group_belief, max_group_belief];
    return group_belief;
}

fn product_beliefs_times_weight(beliefs: Vec<f32>, weight: f32) -> f32 {
    let mut product: f32 = 1.0;
    for belief in beliefs {
        product = product * belief.powf(weight);
    }
    return product;
}

fn product_inverse_beliefs_times_weight(beliefs: Vec<f32>, weight: f32) -> f32 {
    let mut product: f32 = 1.0;
    for belief in beliefs {
        product = product * (1.0 - belief).powf(weight);
    }
    return product;
}
