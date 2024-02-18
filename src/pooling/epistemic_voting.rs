/*
use rand_distr::num_traits::pow;

pub(crate) fn _epistemic_pooling(imprecise_beliefs: Vec<[f32; 2]>) {
    println!("-- Epistemic pooling:");
    //aggregtion
    compute_scoring_rules(imprecise_beliefs);
    /*
    //voting
    let votes =
    println!("Alternatives voted for: {:?}", votes);
    Existiert überhaupt eine Generalisierung für den Fall verschiedener Scoring rules?
     */
    //scoring
}

fn compute_scoring_rules(imprecise_beliefs: Vec<[f32; 2]>) -> Vec<f32> {
    let mut scoring_rules: Vec<f32> = Vec::new();
    for imprecise_belief in imprecise_beliefs {
        let min_belief = imprecise_belief[0];
        let max_belief = imprecise_belief[1];
        let scoring_rule = (-1.0 * max_belief) + (min_belief * max_belief) + (f32::powf((min_belief * max_belief) - (pow(min_belief, 2) * max_belief) - (min_belief * pow(max_belief, 2)) + (pow(min_belief, 2) * pow(max_belief, 2)), 0.5)) / (min_belief - max_belief);
        scoring_rules.push(scoring_rule);
    }
    println!("Scoring rules: {:?}", scoring_rules);
    return scoring_rules;
}
*/