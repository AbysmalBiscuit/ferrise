use crate::ascent_data::AscentSeries;

pub fn basic_output(ascent_series: &AscentSeries) {
    println!(
        "\
altitude: {altitude}
total_ascent: {total_ascent}
total_descent: {total_descent}",
        altitude = ascent_series.get_final_elevation(),
        total_ascent = ascent_series.get_total_ascent(),
        total_descent = ascent_series.get_total_descent()
    );
}
