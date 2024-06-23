use std::fs;
use crate::backend_error::BackendError;

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path};
use reqwest;
use reqwest::multipart::{Form, Part};
use serde_json::{Value};

#[tauri::command]
pub async fn process_image(
    path: &str,
    name: String,
    endpoint: String,
    directory: &str,
    artefacts: Vec<String>) -> Result<Value, BackendError>
{
    let prediction = upload_image(path, name, endpoint, artefacts).await?;
    save_image_and_prediction(path, &prediction, directory).await?;
    Ok(prediction)
}

async fn upload_image(path: &str, name: String, endpoint: String, artefacts: Vec<String>) -> Result<Value, BackendError> {
    let mut file = File::open(path).map_err(handle_error)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(handle_error)?;

    let artefacts_json = serde_json::to_string(&artefacts).map_err(handle_error)?;
    println!("artefacts json: {}", artefacts_json);

    let form = Form::new()
        .part("image", Part::bytes(buffer).file_name(name))
        .part("artefacts", Part::text(artefacts_json));

    let response = reqwest::Client::new()
        .post(endpoint)
        .multipart(form)
        .send()
        .await.map_err(handle_error)?;

    if response.status().is_success() {
        let json = response.json().await.map_err(handle_error)?;
        Ok(json)
    } else {
        Err(handle_string_error("Connection error"))
    }
}

fn handle_error(err: impl std::error::Error) -> BackendError {
    return BackendError {
        status_code: 101,
        message: err.to_string(),
    }
}

fn handle_string_error(message: &str) -> BackendError {
    return BackendError {
        status_code: 101,
        message: message.to_string(),
    }
}

async fn save_image_and_prediction(image_path: &str, metadata: &Value, output_dir: &str) -> Result<(), BackendError> {
    // let output_dir = "/Users/kl/Kevin/Projects/ASLA/ASLA Editor test dir/projects/CRE/working/";
    let from_path = Path::new(image_path);
    let file_name = from_path.file_name().ok_or(handle_string_error("Could not extract filename from path"))?;
    let to_path = Path::new(output_dir).join(file_name);
    let mut prediction_path = to_path.clone();
    prediction_path.set_extension("json");

    // move image to working directory
    let move_image_result = move_image(from_path, to_path.as_path());
    if !move_image_result {
        Err(BackendError{status_code: 101, message: "Error when moving the image from the upload directory to the working directory.".to_string()})?;
    }
    
    // write metadata to json file
    let write_prediction_result = write_prediction(metadata, prediction_path.as_path());
    if write_prediction_result.is_err() {
        let _ = move_image(to_path.as_path(), from_path); // undo move
        write_prediction_result?;
    }

    Ok(())
}

fn move_image(from_path: &Path, to_path: &Path) -> bool {
    return match fs::rename(from_path, to_path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn write_prediction(prediction: &Value, path: &Path) -> Result<(), BackendError> {
    let json_string = serde_json::to_string(&prediction).map_err(|err| handle_error(err))?;
    let mut file = File::create(path).map_err(|err| handle_error(err))?;
    file.write_all(json_string.as_bytes()).map_err(|err| handle_error(err))?;
    Ok(())
}
