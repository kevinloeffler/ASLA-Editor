use std::ffi::OsString;
use std::fs;
use std::fs::{File, write};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use serde_json::{Value};
use crate::backend_error::{BackendError, handle_error};
use crate::metadata::Metadata;


#[tauri::command]
pub fn load_config_file(path: &str) -> Result<Value, BackendError> {
    let json: Value = read_json_file(path)?;
    
    // validate json
    if json["environment"].is_null() {
        return Err(BackendError {status_code: 101, message: "Missing field 'environment' in config file".to_string()})
    }
    
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
    if json["apiEndpoint"].is_null() {
        return Err(BackendError {status_code: 101, message: "Missing 'apiEndpoint' field in JSON".to_string()});
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



const NO_IMAGE_TEXT: &str = "nil";


#[tauri::command]
pub fn read_current_file(directory: String, reload: bool) -> Result<String, BackendError> {
    let file_path = format!("{}/.current", directory);
    let mut content = String::new();
    
    return if !reload && Path::new(&file_path).exists() {
        let mut file = File::open(&file_path).map_err(|err| handle_error(err))?;
        file.read_to_string(&mut content).map_err(|err| handle_error(err))?;
        Ok(content)
    } else {
        // get oldest image in directory
        let oldest_image: String = get_oldest_image(&directory).unwrap_or(NO_IMAGE_TEXT.to_string());
        update_current_file(directory, oldest_image.clone())?;
        Ok(oldest_image)
    }
}


#[tauri::command]
pub fn update_current_file(directory: String, content: String) -> Result<(), BackendError> {
    let file_path = format!("{}/.current", directory);
    let mut file = File::create(file_path).map_err(|err| handle_error(err))?;
    file.write_all(content.as_bytes()).map_err(|err| handle_error(err))?;
    Ok(())
}


#[tauri::command]
pub fn get_next_image(directory: String, current_image: String, forward: bool) -> Result<String, BackendError> {
    // Read the directory and collect all image files
    let mut images: Vec<PathBuf> = fs::read_dir(directory.clone()).map_err(|err| handle_error(err))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() && is_image(&path) { return Some(path); } 
            None
        })
        .collect();

    images.sort_by(|a, b| {
        a.file_name()
            .unwrap_or(&OsString::new())
            .to_string_lossy()
            .cmp(&b.file_name().unwrap_or(&OsString::new()).to_string_lossy())
    });

    println!("images: {:?}", images);
    
    // Find the current image index
    let current_index = images.iter().position(|path| {
        path.file_name().map_or(false, |name| name.to_string_lossy().to_string() == current_image)
    }).unwrap_or(1);
    
    // Determine the next or previous index
    let new_index = if forward {
        (current_index + 1) % images.len()
    } else {
        if current_index == 0 {
            images.len() - 1
        } else {
            current_index - 1
        }
    };

    // Return the path of the next or previous image
    let new_image;
    match images[new_index].file_name() { 
        Some(val) => new_image = val.to_string_lossy().to_string(),
        None => return Err(BackendError{status_code: 101, message: "error".to_string()})
    };
    
    update_current_file(directory, new_image.clone())?;
    Ok(new_image)
}


pub fn get_oldest_image(dir: &str) -> Option<String> {
    let path = Path::new(dir);
    let mut oldest_file: Option<PathBuf> = None;
    let mut oldest_time: Option<SystemTime> = None;

    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && is_image(&path) {
                        if let Ok(metadata) = fs::metadata(&path) {
                            if let Ok(modified) = metadata.modified() {
                                if oldest_time.is_none() || modified < oldest_time.unwrap() {
                                    oldest_time = Some(modified);
                                    oldest_file = Some(path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    oldest_file.and_then(|path| path.file_name().and_then(|name| name.to_str().map(|s| s.to_string())))
}


fn is_image(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap().to_lowercase().as_str() {
            "jpg" | "jpeg" => true,
            _ => false,
        }
    } else {
        false
    }
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
