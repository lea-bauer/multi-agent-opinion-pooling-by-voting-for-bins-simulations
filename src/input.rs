use std::io::{Read, stdin, stdout, Write};

pub(crate) fn enter_voters() -> f32 {
    println!("\nPlease enter number of voters n ≥ 1:");
    let mut number_of_voters = String::new();
    stdin()
        .read_line(&mut number_of_voters)
        .expect("Please enter number of voters!");
    let n: f32 = number_of_voters
        .trim()
        .parse()
        .expect("Please enter a natural number!");
    //assert!(n >= 1, "n: {}", n);
    println!("Number of voters n: {}\n", n);
    return n;
}

pub(crate) fn enter_reliability() -> f32 {
    println!("Please enter reliability 𝜟𝒑 ∈ (0,1):");
    let mut reliability = String::new();
    stdin()
        .read_line(&mut reliability)
        .expect("Please enter reliability!");
    let delta_p: f32 = reliability
        .trim()
        .parse()
        .expect("Please enter a real number between 0 and 1!");
    println!("Reliability 𝜟𝒑: {}\n", delta_p);
    return delta_p;
}

pub(crate) fn enter_pmin() -> f32 {
    println!("Please enter guaranteed worst case success probability 𝑃_min ∈ (0,1):");
    let mut worst_case_success_probability = String::new();
    stdin()
    .read_line(&mut worst_case_success_probability)
    .expect("Please enter guaranteed worst case success probability 𝑃_min!");
    let p_min: f32 = worst_case_success_probability
    .trim()
    .parse()
    .expect("Please enter a real number between 0 and 1!");
    println!("Worst case success probability 𝑃_min: {}\n", p_min);
    return p_min;
}

pub(crate) fn enter_rounds() -> usize {
    println!("Please enter number of rounds r ≥ 1:");
    let mut number_of_rounds = String::new();
    stdin()
        .read_line(&mut number_of_rounds)
        .expect("Please enter number of rounds!");
    let r: usize = number_of_rounds
        .trim()
        .parse()
        .expect("Please enter a natural number!");
    println!("Number of rounds r: {}\n", r);
    return r;
}

pub(crate) fn enter_priority() -> f32 {
    println!("Please enter priority 𝛼 given to the two principles 1. Avoid Error, 2. Seek Truth:");
    let mut priority = String::new();
    stdin()
        .read_line(&mut priority)
        .expect("Please enter priority 𝛼!");
    let alpha: f32 = priority
        .trim()
        .parse()
        .expect("Please enter a real number between 0 and 1!");
    println!("Priority 𝛼: {}", alpha);
    return alpha;
}

pub(crate) fn enter_percentage_of_bins(max_bins: usize) -> usize {
    println!("\nPlease enter the percentage of max. {} bins you want to use:", max_bins);
    let mut share = String::new();
    stdin()
        .read_line(&mut share)
        .expect("Please enter the share of bins!");
    let percentage: usize = share
        .trim()
        .parse()
        .expect("Please enter an integer between 0 and 100!");
    println!("Share: {}", percentage);
    return percentage;
}

pub(crate) fn pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to proceed ...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}