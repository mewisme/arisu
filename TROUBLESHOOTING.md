# Troubleshooting Guide

## App không khởi động / không hiển thị gì

### 1. Windows Defender / Antivirus chặn

**Triệu chứng:** Double-click Arisu.exe nhưng không có gì xảy ra

**Giải pháp:**
1. Right-click vào `Arisu.exe` → Properties
2. Tab "General" → Check "Unblock" (nếu có)
3. Click Apply → OK
4. Thử chạy lại

**Hoặc:**
1. Mở Windows Security → Virus & threat protection
2. Protection history → Tìm Arisu.exe
3. Actions → Allow

### 2. Thiếu quyền thực thi

**Triệu chứng:** App crash ngay khi mở

**Giải pháp:**
1. Right-click Arisu.exe → "Run as administrator"
2. Nếu hoạt động → app cần quyền cao hơn
3. Check folder permissions

### 3. Config file lỗi

**Triệu chứng:** App chạy một lần rồi không chạy nữa

**Giải pháp:**
1. Tìm file `arisu.config.json` (cùng folder với exe)
2. Xóa file này
3. Chạy lại app → config mới sẽ được tạo

### 4. Frame files bị lỗi (khi dùng custom frames)

**Triệu chứng:** App không load được custom frames

**Giải pháp:**
1. Check folder `assets/frames/` có đúng format không:
   - File: `frame_0001.png`, `frame_0002.png`, etc.
   - Format: PNG
   - Naming: Đúng số chữ số theo config
2. Check config `frame_digits` khớp với tên file
3. Xóa folder `assets` để dùng builtin frames

## Debug chi tiết

### Dùng Debug build

1. Download file `Arisu-debug.exe` từ GitHub Release
2. Chạy `Arisu-debug.exe` (nó sẽ hiển thị console window)
3. Hoặc chạy từ Command Prompt để xem đầy đủ logs:
   ```cmd
   cd path\to\arisu
   Arisu-debug.exe
   ```
4. Xem error messages và logs trong console window

**Khác biệt giữa Arisu.exe và Arisu-debug.exe:**
- `Arisu.exe`: Release build, không có console, chạy ẩn
- `Arisu-debug.exe`: Debug build, có console window, hiển thị logs và errors

### Check system requirements

**Minimum:**
- Windows 10/11 (64-bit)
- DirectX 11 compatible GPU
- 100MB RAM
- 100MB disk space

**Recommended:**
- Windows 11
- Dedicated GPU
- 200MB RAM

## Các lỗi phổ biến

### "VCRUNTIME140.dll not found"

**Giải pháp:**
- Download Microsoft Visual C++ Redistributable
- Link: https://aka.ms/vs/17/release/vc_redist.x64.exe

### "d3d11.dll not found"

**Giải pháp:**
- Update DirectX
- Windows Update
- Cài đặt DirectX Runtime

### App bị lag/giật

**Giải pháp:**
1. Giảm FPS trong config: `"fps": 5` → `"fps": 2`
2. Giảm số frames (nếu dùng custom)
3. Giảm kích thước window: `"frame_width": 64.0`

### Window không hiển thị đúng vị trí

**Giải pháp:**
1. App tự động snap vào taskbar
2. Kéo thả để di chuyển
3. Nếu vẫn lỗi: Restart app

## Báo lỗi

Nếu vẫn không giải quyết được:

1. Tạo issue tại: https://github.com/mewisme/arisu/issues
2. Cung cấp thông tin:
   - Windows version
   - Error messages (nếu có)
   - Config file content
   - Steps to reproduce

## FAQ

**Q: Tại sao app không có giao diện?**
A: App là desktop pet, không có window controls. Chỉ hiển thị animation.

**Q: Làm sao tắt app?**
A: Task Manager → Processes → Arisu.exe → End task

**Q: App có virus không?**
A: Không. Source code public, build on GitHub Actions. Windows Defender có thể false positive.

**Q: Tại sao file exe lớn (~100MB)?**
A: 620 animation frames được embed vào binary.

**Q: Có thể chạy nhiều instance?**
A: Có, chạy nhiều lần Arisu.exe.

