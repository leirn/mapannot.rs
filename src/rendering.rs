use slint::{Image, Rgba8Pixel, SharedPixelBuffer};

use crate::math::{distance, find_line_extreme_coordinates};
use log::{debug, warn};

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

/// Represents a point in 2D space with x and y coordinates
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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

/// Implements a generator for generating unique identifiers for entities
#[derive(Clone, Copy, Debug)]
struct IdGenerator {
    id: i32,
}

/// Represents a layer that can be added to the map
#[derive(Clone, Debug)]
struct Layer {
    _image_height: u32,
    _image_width: u32,
    x: i32,
    y: i32,
    transparency: f32,
    image_data: Vec<u8>,
}

impl IdGenerator {
    /// Creates a new `IdGenerator` with an initial identifier of 0
    fn new() -> IdGenerator {
        IdGenerator { id: 0 }
    }

    /// Generates a new unique identifier
    /// Increments the identifier by 1 and returns the new identifier
    /// # Returns
    /// The new unique identifier
    fn get_id(&mut self) -> i32 {
        self.id += 1;
        log::debug!("New id: {}", self.id);
        self.id
    }
}

/// Implements a renderer for rendering images and drawables.
///
/// The `Renderer` struct provides methods for creating a new renderer, adding drawables, removing drawables,
/// and rendering the background and overlay images.
///
/// # Examples
///
/// ```
/// use mapannot::rendering::{Renderer, Drawable, DrawableType, Point, Color, Image};
///
/// // Create a new renderer
/// let mut renderer = Renderer::new();
///
/// // Add a drawable to the renderer
/// let drawable = Drawable {
///     id: 1,
///     object_type: DrawableType::Circle,
///     point1: Point { x: 100, y: 200 },
///     point2: Point { x: 150, y: 250 },
///     color: Color { r: 255, g: 0, b: 0 },
///     width: 2.0,
/// };
/// renderer.add_drawable(drawable);
///
/// // Render the background image
/// let background_image = renderer.render_background();
///
/// // Render the overlay image
/// let overlay_image = renderer.render_overlay(1.5);
/// ```
///
/// # Safety
///
/// The `Renderer` struct uses unsafe code internally for manipulating pixel buffers and drawing images. It is the responsibility of the caller to ensure that the inputs are valid and that the renderer is used correctly.
#[derive(Clone, Debug)]
pub struct Renderer {
    drawables: Vec<Drawable>,
    entity_id_generator: IdGenerator,
    image_height: u32,
    image_width: u32,
    bg_pixel_buffer: SharedPixelBuffer<Rgba8Pixel>,
    overlay_pixel_buffer: SharedPixelBuffer<Rgba8Pixel>,
    layers: Vec<Layer>,
    discard_overlay: bool,
}

impl Renderer {
    pub fn new(background_file: &str) -> Renderer {
        // open background image from disk and add to pixel buffer
        let image = image::open(background_file).unwrap();
        let image = image.to_rgba8();
        let image_width = image.width();
        let image_height = image.height();
        let mut image_data = image.into_raw();

        let paint = tiny_skia::PixmapPaint {
            opacity: 1.,
            blend_mode: tiny_skia::BlendMode::Source,
            quality: tiny_skia::FilterQuality::Nearest,
        };

        let map_pixmap =
            tiny_skia::PixmapMut::from_bytes(image_data.as_mut(), image_width, image_height)
                .unwrap();

        let mut bg_pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(image_width, image_height);
        let overlay_pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(image_width, image_height);
        log::debug!("Map pixmap created");

        let mut pixmap = tiny_skia::PixmapMut::from_bytes(
            bg_pixel_buffer.make_mut_bytes(),
            image_width,
            image_height,
        )
        .unwrap();
        pixmap.draw_pixmap(0, 0, map_pixmap.as_ref(), &paint, Default::default(), None);

        Renderer {
            drawables: Vec::new(),
            entity_id_generator: IdGenerator::new(),
            image_height,
            image_width,
            bg_pixel_buffer,
            overlay_pixel_buffer,
            layers: vec![],
            discard_overlay: true,
        }
    }

    /// Adds a layer to the map with the specified image file, position, and transparency.
    /// # Arguments
    /// * `file` - The path to the image file
    /// * `x` - The x-coordinate of the layer
    /// * `y` - The y-coordinate of the layer
    /// * `transparency` - The transparency of the layer
    pub fn add_layer(&mut self, file: &str, x: i32, y: i32, transparency: f32) {
        let image = image::open(file).unwrap();
        let image = image.to_rgba8();
        let image_width = image.width();
        let image_height = image.height();
        let image_data = image.into_raw();
        let layer = Layer {
            _image_height: image_height,
            _image_width: image_width,
            x,
            y,
            transparency,
            image_data,
        };
        self.layers.push(layer);
    }

    /// Adds a drawable object to the map.
    pub fn add_drawable(&mut self, mut drawable: Drawable) {
        drawable.id = self.entity_id_generator.get_id();
        drawable.already_drawn = false;
        self.drawables.push(drawable);
    }

    /// Adds a drawable object to the map with the specified values.
    pub fn add_drawable_by_values(
        &mut self,
        object_type: DrawableType,
        point1: Point,
        point2: Point,
        color: Color,
        width: f32,
    ) {
        let d = Drawable {
            id: self.entity_id_generator.get_id(),
            object_type,
            point1,
            point2,
            color,
            width,
            ..Default::default()
        };
        self.drawables.push(d);
    }

    /// Removes a drawable object from the map by its identifier.
    pub fn remove_drawable(&mut self, id: i32) {
        self.drawables.retain(|d| d.id != id);
        self.discard_overlay = true;
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

    /// Generate the background image
    pub fn render_background(&mut self) -> Option<Image> {
        // if !self.to_be_rendered {
        //     return None;
        // }
        // self.to_be_rendered = false;
        log::debug!("Entering render image");
        let mut pixel_buffer = self.bg_pixel_buffer.clone();
        log::debug!("Pixel buffer cloned");
        let mut pixmap = tiny_skia::PixmapMut::from_bytes(
            pixel_buffer.make_mut_bytes(),
            self.image_width,
            self.image_height,
        )
        .unwrap();

        for layer in self.layers.iter_mut() {
            let layer_pixmap = tiny_skia::PixmapMut::from_bytes(
                layer.image_data.as_mut(),
                self.image_width,
                self.image_height,
            );
            if layer_pixmap.is_none() {
                warn!("Layer pixmap is not created");
                continue;
            }
            let paint = tiny_skia::PixmapPaint {
                opacity: layer.transparency,
                blend_mode: tiny_skia::BlendMode::Source,
                quality: tiny_skia::FilterQuality::Bicubic,
            };
            pixmap.draw_pixmap(
                layer.x,
                layer.y,
                layer_pixmap.unwrap().as_ref(),
                &paint,
                Default::default(),
                None,
            );
        }

        log::debug!("Pixmap initialized");
        log::debug!("pixmap ready for drawing");
        Some(Image::from_rgba8_premultiplied(pixel_buffer))
    }

    /// Generate the overlay image
    pub fn render_overlay(&mut self, zoom: f32) -> Option<Image> {
        log::debug!("Entering render overlay");
        if !self.discard_overlay {
            let mut already_rendered = true;
            for draw in self.drawables.iter() {
                already_rendered &= draw.already_drawn;
            }
            if already_rendered {
                log::debug!("Overlay is already rendered");
                return None;
            }
        }

        log::debug!("Get sharepixel buffer");
        let data = self.overlay_pixel_buffer.make_mut_bytes();
        log::debug!("Creating pixmap");

        let mut pixmap =
            tiny_skia::PixmapMut::from_bytes(data, self.image_width, self.image_height).unwrap();

        if self.discard_overlay {
            log::debug!("Discarding overlay");
            for draw in self.drawables.iter_mut() {
                draw.already_drawn = false;
            }
            pixmap.fill(tiny_skia::Color::TRANSPARENT);
        }

        log::debug!("Pixmap initialized");

        // add all drawables to the pixmap
        for draw in self.drawables.iter_mut() {
            if draw.already_drawn {
                continue;
            }
            draw.already_drawn = true;

            log::debug!("Draw {}", draw.id);
            let mut paint = tiny_skia::Paint::default();
            paint.set_color_rgba8(draw.color.r, draw.color.g, draw.color.b, 255);
            paint.anti_alias = true;

            debug!("Width: {}, zoom: {}", draw.width, zoom);
            let stroke = tiny_skia::Stroke {
                width: draw.width * zoom,
                ..Default::default()
            };

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
                        self.image_width as f32,
                        0.,
                        self.image_height as f32,
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
        self.discard_overlay = false;

        log::debug!("End of rendering");
        Some(Image::from_rgba8_premultiplied(
            self.overlay_pixel_buffer.clone(),
        ))
    }
}
