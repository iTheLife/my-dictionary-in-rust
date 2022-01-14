use crate::app::AppContext;

use super::errors::FlowsError;

pub async fn insert(app: &AppContext, key: &str, value: &str) -> Result<(), FlowsError> {
    app.dictionary
        .insert(key.to_string(), value.to_string())
        .await?;

    Ok(())
}

pub async fn get(app: &AppContext, key: &str) -> Result<String, FlowsError> {
    let result = app.dictionary.get(key.to_string()).await?;

    Ok(result.to_string())
}

pub async fn remove(app: &AppContext, key: &str) -> Result<(), FlowsError> {
    app.dictionary.remove(key.to_string()).await?;
    Ok(())
}
