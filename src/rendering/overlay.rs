use slint::{Image, Rgba8Pixel, SharedPixelBuffer};

use crate::math::{
    distance, distance_to_half_line, distance_to_segment, find_line_extreme_coordinates,
    perpendicular_distance, Point,
};
use crate::utils::IdGenerator;
use crate::OverlayDrawable;
use log::debug;

/// trait for circle
pub trait Circle {
    fn center(&self) -> Point;
    fn radius(&self) -> f32;
}

impl Circle for Drawable {
    fn center(&self) -> Point {
        self.point1
    }

    fn radius(&self) -> f32 {
        distance(self.point1, self.point2)
    }
}

/// Represents the type of a drawable object that can be rendered on the map
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DrawableType {
    #[default]
    Point,
    Segment,
    HalfLine,
    Line,
    Circle,
}

/// Represents a color with red, green, and blue components
#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Represents a drawable object that can be rendered on the map
#[derive(Clone, Copy, Debug, Default)]
pub struct Drawable {
    pub id: i32,
    pub object_type: DrawableType,
    pub point1: Point,
    pub point2: Point,
    pub color: Color,
    pub width: f32,
    pub already_drawn: bool,
    pub listview_id: i32,
}

pub struct OverlayRenderer {
    drawables: Vec<Drawable>,
    pub drawable_images: Vec<OverlayDrawable>,
    entity_id_generator: IdGenerator,
    image_height: u32,
    image_width: u32,
    is_overlay_discarded: bool,
    stroke_width: f32,
    color: Color,
}

impl OverlayRenderer {
    pub fn new(image_width: u32, image_height: u32) -> OverlayRenderer {
        OverlayRenderer {
            drawables: Vec::new(),
            drawable_images: Vec::new(),
            entity_id_generator: IdGenerator::new(),
            image_height,
            image_width,
            is_overlay_discarded: true,
            stroke_width: 4.,
            color: Color {
                r: 42,
                g: 0,
                b: 150,
            },
        }
    }

    pub fn reset(&mut self, image_width: u32, image_height: u32) {
        self.image_height = image_height;
        self.image_width = image_width;
        self.drawables = Vec::new();
        self.drawable_images = Vec::new();
        self.entity_id_generator = IdGenerator::new();
        self.is_overlay_discarded = true;
    }

    /// Set the width of the lines to be drawn
    pub fn set_width(&mut self, width: f32) {
        self.stroke_width = width;
    }

    /// Set the color of the lines to be drawn
    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = Color {
            r: red,
            g: green,
            b: blue,
        };
    }

    pub fn discard_overlay(&mut self) {
        self.is_overlay_discarded = true;
    }

    /// Add half line to the list of drawables
    /// # Arguments
    /// * `point1` - The first point of the half line
    /// * `point2` - The second point of the half line
    pub fn add_half_line(&mut self, point1: Point, point2: Point) {
        let (point3, point4) = find_line_extreme_coordinates(
            point1,
            point2,
            0.,
            self.image_width as f32,
            0.,
            self.image_height as f32,
        );
        // TODO : don't rely on distance but rather on the direction of the line compared to point2 position
        let d1 = distance(point2, point3);
        let d2 = distance(point2, point4);
        let point2 = if d1 < d2 { point3 } else { point4 };
        self.add_segment(point1, point2);
        self.drawables.last_mut().unwrap().object_type = DrawableType::HalfLine;
    }

    /// Add line to the list of drawables
    /// # Arguments
    /// * `point1` - The first point of the line
    /// * `point2` - The second point of the line
    pub fn add_line(&mut self, point1: Point, point2: Point) {
        let (point1, point2) = find_line_extreme_coordinates(
            point1,
            point2,
            0.,
            self.image_width as f32,
            0.,
            self.image_height as f32,
        );
        self.add_segment(point1, point2);
        self.drawables.last_mut().unwrap().object_type = DrawableType::Line;
    }

    /// Add segment to the list of drawables
    /// # Arguments
    /// * `point1` - The first point of the segment
    /// * `point2` - The second point of the segment
    pub fn add_segment(&mut self, point1: Point, point2: Point) {
        let size_x = ((point1.x - point2.x).abs() as u32).max(self.stroke_width as u32);
        let size_y = ((point1.y - point2.y).abs() as u32).max(self.stroke_width as u32);
        let corner_x = (point1.x.min(point2.x) - (self.stroke_width / 2.) as i32).max(0);
        let corner_y = (point1.y.min(point2.y) - (self.stroke_width / 2.) as i32).max(0);

        let local_point1 = Point {
            x: point1.x - corner_x + (self.stroke_width / 2.) as i32,
            y: point1.y - corner_y + (self.stroke_width / 2.) as i32,
        };
        let local_point2 = Point {
            x: point2.x - corner_x,
            y: point2.y - corner_y,
        };

        let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(size_x, size_y);
        let mut pixmap =
            tiny_skia::PixmapMut::from_bytes(pixel_buffer.make_mut_bytes(), size_x, size_y)
                .unwrap();

        let mut paint = tiny_skia::Paint::default();
        paint.set_color_rgba8(self.color.r, self.color.g, self.color.b, 255);
        paint.anti_alias = true;

        let stroke = tiny_skia::Stroke {
            width: self.stroke_width,
            ..Default::default()
        };
        let mut pb = tiny_skia::PathBuilder::new();
        pb.move_to(local_point1.x as f32, local_point1.y as f32);
        pb.line_to(local_point2.x as f32, local_point2.y as f32);
        let path = pb.finish().unwrap();
        pixmap.stroke_path(&path, &paint, &stroke, Default::default(), None);

        let id = self.entity_id_generator.get_id();

        let d = Drawable {
            id,
            object_type: DrawableType::Segment,
            point1,
            point2,
            color: self.color,
            width: self.stroke_width,
            ..Default::default()
        };
        self.drawables.push(d);

        self.drawable_images.push(OverlayDrawable {
            id,
            data: Image::from_rgba8_premultiplied(pixel_buffer),
            x: corner_x as f32,
            y: corner_y as f32,
        });
        debug!("Adding segment  {:?}", d);
        debug!("Buffer size : {}x{}", size_x, size_y);
        debug!("x, y : {}, {}", corner_x, corner_y);
    }

    /// Add circle to the list of drawables
    /// # Arguments
    /// * `center` - The center of the circle
    /// * `radius` - The radius of the circle
    pub fn add_circle(&mut self, center: Point, radius: f32) {
        let size = ((radius + self.stroke_width) * 2.) as u32;
        let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(size, size);

        let mut pixmap =
            tiny_skia::PixmapMut::from_bytes(pixel_buffer.make_mut_bytes(), size, size).unwrap();

        let path = tiny_skia::PathBuilder::from_circle(
            radius + self.stroke_width,
            radius + self.stroke_width,
            radius,
        )
        .unwrap();
        let mut paint = tiny_skia::Paint::default();
        paint.set_color_rgba8(self.color.r, self.color.g, self.color.b, 255);
        paint.anti_alias = true;

        let stroke = tiny_skia::Stroke {
            width: self.stroke_width,
            ..Default::default()
        };
        pixmap.stroke_path(&path, &paint, &stroke, Default::default(), None);

        let id = self.entity_id_generator.get_id();

        let d = Drawable {
            id,
            object_type: DrawableType::Circle,
            point1: center,
            point2: Point {
                x: center.x + radius as i32,
                y: center.y,
            },
            color: self.color,
            width: self.stroke_width,
            ..Default::default()
        };
        debug!("Adding circle  {:?}", d);
        debug!("Buffer size : {}", size);
        debug!(
            "x, y : {:.2}, {:.2}",
            center.x as f32 - radius - self.stroke_width,
            center.y as f32 - radius - self.stroke_width
        );
        self.drawables.push(d);

        self.drawable_images.push(OverlayDrawable {
            id,
            data: Image::from_rgba8_premultiplied(pixel_buffer),
            x: center.x as f32 - radius - self.stroke_width,
            y: center.y as f32 - radius - self.stroke_width,
        });
    }

    /// Add point to the list of drawables
    /// # Arguments
    /// * `point` - The point to be added
    pub fn add_point(&mut self, point: Point) {
        let size = (self.stroke_width * 2.) as u32;
        let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(size, size);

        let mut pixmap =
            tiny_skia::PixmapMut::from_bytes(pixel_buffer.make_mut_bytes(), size, size).unwrap();

        let path =
            tiny_skia::PathBuilder::from_circle(point.x as f32, point.y as f32, self.stroke_width)
                .unwrap();
        let mut paint = tiny_skia::Paint::default();
        paint.set_color_rgba8(self.color.r, self.color.g, self.color.b, 255);
        paint.anti_alias = true;

        let stroke = tiny_skia::Stroke {
            width: self.stroke_width,
            ..Default::default()
        };
        pixmap.stroke_path(&path, &paint, &stroke, Default::default(), None);

        let id = self.entity_id_generator.get_id();

        let d = Drawable {
            id,
            object_type: DrawableType::Circle,
            point1: point,
            point2: point,
            color: self.color,
            width: self.stroke_width,
            ..Default::default()
        };
        self.drawables.push(d);

        self.drawable_images.push(OverlayDrawable {
            id,
            data: Image::from_rgba8_premultiplied(pixel_buffer),
            x: point.x as f32 - self.stroke_width,
            y: point.y as f32 - self.stroke_width,
        });
        debug!("Adding point  {:?}", d);
        debug!("Buffer size : {}", size);
        debug!(
            "x, y : {:.2}, {:.2}",
            point.x as f32 - self.stroke_width,
            point.y as f32 - self.stroke_width
        );
    }

    /// Removes a drawable object from the map by its identifier.
    pub fn remove_drawable(&mut self, id: i32) {
        self.drawables.retain(|d| d.id != id);
        self.is_overlay_discarded = true;
    }

    /// Retrieve the list of drawables
    pub fn get_drawables(&self) -> Vec<Drawable> {
        self.drawables.clone()
    }

    /// Set listview id for a drawable
    pub fn set_listview_id(&mut self, id: i32, listview_id: i32) {
        for draw in self.drawables.iter_mut() {
            if draw.id == id {
                draw.listview_id = listview_id;
            }
        }
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
    pub fn closest_circle(&self, point: Point) -> Option<Drawable> {
        let mut min_distance = f32::MAX;
        let mut closest_circle = None;

        for drawable in self.drawables.iter() {
            if drawable.object_type != DrawableType::Circle {
                continue;
            }

            let radius = distance(drawable.point1, drawable.point2);

            let center = drawable.point1;
            let distance = distance(point, center) - radius;

            log::debug!("Id: {}, Distance: {}", drawable.id, distance);

            if distance < min_distance {
                min_distance = distance;
                closest_circle = Some(drawable.clone());
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
    pub fn _closest_point(&self, point: Point) -> Option<Drawable> {
        let mut min_distance = f32::MAX;
        let mut closest_point = None;

        for drawable in self.drawables.iter() {
            if drawable.object_type != DrawableType::Point {
                continue;
            }

            let distance = distance(point, drawable.point1);

            log::debug!("Id: {}, Distance: {}", drawable.id, distance);

            if distance < min_distance {
                min_distance = distance;
                closest_point = Some(drawable.clone());
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
    pub fn closest_object(&self, point: Point) -> Option<Drawable> {
        let mut min_distance = f32::MAX;
        let mut closest_object = None;

        for drawable in self.drawables.iter() {
            let distance = match drawable.object_type {
                DrawableType::Circle => {
                    let radius = distance(drawable.point1, drawable.point2);
                    let center = drawable.point1;
                    distance(point, center) - radius
                }
                DrawableType::Point => distance(point, drawable.point1),
                // TODO : for segment and halfline, we should calculate the distance to the part that is actually drawn
                DrawableType::Line => {
                    perpendicular_distance(point, drawable.point1, drawable.point2)
                }
                DrawableType::Segment => {
                    distance_to_segment(point, drawable.point1, drawable.point2)
                }
                DrawableType::HalfLine => {
                    distance_to_half_line(point, drawable.point1, drawable.point2)
                }
            };

            log::debug!("Id: {}, Distance: {}", drawable.id, distance);

            if distance < min_distance {
                min_distance = distance;
                closest_object = Some(drawable.clone());
            }
        }

        closest_object
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
    pub fn closest_line(&self, point: Point) -> Option<Drawable> {
        let mut min_distance = f32::MAX;
        let mut closest_line = None;

        for drawable in self.drawables.iter() {
            if drawable.object_type != DrawableType::Line
                && drawable.object_type != DrawableType::Segment
                && drawable.object_type != DrawableType::HalfLine
            {
                continue;
            }
            let distance = perpendicular_distance(point, drawable.point1, drawable.point2);

            log::debug!("Id: {}, Distance: {}", drawable.id, distance);

            if distance < min_distance {
                min_distance = distance;
                closest_line = Some(drawable.clone());
            }
        }

        closest_line
    }
}
