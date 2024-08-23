use image;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use tiny_skia;

use crate::math::{distance, find_line_extreme_coordinates};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawableType {
    Point,
    Segment,
    HalfLine,
    Line,
    Circle,
}
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[derive(Clone, Copy, Debug)]
pub struct Drawable {
    pub id: i32,
    pub object_type: DrawableType,
    pub point1: Point,
    pub point2: Point,
    pub color: Color,
    pub width: f32,
}

pub fn render_image(drawables: &Vec<Drawable>) -> Image {
    log::debug!("Entering render image");
    let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(1000, 1000);
    let width = pixel_buffer.width();
    let height = pixel_buffer.height();
    let mut pixmap =
        tiny_skia::PixmapMut::from_bytes(pixel_buffer.make_mut_bytes(), width, height).unwrap();
    pixmap.fill(tiny_skia::Color::TRANSPARENT);

    // open image from disk and add to pixel buffer
    let image = image::open("data/chouette/500.png").unwrap();
    let image = image.to_rgba8();
    let image_width = image.width();
    let image_height = image.height();
    let mut image_data = image.into_raw();

    let map_pixmap =
        tiny_skia::PixmapMut::from_bytes(image_data.as_mut(), image_width, image_height).unwrap();

    let paint = tiny_skia::PixmapPaint {
        opacity: 1.,
        blend_mode: tiny_skia::BlendMode::Source,
        quality: tiny_skia::FilterQuality::Bicubic,
    };

    pixmap.draw_pixmap(0, 0, map_pixmap.as_ref(), &paint, Default::default(), None);

    // add all drawables to the pixmap
    for draw in drawables {
        log::debug!("Draw");
        let mut paint = tiny_skia::Paint::default();
        paint.set_color_rgba8(draw.color.r, draw.color.g, draw.color.b, 255);
        paint.anti_alias = true;

        let mut stroke = tiny_skia::Stroke::default();
        stroke.width = draw.width;

        match draw.object_type {
            DrawableType::Circle => {
                let radius = distance(draw.point1, draw.point2);
                let path = tiny_skia::PathBuilder::from_circle(
                    draw.point1.x as f32,
                    draw.point1.y as f32,
                    radius,
                )
                .unwrap();

                pixmap.stroke_path(&path, &paint, &stroke, Default::default(), None);
            }
            DrawableType::Point => {
                let path = tiny_skia::PathBuilder::from_circle(
                    draw.point1.x as f32,
                    draw.point1.y as f32,
                    draw.width,
                )
                .unwrap();

                pixmap.fill_path(
                    &path,
                    &paint,
                    tiny_skia::FillRule::Winding,
                    Default::default(),
                    None,
                );
            }
            DrawableType::Line | DrawableType::HalfLine => {
                let (p1, p2) = find_line_extreme_coordinates(
                    draw.point1,
                    draw.point2,
                    0.,
                    width as f32,
                    0.,
                    height as f32,
                );
                let mut pb = tiny_skia::PathBuilder::new();
                if draw.object_type == DrawableType::HalfLine {
                    pb.move_to(draw.point1.x as f32, draw.point1.y as f32);
                } else {
                    pb.move_to(p1.x as f32, p1.y as f32);
                }
                pb.line_to(p2.x as f32, p2.y as f32);
                let path = pb.finish().unwrap();

                pixmap.stroke_path(&path, &paint, &stroke, Default::default(), None);
            }
            DrawableType::Segment => {
                let mut pb = tiny_skia::PathBuilder::new();
                pb.move_to(draw.point1.x as f32, draw.point1.y as f32);
                pb.line_to(draw.point2.x as f32, draw.point2.y as f32);
                let path = pb.finish().unwrap();

                pixmap.stroke_path(&path, &paint, &stroke, Default::default(), None);
            }
        };
    }

    Image::from_rgba8_premultiplied(pixel_buffer)
}
