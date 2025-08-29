use actix_web::{get, web, HttpResponse, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::camera::CameraManager;
use crate::web::stream::mjpeg_stream;

#[get("/")]
pub async fn index() -> Result<HttpResponse> {
    let html = r#"<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Raspberry Pi Camera Stream</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <div class="container">
        <h1>Raspberry Pi Camera</h1>
        <div class="video-container">
            <img id="stream" src="/stream" alt="Camera Stream">
        </div>
        <div class="controls">
            <button id="captureBtn">📷 Capture Photo</button>
            <div id="status"></div>
        </div>
        <div id="captured-image"></div>
    </div>
    <script src="/static/app.js"></script>
</body>
</html>"#;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

#[get("/capture")]
pub async fn capture(
    camera: web::Data<Arc<Mutex<CameraManager>>>
) -> Result<HttpResponse> {
    let mut camera = camera.lock().await;
    
    match camera.capture_photo().await {
        Ok(jpeg_data) => {
            Ok(HttpResponse::Ok()
                .content_type("image/jpeg")
                .body(jpeg_data))
        }
        Err(e) => {
            log::error!("Failed to capture photo: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({
                    "error": format!("Failed to capture photo: {}", e)
                })))
        }
    }
}

#[get("/stream")]
pub async fn stream(
    camera: web::Data<Arc<Mutex<CameraManager>>>
) -> Result<HttpResponse> {
    mjpeg_stream(camera.get_ref().clone()).await
}