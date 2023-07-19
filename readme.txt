a non complete 2d geometry lib

Geo
    - Point
        - distance_between <Point>
        - distance_from <x,y>
    - Line
        - intercept <Line> -> Point | null
        - parallel <Line> -> boolean
        - perpendicular <Line> -> bool
        - minimize_point "minimize" the point to the origin
    - Line Segment
        - len -> number
        - slope -> number
        - midpoint -> Point
        - bisect -> 2 Line
        - to_line -> Line
        Add intercept, parallel, perpendicular
    - Poly
        - area -> number
        - perimeter -> number
        - center -> Point
    - Circle
        - area -> number
        - circumference -> number
        - diameter -> number
        - contains <Point> -> bool
        - tangents -> bool
        - on_radius <Point> -> bool
        - Add intersect (line, circle, poly)
    - Add Ray
    - Add Ellipse


Movement
    - Vector
        - y_component -> number
        - x_component -> number
        - scale <number> -> Vector
        - add <Vector> -> Vector

    - Add more shortcuts, mass, force

consts 
    - Gravity