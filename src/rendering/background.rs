use slint::{Image, Rgba8Pixel, SharedPixelBuffer};

#[derive(Clone, Debug)]
pub struct BackgroundRenderer {
    pub image_height: u32,
    pub image_width: u32,
    bg_pixel_buffer: SharedPixelBuffer<Rgba8Pixel>,
}

impl BackgroundRenderer {
    pub fn new(background_file: &str) -> BackgroundRenderer {
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
        log::debug!("Map pixmap created");

        let mut pixmap = tiny_skia::PixmapMut::from_bytes(
            bg_pixel_buffer.make_mut_bytes(),
            image_width,
            image_height,
        )
        .unwrap();
        pixmap.draw_pixmap(0, 0, map_pixmap.as_ref(), &paint, Default::default(), None);

        BackgroundRenderer {
            image_height,
            image_width,
            bg_pixel_buffer,
        }
    }

    /// Generate the background image
    pub fn render_background(&mut self) -> Option<Image> {
        // if !self.to_be_rendered {
        //     return None;
        // }
        // self.to_be_rendered = false;
        log::debug!("Entering render image");
        let pixel_buffer = self.bg_pixel_buffer.clone();
        log::debug!("Pixel buffer cloned");
        Some(Image::from_rgba8_premultiplied(pixel_buffer))
    }
}
