use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub camera: CameraConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub width: u32,
    pub height: u32,
    pub framerate: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
            camera: CameraConfig {
                width: 640,
                height: 480,
                framerate: 15,
            },
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut config = Config::default();

        if let Ok(host) = std::env::var("SERVER_HOST") {
            config.server.host = host;
        }

        if let Ok(port) = std::env::var("SERVER_PORT") {
            if let Ok(port) = port.parse() {
                config.server.port = port;
            }
        }

        if let Ok(width) = std::env::var("CAMERA_WIDTH") {
            if let Ok(width) = width.parse() {
                config.camera.width = width;
            }
        }

        if let Ok(height) = std::env::var("CAMERA_HEIGHT") {
            if let Ok(height) = height.parse() {
                config.camera.height = height;
            }
        }

        if let Ok(fps) = std::env::var("CAMERA_FPS") {
            if let Ok(fps) = fps.parse() {
                config.camera.framerate = fps;
            }
        }

        config
    }
}