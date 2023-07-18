#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
#[allow(dead_code)]
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn unpack(&self)-> (f64, f64){

        (self.x, self.y)
    }

    pub fn distance_between(&self, other: &Self) -> f64{
        let (x1, y1) = self.unpack();
        let (x2, y2) = other.unpack();

        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }

    pub fn distance_from(&self, x: f64, y: f64) -> f64{
        let (x1, y1) = self.unpack();

        ((x1 - x).powi(2) + (y1 - y).powi(2)).sqrt()
    }

    pub fn distance_from_origin(&self) -> f64{
        self.distance_from(0.0, 0.0)
    }

}