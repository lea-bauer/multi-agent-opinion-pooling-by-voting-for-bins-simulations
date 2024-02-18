pub(crate) fn compute_average_epistemic_values(epistemic_values: Vec<Vec<f32>>, rounds: usize) -> Vec<f32> {
    println!("\nEpistemic values: {:?}", epistemic_values);
    let mut voting_for_bins_average: f32 = 0.0;
    let mut linear_pooling_average: f32 = 0.0;
    let mut logarithmic_pooling_average: f32 = 0.0;
    let mut convex_pooling_average: f32 = 0.0;
    for values in 0..rounds {
        let epistemics: Vec<f32> = epistemic_values[values].clone().into();
        voting_for_bins_average += epistemics[0];
        linear_pooling_average += epistemics[1];
        logarithmic_pooling_average += epistemics[2];
        convex_pooling_average += epistemics[3];
    }
    let mut averages = vec![voting_for_bins_average,linear_pooling_average,logarithmic_pooling_average,convex_pooling_average];
    for average in 0..averages.len() {
        averages[average] = averages[average] / rounds as f32;
    }
    println!("Epistemic values averages: {:?}\n", averages);
    return averages;
}