use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use crate::backend_error::{BackendError, handle_error};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub entities: Vec<Entity>,
    pub grading: Grading,
    pub format: Format,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entity {
    pub label: String,
    pub text: String,
    #[serde(rename = "hasBoundingBox")]
    pub has_bounding_box: bool,
    #[serde(rename = "boundingBox")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "manuallyChanged")]
    pub manually_changed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoundingBox {
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
    pub left: i16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Grading {
    pub contrast: f64,
    pub brightness: f64,
    pub sharpness: f64,
    #[serde(rename = "whiteBalance")]
    pub white_balance: (f64, f64, f64),
    #[serde(rename = "manuallyChanged")]
    pub manually_changed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Format {
    pub crop: Crop,
    pub rotation: f64,
    #[serde(rename = "manuallyChanged")]
    pub manually_changed: bool,
}

type Crop = BoundingBox;

impl Metadata {
    
    pub fn default() -> Metadata {
        Metadata {
            entities: Vec::new(),
            grading: Grading {
                contrast: 1.0,
                brightness: 1.0,
                sharpness: 1.0,
                white_balance: (1.0, 1.0, 1.0),
                manually_changed: false,
            },
            format: Format {
                crop: BoundingBox {
                    top: 0,
                    right: 0,
                    bottom: 0,
                    left: 0,
                },
                rotation: 0.0,
                manually_changed: false,
            },
        }
    }
    
    pub fn from_json(value: Value) -> Result<Metadata, BackendError> {
        serde_json::from_value(value).map_err(|err| handle_error(err))
    }
}
