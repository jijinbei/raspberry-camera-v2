mod camera;
mod config;
mod web;

use std::sync::Arc;
use tokio::sync::Mutex;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = config::Config::from_env();
    
    info!("Starting Raspberry Pi Camera Server");
    info!("Camera: {}x{} @ {}fps", 
          config.camera.width, 
          config.camera.height, 
          config.camera.framerate);
    info!("Server: {}:{}", config.server.host, config.server.port);

    let camera_manager = Arc::new(Mutex::new(
        camera::CameraManager::new(
            config.camera.width,
            config.camera.height,
            config.camera.framerate,
        )
    ));

    web::start_server(
        &config.server.host,
        config.server.port,
        camera_manager,
    ).await
}
