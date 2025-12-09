use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Cube {
    pub x_range: RangeInclusive<isize>,
    pub y_range: RangeInclusive<isize>,
    pub z_range: RangeInclusive<isize>,
}

impl Cube {
    pub fn new(
        x_range: RangeInclusive<isize>,
        y_range: RangeInclusive<isize>,
        z_range: RangeInclusive<isize>,
    ) -> Self {
        Cube {
            x_range,
            y_range,
            z_range,
        }
    }

    pub fn drop_by(&self, z_delta: isize) -> Cube {
        Cube {
            x_range: self.x_range.clone(),
            y_range: self.y_range.clone(),
            z_range: (self.z_range.start() - z_delta)..=(self.z_range.end() - z_delta),
        }
    }

    pub fn intersects(&self, other: &Cube) -> bool {
        self.x_range.start() <= other.x_range.end()
            && self.x_range.end() >= other.x_range.start()
            && self.y_range.start() <= other.y_range.end()
            && self.y_range.end() >= other.y_range.start()
            && self.z_range.start() <= other.z_range.end()
            && self.z_range.end() >= other.z_range.start()
    }

    pub fn is_below(&self, other: &Cube) -> bool {
        // Below means we intersect on x / y plane.
        self.x_range.start() <= other.x_range.end()
            && self.x_range.end() >= other.x_range.start()
            && self.y_range.start() <= other.y_range.end()
            && self.y_range.end() >= other.y_range.start()
            // And we are lower on the z dimension.
            && self.z_range.end() < other.z_range.start()
    }

    pub fn z_difference(&self, other: &Cube) -> isize {
        // Calculate the absolute difference between the z-ranges.
        (self.z_range.start() - other.z_range.end()).abs()
    }

    pub fn is_directly_above(&self, other: &Cube) -> bool {
        // Check if the x and y ranges overlap and z ranges have a difference of exactly 1.
        self.x_range.start() <= other.x_range.end()
            && self.x_range.end() >= other.x_range.start()
            && self.y_range.start() <= other.y_range.end()
            && self.y_range.end() >= other.y_range.start()
            && *other.z_range.end() + 1 == *self.z_range.start()
    }
}

impl FromStr for Cube {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('~').collect();

        if parts.len() != 2 {
            return Err("Invalid format");
        }

        let start_str = parts[0];
        let end_str = parts[1];

        let start_coords: Vec<isize> = start_str
            .split(',')
            .map(|s| s.trim().parse::<isize>().unwrap_or_else(|_| 0))
            .collect();

        let end_coords: Vec<isize> = end_str
            .split(',')
            .map(|s| s.trim().parse::<isize>().unwrap_or_else(|_| 0))
            .collect();

        if start_coords.len() != 3 || end_coords.len() != 3 {
            return Err("Invalid format");
        }

        let x_range = start_coords[0]..=end_coords[0];
        let y_range = start_coords[1]..=end_coords[1];
        let z_range = start_coords[2]..=end_coords[2];

        Ok(Cube::new(x_range, y_range, z_range))
    }
}
