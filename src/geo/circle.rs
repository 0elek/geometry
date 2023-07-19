use crate::geo::point::Point;
use crate::geo::line::Line;

pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

trait CircleTrait {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
    fn diameter(&self) -> f64;
    fn contains(&self, point: &Point) -> bool;
    fn tangets(&self, other: &Line) -> bool;
    fn on_radius(&self, point: &Point) -> bool;
}

impl CircleTrait for Circle {

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    fn contains(&self, point: &Point) -> bool {
        let (x, y) = point.unpack();
        let (a, b) = self.center.unpack();

        (x - a).powi(2) + (y - b).powi(2) <= self.radius.powi(2)
    }

    fn tangets(&self, other:&Line) -> bool {
        let (x, y) = self.center.unpack();
        let (a, b) = other.point.unpack();

        (x - a).powi(2) + (y - b).powi(2) == self.radius.powi(2)
    }

    fn on_radius(&self, point: &Point) -> bool{
        point.distance_between(&self.center) == self.radius
    }
    

}