use crate::{calculate_ascent::calculate_ascent_radians, cli::AngleType};

#[derive(Debug)]
pub struct AscentSeries {
    data: Vec<TravelData>,
}

impl AscentSeries {
    pub fn get_final_elevation(&self) -> f64 {
        self.data
            .iter()
            .map(|travel_data| travel_data.elevation_change)
            .sum()
    }

    pub fn get_total_ascent(&self) -> f64 {
        self.data
            .iter()
            .filter(|travel_data| travel_data.elevation_change.is_sign_positive())
            .map(|travel_data| travel_data.elevation_change)
            .sum()
    }

    pub fn get_total_descent(&self) -> f64 {
        self.data
            .iter()
            .filter(|travel_data| travel_data.elevation_change.is_sign_negative())
            .map(|travel_data| travel_data.elevation_change)
            .sum()
    }
}

impl From<Vec<InputTravelData>> for AscentSeries {
    fn from(value: Vec<InputTravelData>) -> Self {
        AscentSeries {
            data: value.iter().map(TravelData::from).collect(),
        }
    }
}

#[derive(Debug)]
pub struct InputTravelData {
    distance: f64,
    angle: f64,
}

impl InputTravelData {
    pub fn new(distance: &f64, angle: &f64, angle_type: &AngleType) -> Self {
        let mut angle: f64 = *angle;
        if angle_type == &AngleType::Degrees {
            angle = angle.to_radians();
        }
        InputTravelData {
            distance: *distance,
            angle,
        }
    }
}

#[derive(Debug)]
pub struct TravelData {
    distance: f64,
    angle: f64,
    elevation_change: f64,
}

impl From<InputTravelData> for TravelData {
    fn from(value: InputTravelData) -> Self {
        TravelData {
            distance: value.distance,
            angle: value.angle,
            elevation_change: calculate_ascent_radians(value.distance, value.angle),
        }
    }
}
impl From<&InputTravelData> for TravelData {
    fn from(value: &InputTravelData) -> Self {
        TravelData {
            distance: value.distance,
            angle: value.angle,
            elevation_change: calculate_ascent_radians(value.distance, value.angle),
        }
    }
}
