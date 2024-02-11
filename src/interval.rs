pub struct Interval {
    pub min: f32,
    pub max: f32
}

impl Interval {
    pub fn contains(self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(self, x: f32) -> f32 {
        if (x < self.min) {
            self.min
        } else if {x > self.max} {
            self.max
        } else {
            x
        }
    }
}

impl std::clone::Clone for Interval { // interval.clone();
    fn clone(&self) -> Self {
        Interval {
            min: self.min.clone(),
            max: self.max.clone()
        }
    }
}

impl std::marker::Copy for Interval {}

pub const EMPTY: Interval = Interval{min: f32::INFINITY, max: -f32::INFINITY};
pub const UNIVERSE: Interval = Interval{min: -f32::INFINITY, max: f32::INFINITY};