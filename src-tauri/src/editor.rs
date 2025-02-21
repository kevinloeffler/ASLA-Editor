use rayon::prelude::*;

use std::io::{Cursor};
use std::sync::{Mutex, MutexGuard};
use std::time::{Instant};
use crate::backend_error::{BackendError, handle_error};
use base64::{engine::general_purpose::STANDARD, Engine};
use image::{ImageFormat, open, Rgb, RgbImage};
use tauri::State;
use crate::config_manager::{convert_jpg_to_json_path, load_image_metadata, update_image_metadata};
use crate::metadata::Metadata;

pub struct Editor {
    pub current_path: String,
    pub image_cache: RgbImage,
    pub original_image: RgbImage,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            current_path: String::new(),
            image_cache: RgbImage::new(0, 0),
            original_image: RgbImage::new(0, 0),
        }
    }
}

impl Editor {
    pub fn load_image(&mut self, path: &str) -> Result<(), BackendError> {
        if path == self.current_path {
            return Ok(());
        }
        self.current_path = path.to_string();
        self.original_image = open(path).map_err(|err| handle_error(err))?.into_rgb8();
        Ok(())
    }

    pub fn encode(&self) -> Result<String, BackendError> {

        let start_time = Instant::now();

        // a vector of size 1'795'540 should fit the image from the archive camera
        let mut buffer = Vec::with_capacity(1_795_540);
        let mut cursor = Cursor::new(&mut buffer);
        if self.image_cache.dimensions().0 == 0 {
            self.original_image.write_to(&mut cursor, ImageFormat::Jpeg).map_err(|err| handle_error(err))?;
        } else {
            self.image_cache.write_to(&mut cursor, ImageFormat::Jpeg).map_err(|err| handle_error(err))?;
        }

        let image_string = STANDARD.encode(& buffer);

        println!("encode timing: {}", (Instant::now() - start_time).as_millis());

        Ok(image_string)
    }
}


#[tauri::command]
pub async fn get_image(path: &str, state_object: State<'_, Mutex<Editor>>) -> Result<(String, Metadata), BackendError> {
    let mut state = state_object.lock().map_err(|err| handle_error(err))?;
    let metadata = load_image_metadata(&*convert_jpg_to_json_path(path))?;

    if path != state.current_path {
        // the user is working on a new image, load that:
        println!("loading image...");
        state.load_image(& path)?;
        let encoded_image: String = render_image(&metadata, state)?;
        return Ok((encoded_image, metadata))
    }

    let base64_string = state.encode()?;
    Ok((base64_string, metadata))
}


#[tauri::command]
pub async fn update_image(metadata: Metadata, state_object: State<'_, Mutex<Editor>>) -> Result<String, BackendError> {
    let start_time = Instant::now();
    
    let state = state_object.lock().map_err(|err| handle_error(err))?;

    let encoded_image = render_image(&metadata, state)?;

    println!("update_image timing: {:?}", Instant::now() - start_time);

    Ok(encoded_image)
}


fn render_image(metadata: &Metadata, mut state: MutexGuard<Editor>) -> Result<String, BackendError> {
    update_image_metadata(&*convert_jpg_to_json_path(&*state.current_path), &metadata)?;
    let absolut_brightness: i64 = (((metadata.grading.brightness - 0.0) / (2.0 - 0.0)) * (128.0 - (-128.0)) + (-128.0)) as i64;

    let start = Instant::now();

    let mut edited_image = state.original_image.clone();

    edited_image
        .rows_mut()
        .par_bridge()
        .for_each(|row| {
            row.for_each(|pixel| {
                *pixel = update_pixel(
                    *pixel,
                    metadata.grading.white_balance,
                    metadata.grading.contrast,
                    absolut_brightness
                )
            })
        });

    println!("image loop timing: {}", (Instant::now() - start).as_millis());

    state.image_cache = edited_image;
    state.encode()
}


fn update_pixel(pixel: Rgb<u8>, wb: (f64, f64, f64), contrast: f64, brightness: i64) -> Rgb<u8> {
    let mut new_pixel = pixel;
    new_pixel[0] = ((contrast * (new_pixel[0] as f64 * wb.0 - 128f64)).round() as i64 + 128 + brightness).clamp(0, 255) as u8;
    new_pixel[1] = ((contrast * (new_pixel[1] as f64 * wb.1 - 128f64)).round() as i64 + 128 + brightness).clamp(0, 255) as u8;
    new_pixel[2] = ((contrast * (new_pixel[2] as f64 * wb.2 - 128f64)).round() as i64 + 128 + brightness).clamp(0, 255) as u8;
    new_pixel
}


#[tauri::command]
pub fn update_metadata(metadata: Metadata, state_object: State<'_, Mutex<Editor>>) -> Result<(), BackendError> {
    let state = state_object.lock().map_err(|err| handle_error(err))?;
    update_image_metadata(&*convert_jpg_to_json_path(&*state.current_path), &metadata)?;
    Ok(())
}

