document.addEventListener('DOMContentLoaded', () => {
    const captureBtn = document.getElementById('captureBtn');
    const status = document.getElementById('status');
    const capturedImageDiv = document.getElementById('captured-image');
    const streamImg = document.getElementById('stream');

    let isCapturing = false;

    streamImg.onerror = () => {
        console.error('Stream connection lost');
        status.textContent = 'Stream connection lost. Retrying...';
        status.className = 'error';
        
        setTimeout(() => {
            streamImg.src = '/stream?' + Date.now();
        }, 2000);
    };

    captureBtn.addEventListener('click', async () => {
        if (isCapturing) return;
        
        isCapturing = true;
        captureBtn.disabled = true;
        status.textContent = 'Capturing...';
        status.className = '';

        try {
            const response = await fetch('/capture');
            
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const blob = await response.blob();
            const url = URL.createObjectURL(blob);
            
            const timestamp = new Date().toLocaleString('ja-JP');
            capturedImageDiv.innerHTML = `
                <h3>Captured at ${timestamp}</h3>
                <img src="${url}" alt="Captured photo">
                <div style="margin-top: 10px;">
                    <a href="${url}" download="capture_${Date.now()}.jpg" 
                       style="display: inline-block; padding: 8px 16px; 
                              background: #4caf50; color: white; 
                              text-decoration: none; border-radius: 5px;">
                        💾 Download
                    </a>
                </div>
            `;
            
            status.textContent = 'Photo captured successfully!';
            status.className = 'success';
        } catch (error) {
            console.error('Capture failed:', error);
            status.textContent = `Capture failed: ${error.message}`;
            status.className = 'error';
        } finally {
            isCapturing = false;
            captureBtn.disabled = false;
            
            setTimeout(() => {
                status.textContent = '';
                status.className = '';
            }, 3000);
        }
    });

    console.log('Camera interface initialized');
});