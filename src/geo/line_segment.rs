use crate::utils::ccw;
use crate::geo::line::Line;

use crate::geo::point::Point;

#[derive(Debug, Clone)]
pub struct LineSegment {
    pub point1: Point,
    pub point2: Point
}

#[allow(dead_code)]
impl LineSegment {
    pub fn new(point1: &Point, point2: &Point) -> Self{
        LineSegment { 
            point1: *point1, 
            point2: *point2 }
    }

    pub fn len(&self) -> f64{
        let (x1, y1) = self.point1.unpack();
        let (x2, y2) = self.point2.unpack();

        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()

    }

    pub fn slope(&self) -> f64{

        let (x1, y1) = self.point1.unpack();
        let (x2, y2) = self.point2.unpack();

        (y2 - y1) / (x2 - x1)

    }

    pub fn intercept(&self, other: &Self) -> bool {
        let a = self.point1;
        let b = self.point2;

        let c = other.point1;
        let d = other.point2;

        ccw(a,c,d) != ccw(b,c,d) && ccw(a,b,c) != ccw(a,b,d)

    }

    pub fn parallel(&self, other: &Self) -> bool {
        self.slope() == other.slope()
    }

    pub fn midpoint(&self) -> Point{
        let (x1, y1) = self.point1.unpack();
        let (x2, y2) = self.point2.unpack();

        Point::new((x1 + x2) / 2.0, (y1 + y2) / 2.0)
    }

    pub fn bisect(&self) -> (LineSegment, LineSegment) {
        let midpoint = self.midpoint();

        let line1 = LineSegment::new(&self.point1, &midpoint);
        let line2 = LineSegment::new(&self.point2, &midpoint);

        (line1, line2)
    }

    pub fn to_line(&self) -> Line{
        let slope: f64 = self.slope();
        let point: Point = self.point1;

        let mut new_line: Line = Line::new(slope, &point);
        new_line.minimize_point();
        new_line
    }
    
}