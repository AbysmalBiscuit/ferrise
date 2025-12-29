use std::num::ParseFloatError;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    /// Type of Angle data being used. Either degrees (default) or radians
    #[arg(short, long, value_enum, default_value_t = AngleType::Degrees)]
    pub angle_type: AngleType,

    /// Comma separated list of distance,angle pairs to use for calculations
    #[arg(
        value_delimiter = ' ',
        value_parser = parse_distances_angles_pair,
        required = true,
    )]
    pub distances_angles: Vec<(f64, f64)>,
}

/// Type of angle input data.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, ValueEnum)]
pub enum AngleType {
    Degrees,
    Radians,
}

/// Error enum used by functions that parse elevation change series.
#[derive(thiserror::Error, Debug)]
pub(crate) enum CliParseError {
    #[error("Distance cannot be empty")]
    MissingDistance,

    #[error("Angle cannot be empty")]
    MissingAngle,

    #[error("Failed to convert to float {0}")]
    ParseFloatError(String),
}

impl From<ParseFloatError> for CliParseError {
    fn from(value: ParseFloatError) -> Self {
        CliParseError::ParseFloatError(value.to_string())
    }
}

pub fn parse_distances_angles(
    distances_angles: Vec<String>,
) -> Result<Vec<(f64, f64)>, CliParseError> {
    distances_angles
        .iter()
        .map(|pair| {
            let mut split = pair.split(",");
            let distance: f64 = split
                .next()
                .ok_or(CliParseError::MissingDistance)?
                .parse::<f64>()?;
            let angle: f64 = split
                .next()
                .ok_or(CliParseError::MissingAngle)?
                .parse::<f64>()?;
            Ok((distance, angle))
        })
        .collect()
}

/// Parses input distance-angle pair data.
///
/// # Arguments
///
/// * `distances_angles_pair` - &str single pair of distance and angle to parse. The values should
///   be separated by a comma.
///
/// # Returns
/// Result<(f64, f64), CliParseError> Result enum variant containing either the successfully parsed
/// numeric tuple or an error.
///
/// # Examples
/// ```rust
/// let result = parse_distances_angles_pair("800,5"); // result = (800_f64, 5_f64);
/// ```
pub fn parse_distances_angles_pair(
    distances_angles_pair: &str,
) -> Result<(f64, f64), CliParseError> {
    let mut split = distances_angles_pair.split(",");
    let distance: f64 = split
        .next()
        .ok_or(CliParseError::MissingDistance)?
        .parse::<f64>()?;
    let angle: f64 = split
        .next()
        .ok_or(CliParseError::MissingAngle)?
        .parse::<f64>()?;
    Ok((distance, angle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_distances_angles() {
        let expected = vec![(800_f64, 5_f64), (300_f64, -1_f64), (1000_f64, 0_f64)];
        let distances_angles = "800,5 300,-1 1000,0"
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let result = parse_distances_angles(distances_angles).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_distances_angles_pair() {
        let result = parse_distances_angles_pair("800,5");
        assert_eq!(result.unwrap(), (800_f64, 5_f64));
    }
}
