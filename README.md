# webcam_art

Menampilkan footage webcam secara real-time sebagai ASCII art di terminal.

## Requirements

- Rust (stable)
- OpenCV 4.x
- Webcam

## Install OpenCV

### Linux (Ubuntu/Debian)
```bash
sudo apt install libopencv-dev clang libclang-dev
```

### Linux (Arch)
```bash
sudo pacman -S opencv clang
```

### macOS

Homebrew tidak lagi menyediakan binary untuk Intel Mac (sejak September 2026). Ada dua opsi:

**Opsi A — Build dari source (Intel & Apple Silicon):**
```bash
git clone https://github.com/opencv/opencv.git --branch 4.10.0 --depth 1
cd opencv && mkdir build && cd build
cmake \
  -D CMAKE_BUILD_TYPE=Release \
  -D CMAKE_INSTALL_PREFIX=/usr/local \
  -D BUILD_LIST=core,imgproc,videoio,highgui \
  -D BUILD_TESTS=OFF \
  -D BUILD_PERF_TESTS=OFF \
  -D BUILD_EXAMPLES=OFF \
  -D WITH_FFMPEG=OFF \
  -D WITH_QT=OFF \
  ..
make -j$(sysctl -n hw.logicalcpu)
make install
```

Kemudian buat file `.cargo/config.toml` di root project:
```toml
[env]
OpenCV_DIR = "/usr/local/lib/cmake/opencv4"
```

Juga perlu symlink libclang agar build script bisa berjalan:
```bash
# Cek path llvm kamu dulu: brew list llvm
ln -sf /usr/local/Cellar/llvm/<versi>/lib/libclang.dylib /usr/local/lib/libclang.dylib
```

**Opsi B — Apple Silicon dengan Homebrew:**
```bash
brew install opencv
```
Tidak perlu konfigurasi tambahan.

### Windows

**Opsi A — Chocolatey (direkomendasikan):**
```powershell
choco install llvm opencv
```

Kemudian set environment variables (sesuaikan path versi OpenCV kamu):
```powershell
$env:OPENCV_LINK_LIBS="opencv_world4100"
$env:OPENCV_LINK_PATHS="C:\tools\opencv\build\x64\vc16\lib"
$env:OPENCV_INCLUDE_PATHS="C:\tools\opencv\build\include"
```

Atau set permanen via System Properties → Environment Variables.

**Opsi B — vcpkg:**
```powershell
vcpkg install opencv4
$env:VCPKGRS_DYNAMIC="1"
```

## Build & Run

```bash
cargo run --release
```

> **Catatan:** Gunakan `--release` untuk performa optimal (frame rate lebih tinggi).

## Tested

| Platform | Status |
|----------|--------|
| Linux    | ✅ |
| macOS (Intel) | ✅ |
| macOS (Apple Silicon) | ✅ (via Homebrew) |
| Windows  | belum dites |

## Controls

Tekan `Ctrl+C` untuk keluar. Terminal akan kembali normal setelah keluar.
