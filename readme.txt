a non complete 2d geometry lib

Geo
    - Point
        - distance_between <Point>
        - distance_from <x,y>
    - Line
        - intercept <Line> -> Point | null
        - parallel <Line> -> boolean
        - perpendicular <Line> -> boolean
        - minimize_point "minimize" the point to the origin
    - Line Segment
        - len -> number
        - slope -> number
        - midpoint -> Point
        - bisect -> 2 Line
        - to_line -> Line
    - Poly
        - area -> number
        - perimeter -> number
        - center -> Point
    - add Ray
    - Add Circle
    - Add Ellipse


Movement
    - Vector
        - y_component -> number
        - x_component -> number

    - Add more shortcuts, mass, force

consts 
    - Gravity
    - 