use super::traits::Allable;
use super::traits::Stringable;

#[derive(Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
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