use std::sync::Arc;

use async_trait::async_trait;
use my_http_server::{
    middlewares::controllers::{
        actions::{DeleteAction, GetAction, PostAction},
        documentation::{
            HttpActionDescription, HttpInputParameter, HttpParameterInputSource, HttpParameterType,
        },
    },
    HttpContext, HttpFailResult, HttpOkResult, WebContentType,
};

use crate::app::AppContext;

pub struct DictionaryController {
    app: Arc<AppContext>,
}

impl DictionaryController {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait]
impl PostAction for DictionaryController {
    async fn handle_request(&self, ctx: HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        let query = ctx.get_query_string()?;

        let key = query.get_required_string_parameter("key")?;
        let value = query.get_required_string_parameter("value")?;

        crate::flows::key_values::insert(self.app.as_ref(), key, value).await?;

        Ok(HttpOkResult::Empty)
    }

    fn get_description(&self) -> Option<HttpActionDescription> {
        let result = HttpActionDescription {
            name: "Dictionary",
            description: "Add key/value to the dictionary",
            out_content_type: WebContentType::Text,
            input_params: vec![
                HttpInputParameter {
                    name: "key".to_string(),
                    param_type: HttpParameterType::String,
                    description: "Key of the dictionary".to_string(),
                    source: HttpParameterInputSource::Query,
                    required: true,
                },
                HttpInputParameter {
                    name: "value".to_string(),
                    param_type: HttpParameterType::String,
                    description: "Value of the dictionary".to_string(),
                    source: HttpParameterInputSource::Query,
                    required: true,
                },
            ]
            .into(),
        };

        Some(result)
    }
}

#[async_trait]
impl DeleteAction for DictionaryController {
    async fn handle_request(&self, ctx: HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        let query = ctx.get_query_string()?;

        let key = query.get_required_string_parameter("key")?;

        crate::flows::key_values::remove(self.app.as_ref(), key).await?;

        Ok(HttpOkResult::Empty)
    }

    fn get_description(&self) -> Option<HttpActionDescription> {
        let result = HttpActionDescription {
            name: "Dictionary",
            description: "Remove key/value from dictionary",
            out_content_type: WebContentType::Text,
            input_params: vec![HttpInputParameter {
                name: "key".to_string(),
                param_type: HttpParameterType::String,
                description: "Key of the dictionary".to_string(),
                source: HttpParameterInputSource::Query,
                required: true,
            }]
            .into(),
        };

        Some(result)
    }
}

#[async_trait]
impl GetAction for DictionaryController {
    async fn handle_request(&self, ctx: HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        let query = ctx.get_query_string()?;

        let key = query.get_required_string_parameter("key")?;

        let result = crate::flows::key_values::get(self.app.as_ref(), key).await?;

        Ok(HttpOkResult::Text { text: result })
    }

    fn get_description(&self) -> Option<HttpActionDescription> {
        let result = HttpActionDescription {
            name: "Dictionary",
            description: "Get value from dictionary",
            out_content_type: WebContentType::Text,
            input_params: vec![HttpInputParameter {
                name: "key".to_string(),
                param_type: HttpParameterType::String,
                description: "Key of the dictionary".to_string(),
                source: HttpParameterInputSource::Query,
                required: true,
            }]
            .into(),
        };

        Some(result)
    }
}
