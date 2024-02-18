use plotters::prelude::*;
use plotters::style::full_palette::{AMBER_300, TEAL_900};

pub(crate) fn draw_graphs(ip_scoring: Vec<(Vec<[f32;2]>, Vec<f32>)>, average_epistemic_values: Vec<f32>, rounds: usize, voters_belief: Vec<Vec<[f32;2]>>) {
    draw_graph_without_average_voter_belief(ip_scoring.clone(), average_epistemic_values.clone(), rounds);
    draw_graph_with_average_voter_belief(ip_scoring.clone(), average_epistemic_values.clone(), rounds, voters_belief);
}
pub(crate) fn draw_graph_with_average_voter_belief(ip_scoring: Vec<(Vec<[f32;2]>, Vec<f32>)>, average_epistemic_values: Vec<f32>, rounds: usize, voters_belief: Vec<Vec<[f32;2]>>) {
    let x_labels = ["","","VfB", "Lin", "Log", "Conv", ""];
    //create coordinate system
    let background = BitMapBackend::new("graphs/graph_with_average_voter_belief.png", (250, 250))
        .into_drawing_area();
    background.fill(&WHITE).unwrap();
    let mut coordinate_system = ChartBuilder::on(&background)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .margin_top(10)
        .margin_right(5)
        .build_cartesian_2d(0usize..6usize, 0f32..1f32)
        .unwrap();
    coordinate_system
        .configure_mesh()
        .label_style(("avenir", 15))
        .axis_desc_style(("avenir", 30))
        .x_label_formatter(&|x| format!("{}", x_labels[*x]))
        .draw()
        .unwrap();
    //draw data
    //0. prepare coordinates
    //average imprecise beliefs per round
    let mut average_voters_beliefs: Vec<[f32;2]> = Vec::new();
    println!("voters beliefs: {:?}", voters_belief);
    for _round in 0..rounds {
        let mut average: [f32;2] = [0.0,0.0];
        let beliefs_one_round = voters_belief.get(0).unwrap();
        println!("beliefs one round: {:?}", beliefs_one_round);
        for voter in 0..beliefs_one_round.len() {
            let belief = beliefs_one_round[voter];
            println!("voter {:?}", belief);
            average[0] += belief[0];
            average[1] += belief[1];
        }
        println!("average: {:?}", average);
        average[0] = average[0] / beliefs_one_round.len() as f32;
        average[1] = average[1] / beliefs_one_round.len() as f32;
        println!("average: {:?}", average);
        average_voters_beliefs.push(average);
    }
    println!("average voters beliefs: {:?}", average_voters_beliefs);
    let mut overall_average: Vec<[f32;2]> = vec![];
    let mut lower_average_belief: f32 = 0.0;
    let mut upper_average_belief: f32 = 0.0;
    for average in &average_voters_beliefs {
        lower_average_belief += average[0];
        upper_average_belief += average[1];
    }
    lower_average_belief = lower_average_belief / rounds as f32;
    upper_average_belief = upper_average_belief / rounds as f32;
    overall_average.push([lower_average_belief,upper_average_belief]);
    //average aggregated group beliefs all rounds per method
    let mut average_beliefs: Vec<[f32;2]> = vec![];
    let mut average_epistemic_values_coordinates: Vec<(usize,f32)> = vec![];
    for method in 0..4 {
        let mut average_belief: [f32; 2] = [0.0,0.0];
        for round in 0..rounds  {
            let belief = ip_scoring[round].0[method];
            average_belief[0] += belief[0];
            average_belief[1] += belief[1];
        }
        average_belief[0] = average_belief[0] / (rounds as f32);
        average_belief[1] = average_belief[1] / (rounds as f32);
        average_beliefs.push(average_belief);
        let average_epistemic_value = average_epistemic_values[method];
        average_epistemic_values_coordinates.push((method + 2, average_epistemic_value));
    }
    //1. draw average voters' beliefs
    coordinate_system.draw_series((1usize..).zip(overall_average.iter()).map(|(x, y)| {
        let mut bar: Rectangle<(usize,f32)> = Rectangle::new([(x, y[0]), (x, y[1])], BLUE.filled());
        bar.set_margin(0, 0, 3, 3);
        bar
    })).unwrap();
    //2. draw average aggregated beliefs
    coordinate_system.draw_series((2usize..).zip(average_beliefs.iter()).map(|(x, y)| {
        let mut bar: Rectangle<(usize,f32)> = Rectangle::new([(x, y[0]), (x, y[1])], AMBER_300.filled());
        bar.set_margin(0, 0, 3, 3);
        bar
    })).unwrap();
    coordinate_system.draw_series((2usize..).zip(average_beliefs.iter()).map(|(x, y)| {
        let mut frame: Rectangle<(usize,f32)> = Rectangle::new([(x, y[0] - 0.02), (x, y[1] + 0.02)], RED);
        frame.set_margin(0,0,7,7);
        frame
    })).unwrap();
    //3. draw average epistemic values
    for _method in 0..4 {
        coordinate_system.draw_series(average_epistemic_values_coordinates.iter().map(|(x,y)| Circle::new((*x,*y), 5.0, TEAL_900.filled()))).unwrap();
    }
}

pub(crate) fn draw_graph_without_average_voter_belief(ip_scoring: Vec<(Vec<[f32;2]>, Vec<f32>)>, average_epistemic_values: Vec<f32>, rounds: usize) {
    let x_labels = ["","VfB", "Lin", "Log", "Conv",""];
    //create coordinate system
    let background = BitMapBackend::new("graphs/graph.png", (250, 250))
        .into_drawing_area();
    background.fill(&WHITE).unwrap();
    let mut coordinate_system = ChartBuilder::on(&background)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .margin_top(10)
        .margin_right(5)
        .build_cartesian_2d(0usize..5usize, 0f32..1f32)
        .unwrap();
    coordinate_system
        .configure_mesh()
        .label_style(("avenir", 15))
        .axis_desc_style(("avenir", 30))
        .x_label_formatter(&|x| format!("{}", x_labels[*x]))
        .draw()
        .unwrap();
    //draw data
    //0. prepare coordinates
    let mut average_beliefs: Vec<[f32;2]> = vec![];
    let mut average_epistemic_values_coordinates: Vec<(usize,f32)> = vec![];
    for method in 0..4 {
        let mut average_belief: [f32; 2] = [0.0,0.0];
        for round in 0..rounds  {
            let belief = ip_scoring[round].0[method];
            average_belief[0] += belief[0];
            average_belief[1] += belief[1];
        }
        average_belief[0] = average_belief[0] / (rounds as f32);
        average_belief[1] = average_belief[1] / (rounds as f32);
        average_beliefs.push(average_belief);
        let average_epistemic_value = average_epistemic_values[method];
        average_epistemic_values_coordinates.push((method + 1, average_epistemic_value));
    }
    //1. draw average beliefs
    coordinate_system.draw_series((1usize..).zip(average_beliefs.iter()).map(|(x, y)| {
        let mut bar: Rectangle<(usize,f32)> = Rectangle::new([(x, y[0]), (x, y[1])], AMBER_300.filled());
        bar.set_margin(0, 0, 3, 3);
        bar
    })).unwrap();
    coordinate_system.draw_series((1usize..).zip(average_beliefs.iter()).map(|(x, y)| {
        let mut frame: Rectangle<(usize,f32)> = Rectangle::new([(x, y[0] - 0.02), (x, y[1] + 0.02)], RED);
        frame.set_margin(0,0,7,7);
        frame
    })).unwrap();
    //2. draw average epistemic values
    for _method in 0..4 {
        coordinate_system.draw_series(average_epistemic_values_coordinates.iter().map(|(x,y)| Circle::new((*x,*y), 5.0, TEAL_900.filled()))).unwrap();
    }
}

