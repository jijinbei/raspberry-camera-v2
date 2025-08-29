# トラブルシューティングガイド

## 🔧 よくある問題と解決方法

### ❌ エラー: `No such file or directory (os error 2)`

**症状:**
```
ERROR: Failed to execute libcamera-jpeg: No such file or directory (os error 2)
ERROR: Failed to start libcamera-vid: No such file or directory (os error 2)
```

**原因:** 
Raspberry Pi OS の新しいバージョンでは `libcamera-*` コマンドが `rpicam-*` コマンドに変更されました。

**解決策:**
ソースコードを以下のように修正してください：

```bash
cd raspberry-camera-v2
sed -i 's/libcamera-jpeg/rpicam-jpeg/g' src/camera/libcamera.rs
sed -i 's/libcamera-vid/rpicam-vid/g' src/camera/libcamera.rs
cargo clean
cargo build
```

---

### ❌ エラー: `*** no cameras available ***`

**症状:**
```
ERROR: *** no cameras available ***
```

**原因:** 
カメラが物理的に認識されていない、または設定に問題があります。

**解決手順:**

#### 1. カメラの検出確認
```bash
rpicam-hello --list-cameras
```

#### 2. 物理接続の確認
- CSIケーブルがRaspberry PiとCamera Moduleにしっかり接続されているか
- ケーブルの向き（金属コンタクトの方向）が正しいか
- コネクタのロック（プラスチッククリップ）が閉まっているか

#### 3. I2C通信の確認
```bash
# I2Cツールのインストール
sudo apt install -y i2c-tools

# カメラとの通信確認
i2cdetect -y 1
```

Camera Module v2の場合、アドレス `0x10` にデバイスが表示されるはずです。

#### 4. 設定ファイルの確認
```bash
cat /boot/firmware/config.txt | grep camera
```

以下の設定が含まれていることを確認：
```
camera_auto_detect=1
```

#### 5. 再起動
```bash
sudo reboot
```

---

### ❌ エラー: `Stream ended unexpectedly`

**症状:**
```
ERROR: Failed to read frame: Stream ended unexpectedly
```

**原因:** 
rpicam-vidプロセスがエラーで終了している可能性があります。

**解決策:**
手動でコマンドをテストしてみてください：

```bash
# MJPEGストリーミングテスト
rpicam-vid -t 5000 --codec mjpeg -o test.mjpeg --width 640 --height 480 --framerate 15
```

エラーが出る場合は、解像度やフレームレートを調整してください。

---

### ❌ エラー: `Pipeline handler in use by another process`

**症状:**
```
ERROR: *** failed to acquire camera /base/soc/i2c0mux/i2c@1/imx219@10 ***
V4L2 v4l2_device.cpp:390 'imx219 10-0010': Unable to set controls: Device or resource busy
Pipeline handler in use by another process
```

**原因:** 
libcameraでは同時に複数のプロセスでカメラにアクセスできません。ストリーミング中に静止画キャプチャを試行するとこのエラーが発生します。

**解決策:**
ソースコードで排他制御を実装済みです：
- 静止画キャプチャ時にストリーミングを一時停止
- キャプチャ完了後にストリーミングを再開

手動でテストする場合：
```bash
# ストリーミングを停止してからキャプチャ
pkill rpicam-vid
rpicam-jpeg -o capture.jpg --nopreview
```

---

### 📋 システム情報の確認方法

#### カメラの基本情報
```bash
# カメラの検出
rpicam-hello --list-cameras

# 簡単なテスト
rpicam-hello -t 2000 --nopreview

# 静止画テスト
rpicam-jpeg -o test.jpg --nopreview
```

#### 従来コマンドとの対応表

| 古いコマンド | 新しいコマンド | 説明 |
|-------------|---------------|------|
| `libcamera-hello` | `rpicam-hello` | カメラのテスト |
| `libcamera-jpeg` | `rpicam-jpeg` | 静止画撮影 |
| `libcamera-still` | `rpicam-still` | 静止画撮影（高機能版） |
| `libcamera-vid` | `rpicam-vid` | 動画撮影 |

#### 環境変数での設定変更
```bash
# 解像度変更
export CAMERA_WIDTH=1280
export CAMERA_HEIGHT=720

# フレームレート変更
export CAMERA_FPS=30

# サーバーポート変更
export SERVER_PORT=9000

cargo run --release
```

---

### 🔍 ログ確認方法

詳細なログを確認するには：

```bash
# 詳細ログで実行
RUST_LOG=debug cargo run
```

---

### 💡 パフォーマンス向上のヒント

#### 1. 解像度の最適化
低い解像度で安定性を確保：
```bash
export CAMERA_WIDTH=640
export CAMERA_HEIGHT=480
export CAMERA_FPS=15
```

#### 2. リリースビルドの使用
```bash
cargo build --release
./target/release/raspberry-camera-v2
```

#### 3. GPU メモリの調整
`/boot/firmware/config.txt` に追加：
```
gpu_mem=128
```

---

### 📞 サポート

問題が解決しない場合は、以下の情報を含めて報告してください：

```bash
# システム情報
cat /etc/os-release
uname -a

# カメラ情報
rpicam-hello --list-cameras
vcgencmd get_camera

# 設定確認
cat /boot/firmware/config.txt | grep -E "camera|gpu"
```