/*
- in general:
  separately sum up lower beliefs, upper beliefs multiplied by weight s.t. sum of all weights is 1
*/

use rust_decimal::Decimal;
use crate::pooling;

pub(crate) fn linear_pooling(voters: f32, _bins: Vec<[Decimal;2]>, imprecise_beliefs: Vec<[f32;2]>, true_probability: f32, priority: f32) -> ([f32; 2],f32) {
    println!("-- Linear pooling:");
    //aggregation with uniform weight distribution
    let group_belief = linear_pooling_with_equal_weights(voters, imprecise_beliefs.clone());
    println!("Aggregated group belief is: {:?}", group_belief);
    //scoring
    let scoring_value: f32 = pooling::compute_ip_scoring_rule(group_belief, true_probability, priority);
    println!("Scoring value: {:?}", scoring_value);
    return (group_belief, scoring_value);
}

fn linear_pooling_with_equal_weights(voters: f32, imprecise_beliefs: Vec<[f32; 2]>) -> [f32; 2] {
    let weight: f32  = 1.0 / voters;
    let mut min_group_belief = 0.0;
    let mut max_group_belief = 0.0;
    for belief in imprecise_beliefs {
        min_group_belief = min_group_belief + (weight * belief[0]);
        max_group_belief = max_group_belief + (weight * belief[1]);
    }
    let group_belief: [f32; 2] = [min_group_belief, max_group_belief];
    return group_belief;
}