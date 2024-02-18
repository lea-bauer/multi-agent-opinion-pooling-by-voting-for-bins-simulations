use std::cmp::max;
use std::f32::consts::E;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn compute_hoeffding(voters: f32, delta_p: f32, p_min: f32) -> usize{
    return ((1.0 - p_min) / (2.0 * E.powf(-0.5 * voters * (delta_p.powf(2.0)))) + 1.0).floor() as usize;
}


fn compute_chebychev_cantelli(voters: f32, delta_p: f32, p_min: f32) -> usize {
    return ((((1.0 - p_min) * (1.0 + (voters - 1.0) * delta_p.powf(2.0))) / (2.0 * (1.0 - delta_p.powf(2.0)))) + 1.0).floor() as usize;
}

pub(crate) fn compute_number_of_alternatives(voters: f32, delta_p: f32, p_min: f32) -> usize {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let hoeffding: usize = compute_hoeffding(voters, delta_p, p_min);
        sender.send(hoeffding).unwrap();
    });
    let chebychev_cantelli: usize = compute_chebychev_cantelli(voters, delta_p, p_min);
    handle.join().unwrap();
    let hoeffding = receiver.recv().unwrap();
    //maximum from both bounds yields number of alternatives
    let alternatives: usize = max(hoeffding, chebychev_cantelli);
    println!("Number of alternatives m: {}", alternatives);
    return alternatives;
}

pub(crate) fn compute_precision(alternatives: usize) -> Decimal {
    let m: f32 = alternatives as f32;
    let precision_percentage: f32 = 100f32 / m;
    let precision_percentage_string: String = precision_percentage.to_string();
    let precision_percentage_dec: Decimal = Decimal::from_str(&precision_percentage_string).unwrap();
    let precision: Decimal = precision_percentage_dec / dec!(100);
    println!("Precision: {}% = {} \n", precision_percentage_dec, precision);
    return precision;
}

pub(crate) fn create_bins(alternatives: usize, precision: Decimal) -> Vec<[Decimal; 2]> {
    let mut lower_bound: Decimal = dec!(0.0);
    let mut upper_bound: Decimal = precision;
    let mut bins: Vec<[Decimal; 2]> = Vec::new();
    for _alternative in 0..alternatives {
        bins.push([lower_bound, upper_bound]);
        lower_bound = upper_bound;
        upper_bound = upper_bound + precision;
    }
    println!("Bins Datastructure: {:?}", bins); //print bin array (datastructure, not intervals!)
    //pretty print
    for bin_number in 0..alternatives - 1 {
        let bin: [Decimal; 2] = bins[bin_number];
        println!("Bin{}:[{:?},{:?})", bin_number + 1, bin[0], bin[1]);
    }
    println!("Bin{}:[{:?},{:?}]", alternatives, lower_bound - precision, lower_bound);
    return bins;
}

pub(crate) fn generate_true_probability() -> f32 {
    let mut rng = rand::thread_rng();
    let true_probability: f32 = rng.gen_range(0.0..=1.0);
    println!("\nTrue probability is: {}", true_probability);
    return true_probability;
}

/*
- returns bin as viewed by the user in the application!
- to get ground truth bin in the datastructure subtract 1
 */
pub(crate) fn select_ground_truth_bin(alternatives: usize, bins: Vec<[Decimal;2]>, true_probability: f32) -> (usize, [Decimal; 2]) {
    let mut ground_truth_bin: usize = 0;
    let mut ground_truth_interval: [Decimal;2] = [dec!(0.0),dec!(1.0)];
    for alternative in 0..alternatives {
        let bin: [Decimal;2] = bins[alternative];
        if true_probability >= f32::try_from(bin[0]).unwrap() && true_probability < f32::try_from(bin[1]).unwrap() {
            ground_truth_bin = alternative + 1;
            ground_truth_interval = bin;
            break;
        }
    }
    println!("Bin{:?} = {:?} represents ground truth.\n", ground_truth_bin, ground_truth_interval);
    return (ground_truth_bin, ground_truth_interval);
}

pub(crate) fn compute_share_of_bins(max_alternatives: usize, percentage: usize) -> usize {
    let share: f32 = percentage as f32 / 100f32;
    println!("share: {}", share);
    let less_bins = max_alternatives as f32 * share;
    println!("less bins: {}", less_bins);
    let bins = less_bins.floor();
    return bins as usize;
}