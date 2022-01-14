use std::sync::Arc;

use my_http_server::middlewares::controllers::ControllersMiddleware;

use crate::app::AppContext;

use super::controllers::dictionary_controller::DictionaryController;

pub fn build(app: Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new();

    let dictionary_controller = Arc::new(DictionaryController::new(app));

    result.register_post_action("/add", dictionary_controller.clone());
    result.register_delete_action("/remove", dictionary_controller.clone());
    result.register_get_action("/get", dictionary_controller);

    result
}
