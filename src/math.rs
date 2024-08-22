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
    let angle = det.atan2(dot);
    angle
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
fn perpendicular_distance(p: Point, p1: Point, p2: Point) -> f32 {
    let x0 = p.x as f32;
    let y0 = p.y as f32;
    let x1 = p1.x as f32;
    let y1 = p1.y as f32;
    let x2 = p2.x as f32;
    let y2 = p2.y as f32;

    let num = ((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1).abs();
    let den = ((y2 - y1).powi(2) + (x2 - x1).powi(2)).sqrt();
    num / den
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
            && drawable.object_type != DrawableType::DemiDroite
        {
            continue;
        }
        let p1 = drawable.point1;
        let p2 = drawable.point2;
        let distance = perpendicular_distance(point, p1, p2);

        log::debug!("Id: {}, Distance: {}", drawable.id, distance);

        if distance < min_distance {
            min_distance = distance;
            closest_line = Some(drawable.clone());
        }
    }

    closest_line
}
