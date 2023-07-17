pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Vector{
    magnitude: f64,
    direction: f64,
}

impl Vector {

    pub fn new(magnitude: f64, direction: f64) -> Self {
        Self { 
            magnitude,
            direction: direction.to_radians(),
        }
    }

}