use std::error::Error;

use crate::geo::point::Point;

#[derive(Debug, Clone)]
pub struct Poly{
    points: Vec<Point>,
    connections: Vec<i32>
}

impl Poly {

    pub fn new(points: Vec<Point>, connections: Vec<i32>) -> Result<Self, Box<dyn Error>> {
        if connections.len() != points.len() {
            return Err("The number of connections must be equal to the number of points".into());
        } 
        if connections.iter().any(|&x| x < 0) {
            return Err("All connections must be positive".into());
        }
        if connections.iter().any(|&x| x as usize >= points.len()) {
            return Err("All connections must be less than the number of points".into());
        } 
        else {
            return Ok(Self { points, connections });
        }
    }
    
    pub fn area(&self) -> f64 {
        let n: usize = self.points.len();
        let mut area: f64 = 0.0;

        for i in 0..n {
            let current: &Point = &self.points[i];
            let next: &Point = &self.points[(i + 1) % n];

            area += (current.x * next.y) - (current.y * next.x);
        }

        area / 2.0
    }

    pub fn perimeter(&self) -> f64 {
        let n = self.connections.len();
        let mut perimeter = 0.0;

        for i in 0..n {
            let current_index = self.connections[i] as usize;
            let next_index = self.connections[(i + 1) % n] as usize;

            let current = &self.points[current_index];
            let next = &self.points[next_index];

            perimeter += current.distance_between(next);
        }

        perimeter
    }

}