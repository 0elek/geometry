mod geo;
use geo::line_segment::LineSegment;
use geo::point::Point;
use geo::poly::Poly;

use crate::geo::line;

mod utils;


fn main() {
    let point1: Point = Point::new(0.0, 0.0);
    let point2: Point = Point::new(10.0, 0.0);
    let point3: Point = Point::new(10.0, 10.0);
    let point4: Point = Point::new(0.0, 10.0);
    let line: LineSegment = LineSegment::new(&point1, &point2);
    let poly: Poly = Poly::new(
        vec![point1, point2, point3, point4],
        vec![0, 1, 2, 3]
    ).unwrap();
    let line2: LineSegment = LineSegment::new(&Point { x: 0., y: 10. }, &Point { x: 0., y: -10. });
    let line3: LineSegment = LineSegment::new(&Point { x: -5., y: 0. }, &Point { x: 5., y: 0. });

    
    println!("{:?}", line.len());
    println!("{:?} area", poly.area());
    println!("{:?} premiter", poly.perimeter());
    println!("{:?} slope", line.slope());
    println!("{:?}", line.parallel(&line));
    println!("{:?} intercept", line2.intercept(&line3));



    let mut line4 = line::Line::new(1.0, &Point::new(10.0, 10.0));
    let line5 = line::Line::new(0.0, &Point::new(10.0, 0.0));

    println!("{:?}", line4.intercept(&line5));

    // minimize_point(&mut line4);

    println!("{:?}", line4);
    line4.minimize_point();
    println!("{:?}", line4);







}
