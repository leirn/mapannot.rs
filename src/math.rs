use log::debug;

use crate::rendering::{Drawable, DrawableType, Point};

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
pub fn angle_between(line1: Drawable, line2: Drawable) -> f32 {
    let angle = angle(line1.point1, line1.point2, line2.point1, line2.point2);
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
pub fn perpendicular_distance(p: Point, line: Drawable) -> f32 {
    let x0 = p.x as f32;
    let y0 = p.y as f32;
    let x1 = line.point1.x as f32;
    let y1 = line.point1.y as f32;
    let x2 = line.point2.x as f32;
    let y2 = line.point2.y as f32;

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
pub fn distance_to_segment(point: Point, segment: Drawable) -> f32 {
    let p1 = segment.point1;
    let p2 = segment.point2;
    let perp_distance = perpendicular_distance(point, segment);

    let distance_to_p1 = distance(point, p1);
    let distance_to_p2 = distance(point, p2);

    let distance = perp_distance.min(distance_to_p1).min(distance_to_p2);

    log::debug!("Id: {}, Distance: {}", segment.id, distance);

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
pub fn distance_to_half_line(point: Point, halfline: Drawable) -> f32 {
    let p1 = halfline.point1;
    let perp_distance = perpendicular_distance(point, halfline);

    let distance_to_p1 = distance(point, p1);
    let distance = perp_distance.min(distance_to_p1);

    log::debug!("Id: {}, Distance: {}", halfline.id, distance);

    distance
}

/// Find the closest line segment to a specific point
///
/// # Arguments
///
/// * `point` - The specific point
/// * `lines` - A vector of tuples representing the line segments
///
/// # Returns
///
/// The closest line segment to the specific point
pub fn closest_line(point: Point, lines: Vec<Drawable>) -> Option<Drawable> {
    let mut min_distance = f32::MAX;
    let mut closest_line = None;

    for drawable in lines {
        if drawable.object_type != DrawableType::Line
            && drawable.object_type != DrawableType::Segment
            && drawable.object_type != DrawableType::HalfLine
        {
            continue;
        }
        let distance = perpendicular_distance(point, drawable);

        log::debug!("Id: {}, Distance: {}", drawable.id, distance);

        if distance < min_distance {
            min_distance = distance;
            closest_line = Some(drawable);
        }
    }

    closest_line
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

/// Find closest circle to a specific point
///
/// # Arguments
///
/// * `point` - The specific point
/// * `circles` - A vector of tuples representing the circles
///
/// # Returns
///
/// The closest circle to the specific point
pub fn closest_circle(point: Point, circles: Vec<Drawable>) -> Option<Drawable> {
    let mut min_distance = f32::MAX;
    let mut closest_circle = None;

    for drawable in circles {
        if drawable.object_type != DrawableType::Circle {
            continue;
        }

        let radius = distance(drawable.point1, drawable.point2);

        let center = drawable.point1;
        let distance = distance(point, center) - radius;

        log::debug!("Id: {}, Distance: {}", drawable.id, distance);

        if distance < min_distance {
            min_distance = distance;
            closest_circle = Some(drawable);
        }
    }

    closest_circle
}

/// Find closest point to a specific point
///
/// # Arguments
///
/// * `point` - The specific point
/// * `points` - A vector of tuples representing the points
///
/// # Returns
///
/// The closest point to the specific point
///
pub fn _closest_point(point: Point, points: Vec<Drawable>) -> Option<Drawable> {
    let mut min_distance = f32::MAX;
    let mut closest_point = None;

    for drawable in points {
        if drawable.object_type != DrawableType::Point {
            continue;
        }

        let distance = distance(point, drawable.point1);

        log::debug!("Id: {}, Distance: {}", drawable.id, distance);

        if distance < min_distance {
            min_distance = distance;
            closest_point = Some(drawable);
        }
    }

    closest_point
}

/// Find closest object to a specific point
///
/// # Arguments
///
/// * `point` - The specific point
/// * `objects` - A vector of tuples representing the objects
///
/// # Returns
///
/// The closest object to the specific point
pub fn closest_object(point: Point, objects: Vec<Drawable>) -> Option<Drawable> {
    let mut min_distance = f32::MAX;
    let mut closest_object = None;

    for drawable in objects {
        let distance = match drawable.object_type {
            DrawableType::Circle => {
                let radius = distance(drawable.point1, drawable.point2);
                let center = drawable.point1;
                distance(point, center) - radius
            }
            DrawableType::Point => distance(point, drawable.point1),
            // TODO : for segment and halfline, we should calculate the distance to the part that is actually drawn
            DrawableType::Line => perpendicular_distance(point, drawable),
            DrawableType::Segment => distance_to_segment(point, drawable),
            DrawableType::HalfLine => distance_to_half_line(point, drawable),
        };

        log::debug!("Id: {}, Distance: {}", drawable.id, distance);

        if distance < min_distance {
            min_distance = distance;
            closest_object = Some(drawable);
        }
    }

    closest_object
}

/// Find the coordinates of a line parallel to a given line and passing through a specific point
///
/// # Arguments
///
/// * `point` - The specific point
/// * `line` - The given line
///
/// # Returns
///
/// The coordinates of the line parallel to the given line and passing through the specific point
pub fn parallel_line(point: Point, line: Drawable) -> Drawable {
    if line.point1.x == line.point2.x {
        return Drawable {
            id: 0,
            object_type: DrawableType::Line,
            point1: Point { x: point.x, y: 0 },
            point2: Point { x: point.x, y: 100 },
            color: line.color,
            width: line.width,
            ..Default::default()
        };
    }

    let slope = (line.point2.y - line.point1.y) as f32 / (line.point2.x - line.point1.x) as f32;
    let intercept = point.y as f32 - slope * point.x as f32;

    let x1 = 0;
    let y1 = (slope * x1 as f32 + intercept) as i32;
    let x2 = 1000;
    let y2 = (slope * x2 as f32 + intercept) as i32;

    Drawable {
        id: 0,
        object_type: DrawableType::Line,
        point1: Point { x: x1, y: y1 },
        point2: Point { x: x2, y: y2 },
        color: line.color,
        width: line.width,
        ..Default::default()
    }
}

/// Find the coordinates of a line perpendicular to a given line and passing through a specific point
///
/// # Arguments
///
/// * `point` - The specific point
/// * `line` - The given line
///
/// # Returns
///
/// The coordinates of the line perpendicular to the given line and passing through the specific point
pub fn perpendicular_line(point: Point, line: Drawable) -> Drawable {
    if line.point1.x == line.point2.x {
        return Drawable {
            id: 0,
            object_type: DrawableType::Line,
            point1: Point { x: point.x, y: 0 },
            point2: Point { x: point.x, y: 100 },
            color: line.color,
            width: line.width,
            ..Default::default()
        };
    }

    let slope = (line.point2.y - line.point1.y) as f32 / (line.point2.x - line.point1.x) as f32;
    let perpendicular_slope = -1. / slope;
    let intercept = point.y as f32 - perpendicular_slope * point.x as f32;

    let x1 = 0;
    let y1 = (perpendicular_slope * x1 as f32 + intercept) as i32;
    let x2 = 1000;
    let y2 = (perpendicular_slope * x2 as f32 + intercept) as i32;

    Drawable {
        id: 0,
        object_type: DrawableType::Line,
        point1: Point { x: x1, y: y1 },
        point2: Point { x: x2, y: y2 },
        width: line.width,
        ..Default::default()
    }
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
pub fn median_line(p1: Point, p2: Point) -> Drawable {
    let x = (p1.x + p2.x) / 2;
    let y = (p1.y + p2.y) / 2;

    Drawable {
        id: 0,
        object_type: DrawableType::Line,
        point1: Point { x, y },
        point2: Point {
            x: x + (p2.y - p1.y),
            y: y + (p1.x - p2.x),
        },
        width: 1.0,
        ..Default::default()
    }
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
pub fn rotate_line(line: Drawable, point: Point, angle: f32) -> Drawable {

    debug!("Angle to rotate : {}", angle);

    let d1 = distance(point, line.point1);
    let d2 = distance(point, line.point2);
    let (x, y) = if d1 > d2 {
        (line.point1.x, line.point1.y)
    }
    else {
        (line.point2.x, line.point2.y)
    };

    let x1 = point.x as f32 - x as f32;
    let y1 = point.y as f32 - y as f32;

    let x2 = x1 * angle.to_radians().cos() - y1 * angle.to_radians().sin();
    let y2 = x1 * angle.to_radians().sin() + y1 * angle.to_radians().cos();

    Drawable {
        id: 0,
        object_type: DrawableType::Line,
        point1: Point {
            x: point.x + x2 as i32,
            y: point.y + y2 as i32,
        },
        point2: Point {
            x: point.x,
            y: point.y,
        },
        color: line.color,
        width: line.width,
        ..Default::default()
    }
}

/// Get the two lines from a line and an angle
///
/// # Arguments
///
/// * `line` - The line
/// * `angle` - The angle
///
/// # Returns
///
/// The two lines
pub fn get_lines_from_angles(line: Drawable, point: Point, angle: f32) -> (Drawable, Drawable) {
    (
        rotate_line(line, point, angle),
        rotate_line(line, point, -angle),
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
pub fn tangent_lines_to_circle(point: Point, circle: Drawable) -> Option<(Drawable, Drawable)> {
    let radius = distance(circle.point1, circle.point2);
    let center = circle.point1;

    let distance_to_center = distance(point, center);

    if distance_to_center < radius  {
        return None;
    }

    let angle = (radius / distance_to_center).asin().to_degrees();

    Some(get_lines_from_angles(
        Drawable {
            object_type: DrawableType::Line,
            point1: center,
            point2: Point {
                x: center.x + radius as i32,
                y: center.y,
            },
            color: circle.color,
            width: circle.width,
            ..Default::default()
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
pub fn circle_from_three_points(p1: Point, p2: Point, p3: Point) -> Drawable {
    let center = circle_center_from_three_points(p1, p2, p3);

    Drawable {
        object_type: DrawableType::Circle,
        point1: center,
        point2: p1,
        width: 1.0,
        ..Default::default()
    }
}
