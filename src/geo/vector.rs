#[allow(dead_code)]
pub struct Vector{
    magnitude: f64,
    direction: f64,
}
#[allow(dead_code)]
impl Vector {

    pub fn new(magnitude: f64, direction: f64) -> Self {
        Self { 
            magnitude,
            direction: direction.to_radians(),
        }
    }

}