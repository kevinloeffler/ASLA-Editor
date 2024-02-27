// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend_error;
use backend_error::BackendError;

use std::fs::File;
use std::io::Read;
use reqwest;
use reqwest::multipart::{Form, Part};
use serde_json::Value;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![upload_image])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
async fn upload_image(path: &str, name: String) -> Result<Value, BackendError> {
  let endpoint = "http://127.0.0.1:8000/image/";

  let mut file = File::open(path).map_err(|err| BackendError{ message: err.to_string() })?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer).map_err(|err| BackendError{ message: err.to_string() })?;

  let form = Form::new()
      .part("image", Part::bytes(buffer).file_name(name.clone()))
      .text("name", name);

  let response = reqwest::Client::new()
      .post(endpoint)
      .multipart(form)
      .send()
      .await.map_err(|err| BackendError{ message: err.to_string() })?;

  let body: Value = response.json().await.map_err(|err| BackendError{ message: err.to_string() })?;
  return Ok(body)
}
