use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64,
    pub width: f64,
    pub height: f64,
}

impl BoundingBox {
    pub fn new(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> BoundingBox {
        if x_min > x_max { panic!("x_min value ({}) should be less than x_max value ({})", x_min, x_max); }
        if y_min > y_max { panic!("y_min value ({}) should be less than y_max value ({})", y_min, y_max); }

        let width = x_max - x_min;
        let height = y_max - y_min;

        BoundingBox { x_min, y_min, x_max, y_max, width, height }
    }

    pub fn merge(&self, bounding_box: &BoundingBox) -> BoundingBox {
        let x_min = if self.x_min < bounding_box.x_min { self.x_min } else { bounding_box.x_min };
        let y_min = if self.y_min < bounding_box.y_min { self.y_min } else { bounding_box.y_min };
        let x_max = if self.x_max > bounding_box.x_max { self.x_max } else { bounding_box.x_max };
        let y_max = if self.y_max > bounding_box.y_max { self.y_max } else { bounding_box.y_max };

        BoundingBox::new(x_min, y_min, x_max, y_max)
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max
    }

    pub fn intersects(&self, bounding_box: &BoundingBox) -> bool {
        self.contains(bounding_box.x_min, bounding_box.y_max)
            || self.contains(bounding_box.x_min, bounding_box.y_min)
            || self.contains(bounding_box.x_max, bounding_box.y_max)
            || self.contains(bounding_box.x_max, bounding_box.y_min)
            || bounding_box.contains(self.x_min, self.y_max)
            || bounding_box.contains(self.x_min, self.y_min)
            || bounding_box.contains(self.x_max, self.y_max)
            || bounding_box.contains(self.x_max, self.y_min)
    }

    pub fn get_intersection(&self, bounding_box: &BoundingBox) -> Option<BoundingBox> {
        if !self.intersects(bounding_box) {
            return None;
        }

        let x_min = if self.x_min > bounding_box.x_min { self.x_min } else { bounding_box.x_min };
        let x_max = if self.x_max < bounding_box.x_max { self.x_max } else { bounding_box.x_max };
        let y_min = if self.y_min > bounding_box.y_min { self.y_min } else { bounding_box.y_min };
        let y_max = if self.y_max < bounding_box.y_max { self.y_max } else { bounding_box.y_max };

        Some(BoundingBox::new(x_min, y_min, x_max, y_max))
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", json!(self))
    }
}

impl PartialEq for BoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.x_min == other.x_min &&
            self.y_min == other.y_min &&
            self.x_max == other.x_max &&
            self.y_max == other.y_max &&
            self.width == other.width &&
            self.height == other.height
    }
}

impl Eq for BoundingBox {}