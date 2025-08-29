use std::process::Stdio;
use tokio::process::{Command, Child};
use tokio::io::AsyncReadExt;
use bytes::Bytes;
use log::{info, error};


pub struct CameraManager {
    stream_process: Option<Child>,
    width: u32,
    height: u32,
    framerate: u32,
}

impl CameraManager {
    pub fn new(width: u32, height: u32, framerate: u32) -> Self {
        Self {
            stream_process: None,
            width,
            height,
            framerate,
        }
    }

    pub async fn capture_photo(&mut self) -> Result<Vec<u8>, String> {
        info!("Capturing photo: {}x{}", self.width, self.height);
        
        // ストリーミングを一時停止
        let was_streaming = self.stream_process.is_some();
        if was_streaming {
            self.stop_stream().await?;
            // カメラリソースが解放されるまで少し待つ
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        let output = Command::new("rpicam-jpeg")
            .args(&[
                "-o", "-",
                "--width", &self.width.to_string(),
                "--height", &self.height.to_string(),
                "-t", "1",
                "-n",
                "--immediate"
            ])
            .output()
            .await
            .map_err(|e| format!("Failed to execute rpicam-jpeg: {}", e))?;

        let result = if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("rpicam-jpeg failed: {}", stderr);
            Err(format!("Camera capture failed: {}", stderr))
        } else {
            Ok(output.stdout)
        };

        // ストリーミングが動いていた場合は再開
        if was_streaming {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            if let Err(e) = self.start_stream().await {
                error!("Failed to restart stream after capture: {}", e);
            }
        }

        result
    }

    pub async fn start_stream(&mut self) -> Result<(), String> {
        if self.stream_process.is_some() {
            return Ok(());
        }

        info!("Starting MJPEG stream: {}x{} @ {}fps", 
              self.width, self.height, self.framerate);

        let child = Command::new("rpicam-vid")
            .args(&[
                "-t", "0",
                "--codec", "mjpeg",
                "--width", &self.width.to_string(),
                "--height", &self.height.to_string(),
                "--framerate", &self.framerate.to_string(),
                "--inline",
                "-n",
                "-o", "-"
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to start rpicam-vid: {}", e))?;

        self.stream_process = Some(child);
        Ok(())
    }

    pub async fn read_stream_frame(&mut self) -> Result<Bytes, String> {
        if self.stream_process.is_none() {
            self.start_stream().await?;
        }

        let child = self.stream_process.as_mut()
            .ok_or("Stream process not started")?;
        
        let stdout = child.stdout.as_mut()
            .ok_or("Failed to get stdout from stream process")?;

        let mut buffer = vec![0u8; 65536];
        let mut jpeg_data = Vec::new();
        let mut found_start = false;

        loop {
            let n = stdout.read(&mut buffer).await
                .map_err(|e| format!("Failed to read from stream: {}", e))?;
            
            if n == 0 {
                return Err("Stream ended unexpectedly".to_string());
            }

            for i in 0..n {
                if !found_start {
                    if i > 0 && buffer[i-1] == 0xFF && buffer[i] == 0xD8 {
                        found_start = true;
                        jpeg_data.push(0xFF);
                        jpeg_data.push(0xD8);
                    }
                } else {
                    jpeg_data.push(buffer[i]);
                    
                    if jpeg_data.len() > 1 && 
                       jpeg_data[jpeg_data.len()-2] == 0xFF && 
                       jpeg_data[jpeg_data.len()-1] == 0xD9 {
                        return Ok(Bytes::from(jpeg_data));
                    }
                }
            }

            if jpeg_data.len() > 5_000_000 {
                jpeg_data.clear();
                found_start = false;
            }
        }
    }

    pub async fn stop_stream(&mut self) -> Result<(), String> {
        if let Some(mut child) = self.stream_process.take() {
            info!("Stopping camera stream");
            child.kill().await
                .map_err(|e| format!("Failed to stop stream: {}", e))?;
        }
        Ok(())
    }
}

impl Drop for CameraManager {
    fn drop(&mut self) {
        if let Some(mut child) = self.stream_process.take() {
            let _ = child.start_kill();
        }
    }
}