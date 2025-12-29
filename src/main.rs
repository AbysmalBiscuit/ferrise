mod ascent_data;
mod calculate_ascent;
mod cli;
mod plotting;

use crate::{
    ascent_data::{AscentSeries, InputTravelData},
    cli::Cli,
    plotting::basic_output,
};

use clap::Parser;

fn main() {
    let args = Cli::parse();
    // dbg!(&args);
    let input_travel_data: Vec<InputTravelData> = args
        .distances_angles
        .iter()
        .map(|(distance, angle)| InputTravelData::new(distance, angle, &args.angle_type))
        .collect();
    // dbg!(&input_travel_data);
    let ascent_series = AscentSeries::from(input_travel_data);
    // dbg!(&ascent_series);
    basic_output(&ascent_series);
}
