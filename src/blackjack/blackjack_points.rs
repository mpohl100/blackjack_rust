use super::traits::Allable;

pub struct Points {
    lower: i32,
    upper: i32,
}

impl Points {
    fn new(lower: i32, upper: i32) -> Points {
        Points { lower, upper }
    }

    fn lower(&self) -> i32 {
        self.lower
    }

    fn upper(&self) -> i32 {
        self.upper
    }

    fn to_string(&self) -> String {
        //implementation
        return String::default();
    }
}

impl Allable for Points{
    fn create_all() -> Vec<Points> {
        // implementation
        return Vec::<Points>::default();
    }
}

impl Default for Points {
    fn default() -> Points {
        Points { lower: -10000, upper: -10000 }
    }
}

impl Clone for Points {
    fn clone(&self) -> Points {
        Points { lower: self.lower, upper: self.upper }
    }
}

impl Copy for Points {}

impl PartialEq for Points {
    fn eq(&self, other: &Points) -> bool {
        self.lower == other.lower && self.upper == other.upper
    }
}

impl Eq for Points {}

impl PartialOrd for Points {
    fn partial_cmp(&self, other: &Points) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Points {
    fn cmp(&self, other: &Points) -> std::cmp::Ordering {
        match self.lower.cmp(&other.lower) {
            std::cmp::Ordering::Equal => self.upper.cmp(&other.upper),
            o => o,
        }
    }
}