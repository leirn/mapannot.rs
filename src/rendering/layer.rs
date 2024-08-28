use slint::{Image, Rgba8Pixel, SharedPixelBuffer, SharedString};

use crate::{utils::IdGenerator, LayerDrawable};

#[derive(Clone, Debug)]
pub struct LayerRenderer {
    entity_id_generator: IdGenerator,
    pub layers: Vec<LayerDrawable>,
}

impl LayerDrawable {
    pub fn new(id: i32, file: &str, x: i32, y: i32, transparency: f32, zoom: f32) -> LayerDrawable {
        let path = std::path::Path::new(file);
        let filename = path.file_name().unwrap().to_str().unwrap();
        let data = LayerDrawable::redraw(file, transparency);
        LayerDrawable {
            id,
            x: x as f32,
            y: y as f32,
            zoom,
            data,
            transparency,
            file: SharedString::from(file),
            name: SharedString::from(filename),
        }
    }

    pub fn redraw(file: &str, transparency: f32) -> Image {
        let image = image::open(file).unwrap();
        let image = image.to_rgba8();
        let image_width = image.width();
        let image_height = image.height();
        let mut image_data = image.into_raw();

        let map_pixmap =
            tiny_skia::PixmapMut::from_bytes(image_data.as_mut(), image_width, image_height)
                .unwrap();

        let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(image_width, image_height);

        let mut pixmap = tiny_skia::PixmapMut::from_bytes(
            pixel_buffer.make_mut_bytes(),
            image_width,
            image_height,
        )
        .unwrap();

        let paint = tiny_skia::PixmapPaint {
            opacity: transparency,
            blend_mode: tiny_skia::BlendMode::Source,
            quality: tiny_skia::FilterQuality::Nearest,
        };
        pixmap.draw_pixmap(0, 0, map_pixmap.as_ref(), &paint, Default::default(), None);
        Image::from_rgba8_premultiplied(pixel_buffer)
    }
}

impl LayerRenderer {
    pub fn new() -> LayerRenderer {
        LayerRenderer {
            entity_id_generator: IdGenerator::new(),
            layers: vec![],
        }
    }

    /// Adds a layer to the map with the specified image file, position, and transparency.
    /// # Arguments
    /// * `file` - The path to the image file
    /// * `x` - The x-coordinate of the layer
    /// * `y` - The y-coordinate of the layer
    /// * `transparency` - The transparency of the layer
    pub fn add_layer(&mut self, file: &str, x: i32, y: i32, transparency: f32, zoom: f32) {
        let layer = LayerDrawable::new(
            self.entity_id_generator.get_id(),
            file,
            x,
            y,
            zoom,
            transparency,
        );
        self.layers.push(layer);
    }
}
