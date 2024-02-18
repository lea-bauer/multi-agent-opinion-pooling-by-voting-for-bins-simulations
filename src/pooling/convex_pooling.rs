use rust_decimal::Decimal;
use crate::pooling;

//use crate::voting
pub(crate) fn convex_pooling(_bins: Vec<[Decimal;2]>, imprecise_beliefs: Vec<[f32; 2]>, true_probability: f32, priority: f32) -> ([f32;2], f32) {
    println!("-- Convex pooling:");
    //aggregation
    let aggregated_group_belief = aggregation(imprecise_beliefs);
    println!("Aggregated group belief is: {:?}", aggregated_group_belief);
    /*
    //voting
    let votes = voting::vote(bins, group_belief);
    println!("Alternatives voted for: {:?}", votes);
     */
    //scoring
    let scoring_value = pooling::compute_ip_scoring_rule(aggregated_group_belief, true_probability, priority);
    println!("Scoring value: {}", scoring_value);
    return (aggregated_group_belief, scoring_value);
}

fn aggregation(imprecise_beliefs: Vec<[f32; 2]>) -> [f32; 2] {
    let mut min_group_belief= 0.0;
    let mut max_group_belief= 0.0;
    for belief in 0..imprecise_beliefs.len() {
        let imprecise_belief = imprecise_beliefs[belief];
        let min_belief = imprecise_belief[0];
        let max_belief = imprecise_belief[1];
        if belief == 0 {
            min_group_belief = min_belief;
            max_group_belief = max_belief;
        } else {
            if min_belief < min_group_belief {
                min_group_belief = min_belief;
            }
            if max_belief > max_group_belief {
                max_group_belief = max_belief;
            }
        }
    }
    let group_belief: [f32; 2] = [min_group_belief, max_group_belief];
    return group_belief;
}