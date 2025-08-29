# Raspberry Pi Camera V2 Web Streaming System

RustでRaspberry Pi Camera Module 2を使ったWebストリーミングシステム

## 機能

- 📷 リアルタイムMJPEGストリーミング
- 📸 静止画キャプチャ
- 🌐 Webブラウザからアクセス可能
- ⚡ 低遅延・高パフォーマンス

## 必要要件

### ハードウェア
- Raspberry Pi (3B+, 4, 5推奨)
- Raspberry Pi Camera Module 2
- CSIケーブル

### ソフトウェア
- Raspberry Pi OS (Bullseye以降)
- Rust 1.70以降
- libcamera-apps-lite

## セットアップ

### 1. カメラの有効化

```bash
sudo raspi-config
# Interface Options → Camera → Enable

# または /boot/config.txt に追加
camera_auto_detect=1
```

### 2. 必要なパッケージのインストール

```bash
sudo apt-get update
sudo apt-get install -y libcamera-apps-lite
```

### 3. Rustのインストール（未インストールの場合）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## インストールと実行

### 方法1: プリビルドバイナリを使用（推奨）

1. **バイナリをダウンロード**

   [Releases](https://github.com/your-username/raspberry-camera-v2/releases)から適切なバイナリをダウンロード：
   - `raspberry-camera-v2-raspberry-pi-32bit.tar.gz` - Raspberry Pi 2/3/4 (32-bit OS)
   - `raspberry-camera-v2-raspberry-pi-64bit.tar.gz` - Raspberry Pi 3/4/5 (64-bit OS)

2. **展開と実行**
   ```bash
   # ダウンロードと展開
   tar -xzf raspberry-camera-v2-raspberry-pi-*.tar.gz
   
   # 実行権限を付与して実行
   chmod +x raspberry-camera-v2
   ./raspberry-camera-v2
   ```

### 方法2: ソースからビルド

1. **ビルド**
   ```bash
   cargo build --release
   ```

2. **実行**
   ```bash
   # デフォルト設定で起動
   cargo run --release

   # または
   ./target/release/raspberry-camera-v2
   ```

### 環境変数での設定

```bash
# サーバー設定
export SERVER_HOST=0.0.0.0  # デフォルト: 0.0.0.0
export SERVER_PORT=8080      # デフォルト: 8080

# カメラ設定
export CAMERA_WIDTH=1280     # デフォルト: 640
export CAMERA_HEIGHT=720     # デフォルト: 480
export CAMERA_FPS=30         # デフォルト: 15

cargo run --release
```

## 使用方法

1. プログラムを起動
2. ブラウザで `http://<raspberry-pi-ip>:8080` にアクセス
3. ライブストリーミングが表示される
4. "📷 Capture Photo" ボタンで静止画撮影

## API エンドポイント

- `GET /` - Webインターフェース
- `GET /stream` - MJPEGストリーム
- `GET /capture` - 静止画キャプチャ

## トラブルシューティング

問題が発生した場合は、[TROUBLESHOOTING.md](./TROUBLESHOOTING.md) を参照してください。

### よくある問題:
- `No such file or directory` エラー → `libcamera-*` を `rpicam-*` に変更
- `no cameras available` エラー → 物理接続とカメラ設定を確認
- `Stream ended unexpectedly` → カメラパラメータを調整

## ライセンス

MIT