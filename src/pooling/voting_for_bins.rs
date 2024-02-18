use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use crate::pooling;

pub(crate) fn voting_for_bins(voters: f32, alternatives: usize, bins: Vec<[Decimal; 2]>, imprecise_beliefs: Vec<[f32; 2]>, true_probability: f32, alpha: f32, ground_truth_bin: usize) -> ([f32; 2], f32) {
    println!("-- Voting for bins:");
    //voting
    /*
   votes multiple = [[voter1_vote1, voter1_vote2, ...],...,[votern_vote1, votern_vote2, ...]]
   - voters can vote for multiple alternatives, as most m (number of alternatives; cannot vote for more than exist)
   */
    let mut votes_multiple: Vec<Vec<usize>> = Vec::new();
    for voter in 0..voters as usize {
        let imprecise_belief: [f32; 2] = imprecise_beliefs[voter];
        let lebesques = compute_lebesque_measures(alternatives, bins.clone(), imprecise_belief);
        //println!("Lebesque measures with voter{}'s belief = {:?} are {:?}", voter + 1, imprecise_belief, lebesques);
        let votes = vote_for_max_lebesques(alternatives, lebesques);
        //println!("Voter{} votes for {:?}", voter + 1, votes);
        votes_multiple.push(votes);
    }
    println!("Votes: {:?}", votes_multiple);
    //# votes each alternative gets
    let scores = scores(voters, alternatives, votes_multiple);
    println!("Scores: {:?}", scores);
    //vote for all alternatives with max. score
    let mut aggregated_group_belief: [f32;2] = [f32::NAN, f32::NAN];
    let scores_new= max_scores(scores.clone());
    let max_score = scores_new.0;
    println!("Scores ground truth bin: {}, scores winner: {}", scores[ground_truth_bin-1], max_score);
    let votes = scores_new.1;
    if votes.len() > 1 {
        println!("There are {} alternatives with max. lebesque measure, therefore no single alternative wins the approval pooling.", votes.len());
    } else {
        println!("Alternative {} wins the approval pooling.", votes[0]);
        //aggregation
        let belief = bins[votes[0]-1];
        let min_belief = f32::try_from(belief[0]).unwrap();
        let max_belief = f32::try_from(belief[1]).unwrap();
        aggregated_group_belief = [min_belief, max_belief];
        println!("Aggregated group belief is: {:?}", aggregated_group_belief);
    }
    //scoring
    let scoring_value = pooling::compute_ip_scoring_rule(aggregated_group_belief, true_probability, alpha);
    println!("Scoring value: {}", scoring_value);
    return (aggregated_group_belief, scoring_value);
}

fn compute_lebesque_measures(alternatives: usize, bins: Vec<[Decimal; 2]>, imprecise_belief: [f32; 2]) -> Vec<f32> {
    let mut lebesques: Vec<f32> = Vec::new(); //stores lebesque measures OF ONE VOTER with each bin, bins are indices + 1, lebesque measures the values
    for alternative in 0..alternatives {
        let bin: [Decimal; 2] = bins[alternative];
        let lower_bin = bin[0].to_f32().unwrap();
        let upper_bin = bin[1].to_f32().unwrap();
        //1. + 2.: bin above belief or belief above bin, empty intersection, no shared elements
        if (lower_bin > imprecise_belief[1]) || (imprecise_belief[0] >= upper_bin) {
            lebesques.push(0.0);
        }
        //3.: bin within belief, lebesque is bin lebesque
        else if (lower_bin >= imprecise_belief[0]) && (upper_bin <= imprecise_belief[1]) {
            lebesques.push(upper_bin - lower_bin);
        }
        //4.: belief within bin, lebesque is belief lebesque
        else if (imprecise_belief[0] >= lower_bin) && (imprecise_belief[1] < upper_bin) {
            lebesques.push(imprecise_belief[1] - imprecise_belief[0]);
        }
        //5.: bin above belief with shared elements, lebesque is lebesque of intersection
        else if (lower_bin >= imprecise_belief[0]) && (lower_bin <= imprecise_belief[1]) && (upper_bin >= imprecise_belief[1]) {
            lebesques.push(imprecise_belief[1] - lower_bin);
        }
        //6.: belief above bin with shared elements, lebesque is lebesque of intersection
        else if (imprecise_belief[0] >= lower_bin) && (imprecise_belief[0] < upper_bin) && (imprecise_belief[1] >= upper_bin) {
            lebesques.push(upper_bin - imprecise_belief[0]);
        }
    }
    return lebesques;
}

fn vote_for_max_lebesques(alternatives: usize, lebesques: Vec<f32>) -> Vec<usize> {
    /*
    - select max lebesque
    - in case of multiple votes there are several bins this voter has this ONE max lebesque with
     */
    let max_lebesque: f32 = lebesques
        .clone()
        .into_iter()
        .reduce(f32::max)
        .unwrap();
    let mut vote_multiple: Vec<usize> = Vec::new();
    for alternative in 0..alternatives {
        if lebesques[alternative] == max_lebesque {
            vote_multiple.push(alternative + 1);
        }
    }
    return vote_multiple;
}

fn scores(voters: f32, alternatives: usize, voting: Vec<Vec<usize>>) -> Vec<usize> {
    //store for each alternative #votes
    let mut scores: Vec<usize> = Vec::new();
    for _alternative in 0..alternatives {
        scores.push(0);
    }
    for voter in 0..voters as usize {
        let votes: &Vec<usize> = &voting[voter];
        //increase scores of alternatives each voter votes for
        for i in 0..votes.len() {
            let vote: usize = votes[i];
            scores[vote-1] += 1;
        }
    }
    return scores;
}

fn max_scores(scores: Vec<usize>) -> (usize, Vec<usize>) {
    let max_score: usize = scores
        .clone()
        .into_iter()
        .reduce(usize::max)
        .unwrap();
    let mut votes: Vec<usize> = Vec::new();
    for alternative in 0..scores.len() {
        if scores[alternative] == max_score {
            votes.push(alternative + 1);
        }
    }
    return (max_score, votes);
}

