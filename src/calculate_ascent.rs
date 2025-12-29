pub fn calculate_ascent_degrees(distance: f64, angle: f64) -> f64 {
    distance * angle.to_radians().tan()
}

pub fn calculate_ascent_radians(distance: f64, angle: f64) -> f64 {
    distance * angle.tan()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_ascent_degrees() {
        let result = calculate_ascent_degrees(1000_f64, 5_f64);
        let result_rounded: u64 = (result * 10.0).round() as u64;
        assert_eq!(result_rounded, 875);
    }

    #[test]
    fn test_calculate_ascent_radians() {
        let result = calculate_ascent_radians(1000_f64, 5.0_f64.to_radians());
        let result_rounded: u64 = (result * 10.0).round() as u64;
        assert_eq!(result_rounded, 875);
    }
}
