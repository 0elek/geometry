use std::os::fd::AsRawFd;

use crate::geo::point::Point;

#[derive(Debug, Clone)]
pub struct Line {
    point1: Point,
    point2: Point
}

impl Line {
    pub fn new(point1: &Point, point2: &Point) -> Self{
        return Line { 
            point1: point1.clone(), 
            point2: point2.clone() };
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
    //     let ((line_1_x1, line_1_y1), (line_1_x2, line_1_y2)) = self.unpack();
    //     let ((line_2_x1, line_2_y1), (line_2_x2, line_2_y2)) = other.unpack();

    //     let a1 = line_1_y1 - line_1_y2;
    //     let a2 = line_2_y1 - line_2_y2;
    //     let b1 = line_1_x1 - line_1_x2;
    //     let b2 = line_2_x1 - line_2_x2;
    //     let c1 = a1 * line_1_x1 + b1 * line_1_y1;
    //     let c2 = a2 * line_2_x1 + b2 * line_2_y1;

    //     let det = a1 * b2 - a2 * b1;

    //     if det == 0. {
    //         print!("pararell");
    //         return false;
    //     }
    
    //     return true;
    todo!()
    }
    
    
    fn unpack (&self) -> ((f64, f64), (f64, f64)) {
        (self.point1.unpack(), self.point2.unpack())
    }
    
}