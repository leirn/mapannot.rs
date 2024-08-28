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
        let image_data = slint::Image::load_from_path(std::path::Path::new(background_file)).unwrap();

        BackgroundRenderer {
            image_height: image_data.size().height,
            image_width: image_data.size().height,
            bg_pixel_buffer :image_data.to_rgba8_premultiplied().unwrap(),
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
