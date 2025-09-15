# Release Workflow

Bu proje, GitHub Actions kullanarak otomatik olarak çoklu platform paketleri oluşturur.

## Desteklenen Platformlar

### Windows
- **Format**: NSIS Installer (.exe)
- **Target**: x86_64-pc-windows-msvc
- **Özellikler**:
  - Otomatik PATH ekleme
  - Start Menu kısayolları
  - Desktop kısayolu
  - .pmz dosya ilişkilendirmesi
  - Program Ekle/Kaldır entegrasyonu

### Linux
- **Format**: Debian Package (.deb)
- **Targets**: 
  - x86_64-unknown-linux-gnu (glibc)
  - x86_64-unknown-linux-musl (musl)
- **Özellikler**:
  - APT ile yüklenebilir
  - Otomatik bağımlılık yönetimi
  - Sistem entegrasyonu

### macOS
- **Format**: App Bundle (.app)
- **Targets**:
  - x86_64-apple-darwin (Intel)
  - aarch64-apple-darwin (Apple Silicon)
- **Özellikler**:
  - Native macOS uygulaması
  - Terminal entegrasyonu
  - Dosya ilişkilendirmesi

## Workflow Tetikleme

### Otomatik Release
```bash
# Yeni bir tag oluştur
git tag v0.1.0
git push origin v0.1.0
```

### Manuel Release
GitHub Actions sekmesinden "Build and Release" workflow'unu manuel olarak çalıştırabilirsiniz.

## Çıktılar

Her platform için aşağıdaki dosyalar oluşturulur:

### Windows
- `parallel-mengene-windows-x86_64-installer.exe` - NSIS installer
- `parallel-mengene.exe` - Standalone executable

### Linux
- `parallel-mengene-linux-x86_64.deb` - Debian package (glibc)
- `parallel-mengene-linux-x86_64-musl.deb` - Debian package (musl)
- `parallel-mengene` - Standalone executable

### macOS
- `parallel-mengene-macos-x86_64.tar.gz` - Intel Mac App Bundle
- `parallel-mengene-macos-aarch64.tar.gz` - Apple Silicon Mac App Bundle
- `parallel-mengene` - Standalone executable

## Yerel Test

### Windows Installer Test
```powershell
# NSIS kurulumu
choco install nsis -y

# Installer oluştur
cargo build --release --target x86_64-pc-windows-msvc
copy scripts\installer.nsi .
copy README.md .
copy docs\USER_GUIDE.md .
copy docs\API_REFERENCE.md .
"C:\Program Files (x86)\NSIS\makensis.exe" installer.nsi
```

### Linux .deb Test
```bash
# cargo-deb kurulumu
cargo install cargo-deb

# .deb paketi oluştur
cargo build --release --target x86_64-unknown-linux-gnu
cargo deb --target x86_64-unknown-linux-gnu --bin parallel-mengene
```

### macOS App Bundle Test
```bash
# App bundle oluştur
cargo build --release --target x86_64-apple-darwin
./scripts/create-macos-app.sh x86_64-apple-darwin macos-x86_64
```

## Konfigürasyon

### Cargo.toml Metadata
CLI crate'inde `[package.metadata.deb]` bölümü .deb paketi için gerekli metadata'yı içerir.

### NSIS Script
`scripts/installer.nsi` Windows installer için konfigürasyonu içerir.

### macOS Script
`scripts/create-macos-app.sh` macOS app bundle oluşturma script'idir.

## Sorun Giderme

### Windows
- NSIS kurulu değilse: `choco install nsis -y`
- PowerShell execution policy: `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser`

### Linux
- cargo-deb kurulu değilse: `cargo install cargo-deb`
- Debian control dosyası eksikse: `debian/control` dosyasını kontrol edin

### macOS
- Script çalıştırma izni: `chmod +x scripts/create-macos-app.sh`
- Xcode command line tools: `xcode-select --install`

## Gelişmiş Özellikler

### Code Signing (macOS)
```bash
# Developer ID ile imzalama
codesign --force --verify --verbose --sign "Developer ID Application: Your Name" "Parallel Mengene.app"
```

### Code Signing (Windows)
```powershell
# Authenticode ile imzalama
signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com parallel-mengene-installer.exe
```

### Notarization (macOS)
```bash
# Apple notarization
xcrun notarytool submit parallel-mengene-macos-x86_64.tar.gz --keychain-profile "notarytool-profile" --wait
```
