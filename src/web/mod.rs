pub mod handlers;
pub mod stream;

use crate::camera::CameraManager;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn start_server(
    host: &str,
    port: u16,
    camera_manager: Arc<Mutex<CameraManager>>,
) -> std::io::Result<()> {
    log::info!("Starting web server on {host}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(camera_manager.clone()))
            .service(handlers::index)
            .service(handlers::capture)
            .service(handlers::stream)
            .service(Files::new("/static", "./static"))
    })
    .bind((host, port))?
    .run()
    .await
}
