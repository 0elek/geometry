use crate::geo::point::Point;

pub fn ccw(a:Point,b:Point,c:Point) -> bool{
    
    // thanks to https://bryceboe.com/2006/10/23/line-segment-intersection-algorithm/
    
    (c.y - a.y ) * (b.x - a.x ) > (b.y - a.y ) * (c.x - a.x )
    
} 