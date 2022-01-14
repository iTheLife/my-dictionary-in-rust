use my_http_server::{HttpFailResult, WebContentType};

use crate::flows::errors::FlowsError;

impl From<FlowsError> for HttpFailResult {
    fn from(src: FlowsError) -> Self {
        match src {
            FlowsError::KeyExists => HttpFailResult {
                content_type: WebContentType::Text,
                status_code: 401,
                content: "Key already exists".to_string().into_bytes(),
                write_telemetry: true,
            },
            FlowsError::KeyNotFound => HttpFailResult {
                content_type: WebContentType::Text,
                status_code: 401,
                content: "Key not found".to_string().into_bytes(),
                write_telemetry: true,
            },
        }
    }
}
