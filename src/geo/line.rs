use crate::geo::point::Point;
#[derive(Debug, Clone)]
pub struct Line {
    pub slope: f64,
    pub point: Point,
}
#[allow(dead_code)]
pub enum Keep {
    Point1,
    Point2,
}
#[allow(dead_code)]
impl Line {
    pub fn new(slope: f64, point: &Point) -> Self {
        Line {
            slope,
            point: *point,
        }
    }

    pub fn intercept(&self, other: &Self) -> Option<Point> {
        let (a, b) = self.point.unpack();
        let (c, d) = other.point.unpack();
        let e = self.slope;
        let f = other.slope;

        if e == f {
            return None;
        }

        let x = (d - b + e * a - f * c) / (e - f);
        let y = e * (x - a) + b;

        Some(Point::new(x, y))
    }
    pub fn parallel(&self, other: &Self) -> bool {
        self.slope == other.slope
    }

    pub fn perpendicular(&self, other: &Self) -> bool {
        self.slope * other.slope == -1.0
    }

    pub fn minimize_point(&mut self) {
        let (x, y) = self.point.unpack();

        let x = if self.slope == 0.0 {
            0.0
        } else {
            x - (y / self.slope)
        };
        let y = if self.slope == 0.0 { y } else { 0.0 };

        self.point = Point::new(x, y);
    }

}
