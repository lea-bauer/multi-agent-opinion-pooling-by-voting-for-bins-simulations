/*
representing each voter's imprecise belief as interval of probabilities:
[[voter1_lower_belief, voter1_upper_belief],...,[]]
*/
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

pub(crate) fn generate_imprecise_beliefs(voters: f32, alternatives: usize, ground_truth_bin: usize, ground_truth_interval: [Decimal;2], precise_beliefs: Vec<Vec<f32>>, bins: Vec<[Decimal;2]>) -> Vec<[f32;2]> {
    let centers = choose_belief_center(voters as usize, alternatives, bins.clone(), ground_truth_bin, ground_truth_interval, precise_beliefs.clone());
    let votes_all_alternatives = does_voter_vote_for_alternative(voters as usize, alternatives, precise_beliefs.clone());
    let imprecise_beliefs = expand_beliefs(voters as usize, alternatives, bins.clone(), centers, votes_all_alternatives);
    return imprecise_beliefs;
}

/*
For each voter: choose probability this voter's imprecise belief shall be centering around symmetrically:
    1. Decide whether this center probability is from ground truth bin or not:
        - generate random_element from [0.0,1.0]
        - a. if random_element ∈ [0.0,voters_precise_belief_in_ground_truth]: center will be from ground truth bin
        - b. else if random_element ∈ (voters_precise_belief_in_ground_truth,1.0]: center will be from bin other than ground truth
    2. Choose center probability:
        a. choose center probability from ground truth bin
        b. choose center probability from bin other than ground truth
           - add up all precise beliefs of this voter
           - constitute interval: [0, sum_precise_beliefs]
           - subintervals are of the length of precise beliefs: sub1 = [0, preciseBelief1], l(sub1) = preciseBelief1; sub2 = [preciseBelief1, preciseBelief1 + preciseBelief2], l(sub2) = preciseBelief2
           - randomly choose one element from [0, sum_precise_beliefs]
           - this element falls into one of the subintervals with length of one precise belief
           - this precise belief belongs to one bin
           - randomly choose one element from this bin as belief center
 */
fn choose_belief_center(voters: usize, alternatives :usize, bins: Vec<[Decimal;2]>, ground_truth_bin: usize, ground_truth_interval: [Decimal;2], precise_beliefs: Vec<Vec<f32>>) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    let mut centers: Vec<f32> = vec![];
    for voter in 0..voters {
        let mut precise_beliefs_of_one_voter: Vec<f32> = Vec::new();
        for alternative in 0..alternatives {
            precise_beliefs_of_one_voter.push(*precise_beliefs.get(alternative).expect("REASON").get(voter).unwrap());
        }
        println!("\nprecise beliefs of voter {}: {:?}", voter + 1, precise_beliefs_of_one_voter);
        let precise_belief_in_ground_truth = precise_beliefs_of_one_voter[ground_truth_bin - 1];
        println!("precise belief in ground truth bin: {}", precise_belief_in_ground_truth);
        let random: f32 = rng.gen();
        println!("random element is {}", random);
        //if random ∈ [0,precise_belief_in_ground_truth] center of voters imprecise belief will be from ground truth bin
        if random >= 0.0 && random <= precise_belief_in_ground_truth {
            let lower = f32::try_from(ground_truth_interval[0]).unwrap();
            let upper = f32::try_from(ground_truth_interval[1]).unwrap();
            let center = rng.gen_range(lower..=upper);
            println!("element from ground truth bin serving as center of imprecise belief: {}\n", center);
            centers.push(center);
        //if random ∈ (precise_belief_in_ground_truth,1] center of voters imprecise belief will be from any other bin where probability to select one bin depends on precise belief
        } else {
            println!("element from bin other than ground truth bin serving as center of imprecise belief");
            let mut lower = 0.0;
            let mut upper = 0.0;
            //generate first subinterval [0,precise_belief]
            for false_precise_belief in 0..alternatives {
                if false_precise_belief != (ground_truth_bin - 1) {
                    upper = precise_beliefs_of_one_voter[false_precise_belief];
                    break;
                }
            }
            let mut false_alternatives: Vec<[f32;2]> = vec![];
            let mut sum_false_precise_beliefs: f32 = 0.0;
            for false_precise_belief in 0..alternatives {
                if false_precise_belief != (ground_truth_bin - 1) {
                    false_alternatives.push([lower,upper]);
                    sum_false_precise_beliefs += precise_beliefs_of_one_voter[false_precise_belief];
                    lower = upper;
                    let mut next_upper;
                    for next in false_precise_belief..alternatives - 1 {
                        next_upper = next + 1;
                        if next_upper != ground_truth_bin - 1 {
                            upper += precise_beliefs_of_one_voter[next_upper];
                            break;
                        }
                    }
                } else if false_precise_belief == ground_truth_bin - 1 {
                    false_alternatives.push([lower,lower]);
                }
            }
            println!("false alternatives: {:?}", false_alternatives);
            let random = rng.gen_range(0.0..=sum_false_precise_beliefs);
            println!("random element false alternatives: {}", random);
            let mut false_precise = 0;
            for false_alternative in 0..alternatives {
                if random >= false_alternatives[false_alternative][0] && random <= false_alternatives[false_alternative][1] && false_alternative != ground_truth_bin - 1 {
                    false_precise = false_alternative + 1;
                    println!("false precise: {}", false_precise);
                    break;
                }
            }
            println!("precise beliefs of 1 voter are {:?}",precise_beliefs_of_one_voter);
            println!("choosing precise belief {}",false_precise);
            let bin_interval = bins[false_precise - 1];
            let lower_bin = bin_interval[0].to_f32().unwrap();
            let upper_bin = bin_interval[1].to_f32().unwrap();
            let center = rng.gen_range(lower_bin..=upper_bin);
            println!("center from false alternatives {}\n", center);
            centers.push(center);
        }
        //println!("element: {}", element);
    }
    //println!("Imprecise beliefs: {:?}", imprecise_beliefs);
    println!("centers: {:?}", centers);
    return centers;
}

fn does_voter_vote_for_alternative(voters: usize, alternatives: usize, precise_beliefs: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut votes_all_alternatives: Vec<Vec<f32>> = vec![];
    for alternative in 0..alternatives {
        let beliefs = precise_beliefs.get(alternative).unwrap();
        let mut votes_one_alternative: Vec<f32> = vec![];
        for voter in 0..voters {
            let mut rng = rand::thread_rng();
            let vote = rng.gen_range(0.0..=1.0) as f32;
            let precise_belief = beliefs.get(voter).unwrap();
            if vote > *precise_belief {
                votes_one_alternative.push(0.0);
            } else {
                votes_one_alternative.push(1.0);
            }
        }
        votes_all_alternatives.push(votes_one_alternative);
    }
    return votes_all_alternatives;
}

fn expand_beliefs (voters: usize , alternatives :usize, bins: Vec<[Decimal; 2]>, centers: Vec<f32>, votes_all_alternatives: Vec<Vec<f32>>) -> Vec<[f32;2]> {
    let mut imprecise_beliefs: Vec<[f32;2]> = vec![];
    for voter in 0..voters {
        let mut votes_one_voter: Vec<f32> = vec![];
        for alternative in 0..alternatives {
            let votes_one_alternative = votes_all_alternatives.get(alternative);
            let vote = votes_one_alternative.expect("REASON").get(voter).unwrap();
            votes_one_voter.push(*vote);
        }
        let mut imprecise_belief: Vec<f32> = vec![];
        let center = centers.get(voter).unwrap();
        let mut center_bin: usize = 0;
        for alternative in 0..alternatives {
            let bin: [Decimal; 2] = bins[alternative];
            if *center >= f32::try_from(bin[0]).unwrap() && *center < f32::try_from(bin[1]).unwrap() {
                center_bin = alternative;
                break;
            }
        }
        //todo nach unten + oben suchen parallelisieren
        let mut lower = center_bin;
        loop {
            if lower > 0 {
                if votes_one_voter[lower-1] == 1.0 {
                    lower -= 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        let mut upper = center_bin;
        loop {
            if upper < alternatives-1 {
                if votes_one_voter[upper+1] == 1.0 {
                    upper += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        let lower_bin = bins[lower];
        let lower_belief = lower_bin[0];
        let upper_bin = bins[upper];
        let upper_belief = upper_bin[1];
        imprecise_belief.push(f32::try_from(lower_belief).unwrap());
        imprecise_belief.push(f32::try_from(upper_belief).unwrap());
        imprecise_beliefs.push(<[f32; 2]>::try_from(imprecise_belief).unwrap());
    }
    println!("imprecise beliefs: {:?}", imprecise_beliefs);
    return imprecise_beliefs;
}