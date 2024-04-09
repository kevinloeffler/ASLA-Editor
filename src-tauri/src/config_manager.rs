use std::fs::{File, write};
use std::io::{Read};
use serde_json::{Value};
use crate::backend_error::{BackendError, handle_error};
use crate::metadata::Metadata;


#[tauri::command]
pub fn load_config_file(path: &str) -> Result<Value, BackendError> {
    let mut file = File::open(path).map_err(|err| handle_error(err))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|err| handle_error(err))?;
    
    let json: Value = serde_json::from_str(&contents).map_err(|err| handle_error(err))?;
    
    // validate json
    // TODO
    
    Ok(json)
}


#[tauri::command]
pub fn load_environment_file(path: &str) -> Result<Value, BackendError> {
    let json: Value = read_json_file(path)?;

    // validate json
    let projects = json["projects"].as_array();
    if projects.is_none() {
        return Err(BackendError {status_code: 101, message: "Missing 'projects' array in env file".to_string()});
    }
    for project in projects.unwrap() {
        if !project.is_object() {
            return Err(BackendError {status_code: 101, message: "Invalid project object in env file".to_string()});
        }
        let required_fields = vec![
            "code",
            "name",
            "workingDirectory",
            "exportDirectory",
            "subfolders",
            "artefacts",
        ];
        for field in &required_fields {
            if project[field].is_null() {
                return Err(BackendError {status_code: 101, message: format!("Missing field '{}' in env file", field)});
            }
        }
        if !project["artefacts"].is_array() {
            return Err(BackendError {status_code: 101, message: "Invalid 'artefacts' array in env file".to_string()});
        }
    }
    if json["uploadDirectory"].is_null() {
        return Err(BackendError {status_code: 101, message: "Missing 'uploadDirectory' field in JSON".to_string()});
    }

    Ok(json)
}


#[tauri::command]
pub fn update_environment_project(path: &str, new_projects: Value) -> Result<(), BackendError> {
    let mut environment = read_json_file(path)?;
    environment["projects"] = new_projects;
    write(path, serde_json::to_string_pretty(&environment)
        .map_err(|err| handle_error(err))?)
        .map_err(|err| handle_error(err))?;
    Ok(())
}


#[tauri::command]
pub fn update_environment_upload(path: &str, new_upload_path: &str) -> Result<(), BackendError> {
    let mut environment = read_json_file(path)?;
    environment["uploadDirectory"] = Value::String(new_upload_path.to_string());
    write(path, serde_json::to_string_pretty(&environment)
        .map_err(|err| handle_error(err))?)
        .map_err(|err| handle_error(err))?;
    Ok(())
}


pub fn load_image_metadata(path: &str) -> Result<Metadata, BackendError> {
    let json = read_json_file(path)?;
    let metadata = Metadata::from_json(json)?;
    Ok(metadata)
}


pub fn update_image_metadata(path: &str, metadata: &Metadata) -> Result<(), BackendError> {
    let json = serde_json::to_string(&metadata).map_err(|err| handle_error(err))?;
    write(path, json).map_err(|err| handle_error(err))?;
    Ok(())
}


pub fn convert_jpg_to_json_path(jpg_path: &str) -> String {
    let mut parts = jpg_path.rsplitn(2, '.');
    parts.next();  // skip the suffix (iterator works in reverse)
    let stem_path = parts.next().unwrap_or("");
    format!("{}.json", stem_path)
}


fn read_json_file(path: &str) -> Result<Value, BackendError> {
    let mut file = File::open(path).map_err(|err| handle_error(err))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|err| handle_error(err))?;

    let json: Value = serde_json::from_str(&contents).map_err(|err| handle_error(err))?;
    Ok(json)
}
