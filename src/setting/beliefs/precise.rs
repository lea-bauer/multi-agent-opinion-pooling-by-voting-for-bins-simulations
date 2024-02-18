/*
arranged by alternatives:
- alternative 1..m
  - voter 1..n
[[alternative1_belief1,...,alternative1_beliefn],...,[alternativem_belief1,...,alternativem_beliefn]]
 */
use rand::Rng;
use rand_distr::Normal;
use rand_distr::Distribution;

pub(crate) fn generate_precise_beliefs(voters: f32, alternatives: usize, delta_p: f32, ground_truth_bin: usize) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();
    let mut target_probability_ground_truth = rng.gen_range(delta_p..1.0);
    println!("target probability ground truth {}", target_probability_ground_truth);
    let mut derivation_ground_truth = calculate_derivation(target_probability_ground_truth);
    let mut normally_distributed_beliefs_ground_truth = generate_normally_distributed_beliefs(target_probability_ground_truth, derivation_ground_truth, voters as usize);
    let mut clipped_beliefs_ground_truth = clip_beliefs(normally_distributed_beliefs_ground_truth.clone(), voters as usize);
    let mut average_ground_truth_beliefs = compute_average_belief(clipped_beliefs_ground_truth.clone());
    while average_ground_truth_beliefs < delta_p {
        target_probability_ground_truth = rng.gen_range(delta_p..1.0);
        derivation_ground_truth = calculate_derivation(target_probability_ground_truth);
        normally_distributed_beliefs_ground_truth = generate_normally_distributed_beliefs(target_probability_ground_truth, derivation_ground_truth, voters as usize);
        clipped_beliefs_ground_truth = clip_beliefs(normally_distributed_beliefs_ground_truth.clone(), voters as usize);
        average_ground_truth_beliefs = compute_average_belief(clipped_beliefs_ground_truth.clone());
    }
    println!("average ground truth beliefs: {}", average_ground_truth_beliefs);
    let mut target_probabilities: Vec<f32> = Vec::new();
    let mut derivations: Vec<f32> = Vec::new();
    let mut normally_distributed_beliefs: Vec<Vec<f32>> = Vec::new();
    let mut clipped_beliefs: Vec<Vec<f32>> = Vec::new();
    for alternative in 0..alternatives {
        println!("\n//Bin{}", alternative +  1);
        if alternative == ground_truth_bin - 1 {
            println!("ground truth bin");
            target_probabilities.push(target_probability_ground_truth);
            derivations.push(derivation_ground_truth);
            normally_distributed_beliefs.push(normally_distributed_beliefs_ground_truth.clone());
            clipped_beliefs.push(clipped_beliefs_ground_truth.clone());
        } else {
            //generate target probability for false alternative
            let mut rng = rand::thread_rng();
            let target_probability_false_alternative = rng.gen_range(0.0..=average_ground_truth_beliefs - delta_p);
            target_probabilities.push(target_probability_false_alternative);
            let derivation = calculate_derivation(target_probability_false_alternative);
            derivations.push(derivation);
            let normally_distributed_beliefs_false_alternative = generate_normally_distributed_beliefs(target_probability_false_alternative, derivation, voters as usize);
            normally_distributed_beliefs.push(normally_distributed_beliefs_false_alternative.clone());
            let mut clipped_beliefs_false_alternative = clip_beliefs(normally_distributed_beliefs_false_alternative.clone(), voters as usize);
            println!("clipped beliefs false alternative {:?}", clipped_beliefs_false_alternative);
            let average_belief_false_alternative = compute_average_belief(clipped_beliefs_false_alternative.clone());
            println!("average belief false alternative {}, compare with average ground truth beliefs - Δ𝒑 = {}", average_belief_false_alternative, average_ground_truth_beliefs - delta_p);
            if average_belief_false_alternative > average_ground_truth_beliefs - delta_p {
                println!("\n\n----CHANGE BELIEFS----\n\n");
                println!("average belief false alternative = {} > average ground truth beliefs - Δ𝒑 = {}, setting all precise beliefs to target probability {}", average_belief_false_alternative, average_ground_truth_beliefs - delta_p, target_probability_false_alternative);
                for clipped_belief in 0..clipped_beliefs_false_alternative.len() {
                    clipped_beliefs_false_alternative[clipped_belief] = target_probability_false_alternative;
                }
            }
            println!("clipped beliefs false alternative final {:?}", clipped_beliefs_false_alternative);
            clipped_beliefs.push(clipped_beliefs_false_alternative.clone());
        }
    }
    return clipped_beliefs;
}

/*
- generate target probabilities with impact of Δ𝒑:
 - for ground truth: ∈ [Δ𝒑, 1)
 - for false alternatives: ∈ [0, target_probability_ground_truth - Δ𝒑]
- serve as means in sequel calculation of precise beliefs via normal distribution
*/

//calculate standard derivation used in the calculation of precise beliefs via normal distribution s.t. whole Gaussian bell curve is available for belief generation
pub(crate) fn calculate_derivation(target_probability: f32) -> f32 {
    let width = f32::min(target_probability, 1.0 - target_probability); //width ≙ 3σ
    let derivation = width / 3.0;
    return derivation;
}

fn generate_normally_distributed_beliefs(target_probability: f32, derivation: f32, voters: usize) -> Vec<f32> {
    let mut normal_beliefs: Vec<f32> = Vec::new();
    let normal = Normal::new(target_probability, derivation).unwrap();
    for _voter in 0..voters {
        let belief = normal.sample(&mut rand::thread_rng());
        normal_beliefs.push(belief);
    }
    return normal_beliefs;
}

fn clip_beliefs(normal_beliefs: Vec<f32>, voters: usize) -> Vec<f32> {
    let mut clipped_beliefs: Vec<f32> = Vec::new();
    for voter in 0..voters {
        let belief: f32;
        let original_belief = normal_beliefs[voter];
        if original_belief < 0.0 {
            belief = 0.0;
        } else if original_belief > 1.0 {
            belief = 1.0;
        } else {
            belief = original_belief;
        }
        clipped_beliefs.push(belief);
    }
    return clipped_beliefs;
}

fn compute_average_belief (beliefs: Vec<f32>) -> f32 {
    let mut average_belief = 0.0;
    for belief in &beliefs {
        average_belief += belief;
    }
    println!("beliefs sum: {:?}", average_belief);
    average_belief = average_belief / beliefs.len() as f32;
    println!("average: {}", average_belief);
    return average_belief;
}