use log::debug;

/// Represents a point in 2D space with x and y coordinates
#[derive(Clone, Copy, Debug, Default, PartialEq,serde::Deserialize, serde::Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Calculate the distance between two points
///
/// # Arguments
///
/// * `p1` - The first point
/// * `p2` - The second point
///
/// # Returns
///
/// The distance between the two points
pub fn distance(p1: Point, p2: Point) -> f32 {
    ((p1.x - p2.x).pow(2) as f32 + (p1.y - p2.y).pow(2) as f32).sqrt()
}

/// Calculate the angle between two lines in radians
///
/// # Arguments
///
/// * `line1` - The first line
/// * `line2` - The second line
///
/// # Returns
///
/// The angle between the two lines
pub fn angle_between(point1: Point, point2: Point, point3: Point, point4: Point) -> f32 {
    let angle = angle(point1, point2, point3, point4);
    log::debug!("Angle: {}", angle);
    angle
}

/// Calculate the angle between two lines in radians
///
/// # Arguments
///
/// * `p1` - The first point of the first line
/// * `p2` - The second point of the first line
/// * `p3` - The first point of the second line
/// * `p4` - The second point of the second line
///
/// # Returns
///
/// The angle between the two lines
fn angle(p1: Point, p2: Point, p3: Point, p4: Point) -> f32 {
    let x1 = p2.x - p1.x;
    let y1 = p2.y - p1.y;
    let x2 = p4.x - p3.x;
    let y2 = p4.y - p3.y;
    let dot = (x1 * x2 + y1 * y2) as f32;
    let det = (x1 * y2 - y1 * x2) as f32;
    det.atan2(dot)
}

/// Calculate the perpendicular distance from a point to a line segment
///
/// # Arguments
///
/// * `p` - The point
/// * `p1` - The first point of the line segment
/// * `p2` - The second point of the line segment
///
/// # Returns
///
/// The perpendicular distance from the point to the line segment
pub fn perpendicular_distance(p: Point, point1: Point, point2: Point) -> f32 {
    let x0 = p.x as f32;
    let y0 = p.y as f32;
    let x1 = point1.x as f32;
    let y1 = point1.y as f32;
    let x2 = point2.x as f32;
    let y2 = point2.y as f32;

    let num = ((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1).abs();
    let den = ((y2 - y1).powi(2) + (x2 - x1).powi(2)).sqrt();
    num / den
}

/// Calculate the distance from a point to a drawn part of a segment
///
/// # Arguments
///
/// * `p` - The point
/// * `p1` - The first point of the line segment
/// * `p2` - The second point of the line segment
///
/// # Returns
///
/// The distance from a point to a drawn part of a segment
pub fn distance_to_segment(point: Point, point1: Point, point2: Point) -> f32 {
    let perp_distance = perpendicular_distance(point, point1, point2);

    let distance_to_p1 = distance(point, point1);
    let distance_to_p2 = distance(point, point2);

    let distance = perp_distance.min(distance_to_p1).min(distance_to_p2);

    distance
}

/// Calculate the distance from a point to a drawn part of a halfline
///
/// # Arguments
///
/// * `p` - The point
/// * `p1` - The first point of the line halfline
/// * `p2` - The second point of the line halfline
///
/// # Returns
///
/// The distance from a point to a drawn part of a halfline
pub fn distance_to_half_line(point: Point, point1: Point, point2: Point) -> f32 {
    let perp_distance = perpendicular_distance(point, point1, point2);

    let distance_to_p1 = distance(point, point1);
    let distance = perp_distance.min(distance_to_p1);

    distance
}


/// Find line extreme coordinates from two points and axis limits
///
/// # Arguments
///
/// * `p1` - The first point
/// * `p2` - The second point
/// * `x_min` - The minimum x-axis limit
/// * `x_max` - The maximum x-axis limit
/// * `y_min` - The minimum y-axis limit
/// * `y_max` - The maximum y-axis limit
///
/// # Returns
///
/// The extreme coordinates of the line
pub fn find_line_extreme_coordinates(
    p1: Point,
    p2: Point,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
) -> (Point, Point) {
    if p1.x == p2.x {
        return (
            Point {
                x: p1.x,
                y: y_min as i32,
            },
            Point {
                x: p1.x,
                y: y_max as i32,
            },
        );
    }
    let slope = (p2.y - p1.y) as f32 / (p2.x - p1.x) as f32;
    let intercept = p1.y as f32 - slope * p1.x as f32;

    let mut points = Vec::new();

    // Intersection with y = y_min
    if slope != 0. {
        let x = (y_min - intercept) / slope;
        if x >= x_min && x <= x_max {
            points.push(Point {
                x: x as i32,
                y: y_min as i32,
            });
        }
    }

    // Intersection with y = y_max
    if slope != 0. {
        let x = (y_max - intercept) / slope;
        if x >= x_min && x <= x_max {
            points.push(Point {
                x: x as i32,
                y: y_max as i32,
            });
        }
    }

    // Intersection with x = x_min
    let y = slope * x_min + intercept;
    if y >= y_min && y <= y_max {
        points.push(Point {
            x: x_min as i32,
            y: y as i32,
        });
    }

    // Intersection with x = x_max
    let y = slope * x_max + intercept;
    if y >= y_min && y <= y_max {
        points.push(Point {
            x: x_max as i32,
            y: y as i32,
        });
    }

    // Ensure we have exactly two points
    if points.len() != 2 {
        panic!("Line does not intersect the bounding box in exactly two points");
    }

    (points[0], points[1])
}

/// Find the coordinates of a line parallel to a given line and passing through a specific point
///
/// # Arguments
///
/// * `point` - The specific point
/// * `point1` - The given line first point
/// * `point2` - The given line second point
///
/// # Returns
///
/// The coordinates of the line parallel to the given line and passing through the specific point
pub fn parallel_line(point: Point, point1: Point, point2: Point) -> (Point, Point) {
    if point1.x == point2.x {
        return (Point { x: point.x, y: 0 }, Point { x: point.x, y: 100 });
    }

    let slope = (point2.y - point1.y) as f32 / (point2.x - point1.x) as f32;
    let intercept = point.y as f32 - slope * point.x as f32;

    let x1 = 0;
    let y1 = (slope * x1 as f32 + intercept) as i32;
    let x2 = 1000;
    let y2 = (slope * x2 as f32 + intercept) as i32;

    (Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
}

/// Find the coordinates of a line perpendicular to a given line and passing through a specific point
///
/// # Arguments
///
/// * `point` - The specific point
/// * `point1` - The given line first point
/// * `point2` - The given line second point
///
/// # Returns
///
/// The coordinates of the line perpendicular to the given line and passing through the specific point
pub fn perpendicular_line(point: Point, point1: Point, point2: Point) -> (Point, Point) {
    if point1.x == point2.x {
        return (Point { x: point.x, y: 0 }, Point { x: point.x, y: 100 });
    }

    let slope = (point2.y - point1.y) as f32 / (point2.x - point1.x) as f32;
    let perpendicular_slope = -1. / slope;
    let intercept = point.y as f32 - perpendicular_slope * point.x as f32;

    let x1 = 0;
    let y1 = (perpendicular_slope * x1 as f32 + intercept) as i32;
    let x2 = 1000;
    let y2 = (perpendicular_slope * x2 as f32 + intercept) as i32;

    (Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
}

/// Find the median line between two points
///
/// # Arguments
///
/// * `p1` - The first point
/// * `p2` - The second point
///
/// # Returns
///
/// The median line between the two points
pub fn median_line(p1: Point, p2: Point) -> (Point, Point) {
    let x = (p1.x + p2.x) / 2;
    let y = (p1.y + p2.y) / 2;

    (
        Point { x, y },
        Point {
            x: x + (p2.y - p1.y),
            y: y + (p1.x - p2.x),
        },
    )
}

/// Rotate a line coordinates around a point from an angle in degrees
///
/// # Arguments
///
/// * `line` - The line to rotate
/// * `point` - The point to rotate around
/// * `angle` - The angle in degrees
///
/// # Returns
///
/// The rotated line
pub fn rotate_line(
    point1: Point,
    point2: Point,
    rotation_center: Point,
    angle: f32,
) -> (Point, Point) {
    debug!("Angle to rotate : {}", angle);

    let d1 = distance(rotation_center, point1);
    let d2 = distance(rotation_center, point2);
    let (x, y) = if d1 > d2 {
        (point1.x, point1.y)
    } else {
        (point2.x, point2.y)
    };

    let x1 = rotation_center.x as f32 - x as f32;
    let y1 = rotation_center.y as f32 - y as f32;

    let x2 = x1 * angle.to_radians().cos() - y1 * angle.to_radians().sin();
    let y2 = x1 * angle.to_radians().sin() + y1 * angle.to_radians().cos();

    (
        Point {
            x: rotation_center.x + x2 as i32,
            y: rotation_center.y + y2 as i32,
        },
        Point {
            x: rotation_center.x,
            y: rotation_center.y,
        },
    )
}

/// Get the two lines from a line and an angle
///
/// # Arguments
///
/// * `point1` - The line first point
/// * `point2` - The line second point
/// * `point`  - A point the new lines will pass by
/// * `angle`  - The angle
///
/// # Returns
///
/// The two lines
pub fn get_lines_from_angles(
    point1: Point,
    point2: Point,
    point: Point,
    angle: f32,
) -> ((Point, Point), (Point, Point)) {
    (
        rotate_line(point1, point2, point, angle),
        rotate_line(point1, point2, point, -angle),
    )
}

/// Compute the two lines that tanget to a circle from a point
///
/// # Arguments
///
/// * `point` - The point
/// * `circle` - The circle
///
/// # Returns
///
/// The two lines
pub fn tangent_lines_to_circle(
    point: Point,
    circle_center: Point,
    circle_radius: f32,
) -> Option<((Point, Point), (Point, Point))> {
    let distance_to_center = distance(point, circle_center);

    if distance_to_center < circle_radius {
        return None;
    }

    let angle = (circle_radius / distance_to_center).asin().to_degrees();

    Some(get_lines_from_angles(
        circle_center,
        Point {
            x: circle_center.x + circle_radius as i32,
            y: circle_center.y,
        },
        point,
        angle,
    ))
}

/// Compute circle center based on three edge points
///
/// # Arguments
///
/// * `p1` - The first point
/// * `p2` - The second point
/// * `p3` - The third point
///
/// # Returns
///
/// The circle center
pub fn circle_center_from_three_points(p1: Point, p2: Point, p3: Point) -> Point {
    let x1 = p1.x as f32;
    let y1 = p1.y as f32;
    let x2 = p2.x as f32;
    let y2 = p2.y as f32;
    let x3 = p3.x as f32;
    let y3 = p3.y as f32;

    let a = x2 - x1;
    let b = y2 - y1;
    let c = x3 - x1;
    let d = y3 - y1;

    let e = a * (x1 + x2) + b * (y1 + y2);
    let f = c * (x1 + x3) + d * (y1 + y3);

    let g = 2.0 * (a * (y3 - y2) - b * (x3 - x2));

    let x = (d * e - b * f) / g;
    let y = (a * f - c * e) / g;

    Point {
        x: x as i32,
        y: y as i32,
    }
}

/// Compute a circle from three points
///
/// # Arguments
///
/// * `p1` - The first point
/// * `p2` - The second point
/// * `p3` - The third point
///
/// # Returns
///
/// The circle
pub fn circle_from_three_points(p1: Point, p2: Point, p3: Point) -> (Point, f32) {
    let center = circle_center_from_three_points(p1, p2, p3);

    (center, distance(center, p1))
}