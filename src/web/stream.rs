use actix_web::{HttpResponse, Result, web::Bytes};
use futures_util::stream::Stream;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use crate::camera::CameraManager;

pub async fn mjpeg_stream(
    camera: Arc<Mutex<CameraManager>>
) -> Result<HttpResponse> {
    let stream = create_mjpeg_stream(camera);
    
    Ok(HttpResponse::Ok()
        .content_type("multipart/x-mixed-replace; boundary=frame")
        .streaming(stream))
}

fn create_mjpeg_stream(
    camera: Arc<Mutex<CameraManager>>
) -> impl Stream<Item = Result<Bytes, actix_web::error::Error>> {
    let interval = interval(Duration::from_millis(66));
    
    futures_util::stream::unfold(
        (camera, interval),
        |(camera, mut interval)| async move {
            interval.tick().await;
            
            let mut cam = camera.lock().await;
            match cam.read_stream_frame().await {
                Ok(jpeg_data) => {
                    let boundary = b"--frame\r\n";
                    let header = format!(
                        "Content-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                        jpeg_data.len()
                    );
                    
                    let mut frame = Vec::new();
                    frame.extend_from_slice(boundary);
                    frame.extend_from_slice(header.as_bytes());
                    frame.extend_from_slice(&jpeg_data);
                    frame.extend_from_slice(b"\r\n");
                    
                    Some((Ok(Bytes::from(frame)), (camera.clone(), interval)))
                }
                Err(e) => {
                    log::error!("Failed to read frame: {}", e);
                    cam.stop_stream().await.ok();
                    None
                }
            }
        }
    )
}