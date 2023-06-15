use super::traits::Allable;
use super::traits::Stringable;

#[derive(Debug, Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Points {
    lower: i32,
    upper: i32,
}

impl Points {
    pub fn new(lower: i32, mut upper: i32) -> Points {
        if upper > 21{
            upper = lower;
        }
        Points { lower, upper}
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }
}

impl Stringable for Points{
    fn to_string_internal(&self) -> String {
        self.lower.to_string() + &"/".to_string() + &self.upper.to_string()
    }
}

impl Allable for Points{
    fn create_all() -> Vec<Points> {
        let mut points = std::collections::BTreeSet::new();
        for i in 2..=21 {
            points.insert(Points::new(i, i));
            points.insert(Points::new(i, i + 10 ));
        }
        points.into_iter().collect()
    }
}

#[cfg(test)]
mod points_tests {
    use super::*;

    #[test]
    fn test_new_with_upper_above_21() {
        let points = Points::new(15, 25);

        assert_eq!(points.lower(), 15);
        assert_eq!(points.upper(), 15);
    }

    #[test]
    fn test_new_with_upper_below_21() {
        let points = Points::new(8, 18);

        assert_eq!(points.lower(), 8);
        assert_eq!(points.upper(), 18);
    }

    #[test]
    fn test_to_string_internal() {
        let points = Points::new(10, 20);

        assert_eq!(points.to_string_internal(), "10/20".to_string());
    }

    #[test]
    fn test_create_all() {
        let all_points = Points::create_all();
        let expected_length = 19 + 11; // range [2-21] + 11 points with 10 added

        assert_eq!(all_points.len(), expected_length);

        let mut expected_points = std::collections::BTreeSet::new();
        for i in 2..=21 {
            expected_points.insert(Points::new(i, i));
            expected_points.insert(Points::new(i, i + 10));
        }
        let expected_points_vec: Vec<Points> = expected_points.into_iter().collect();
        println!("Len comp.: {:?} {:?}", all_points.len(), expected_points_vec.len());
        assert_eq!(all_points, expected_points_vec);
    }
}
