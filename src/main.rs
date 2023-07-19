mod geo;
use geo::line_segment::LineSegment;
use geo::point::Point;
use geo::poly::Poly;
use geo::vector::Vector;

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

    let segment = LineSegment::new(&Point::new(0.0, 0.0), &Point::new(10.0, 10.0));
    println!("{:?}", segment.midpoint());


    


    let corner1 = Point::new(0.0, 0.0);
    let corner2 = Point::new(10.0, 0.0);
    let corner3 = Point::new(10.0, 10.0);
    let corner4 = Point::new(0.0, 10.0);
    let corner5 = Point::new(5.0, 15.0);

    let poly = Poly::new(
        vec![corner1, corner2, corner3, corner4, corner5],
        vec![0, 1, 2, 3, 4]
    ).unwrap();

    println!("{:?}", poly.area());
    println!("{:?}", poly.center());

    let up_deg: f64 = 90.0; 
    let down_deg: f64 = -90.0;

    let vec_left: Vector = geo::vector::Vector::new(1.0, up_deg);
    let vec_right: Vector = geo::vector::Vector::new(1.0, down_deg);

    let vec_res: Vector = vec_left.subtract(&vec_right);
    println!("{:?}", vec_res);

}
