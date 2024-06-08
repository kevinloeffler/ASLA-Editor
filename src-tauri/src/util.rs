use crate::backend_error::{BackendError, handle_error};

#[tauri::command]
pub async fn ping_server(url: String) -> Result<bool, BackendError> {
    let response = reqwest::get(url).await.map_err(|err| handle_error(err))?;
    Ok(response.status().is_success())
}
