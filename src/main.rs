mod app;
mod dictionary;
mod flows;
mod http;

use std::{net::SocketAddr, sync::Arc, time::Duration};

use my_http_server::{middlewares::swagger::SwaggerMiddleware, MyHttpServer};

#[tokio::main]
async fn main() {
    let app = crate::app::AppContext::new();
    let app = Arc::new(app);

    let mut http_server: MyHttpServer = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 5000)));

    let controllers = Arc::new(crate::http::controllers_builder::build(app.clone()));

    let swagger = SwaggerMiddleware::new(
        controllers.clone(),
        "Dictionary".to_string(),
        "1.0.0".to_string(),
    );
    http_server.add_middleware(controllers);
    http_server.add_middleware(Arc::new(swagger));

    http_server.start(app);

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
