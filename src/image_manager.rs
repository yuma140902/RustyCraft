use std::collections::HashMap;
use std::os::raw::c_void;
use std::path::Path;

use image::GenericImageView;

pub struct ImageManager {
    image_map: HashMap<String, u32>,
}

impl ImageManager {
    pub fn new() -> ImageManager {
        let image_manager = ImageManager {
            image_map: HashMap::new(),
        };
        image_manager
    }

    pub fn load_image(&mut self, path: &Path, id: &str, vflip: bool) -> bool {
        if !path.exists() {
            return false;
        }

        let mut image = image::open(path).expect("failed to load image");
        let format = match image {
            image::DynamicImage::ImageLuma8(_) => gl::RED,
            image::DynamicImage::ImageLumaA8(_) => gl::RG,
            image::DynamicImage::ImageRgb8(_) => gl::RGB,
            image::DynamicImage::ImageRgba8(_) => gl::RGBA,
            image::DynamicImage::ImageBgr8(_) => gl::BGR,
            image::DynamicImage::ImageBgra8(_) => gl::BGRA,
            _ => todo!(),
        };
        if vflip {
            image = image.flipv();
        }

        let data = image.as_bytes();

        let mut texture = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        self.image_map.insert(id.to_string(), texture);

        true
    }

    pub fn get_texture_id(&mut self, id: &str) -> u32 {
        *self.image_map.get(id).expect("failed to get texture")
    }
}
