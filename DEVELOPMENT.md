# Development Guide

## Code Quality Tools

Bu proje, kod kalitesini sağlamak için çeşitli araçlar kullanır:

### 1. Code Formatting
```bash
# Tüm kodu formatla
make fmt

# Formatting kontrolü yap (değişiklik yapmadan)
make check-fmt
```

### 2. Linting
```bash
# Clippy ile kod analizi
make clippy
```

### 3. Testing
```bash
# Tüm testleri çalıştır
make test
```

### 4. Pre-commit ve Pre-push Kontrolleri

#### Otomatik Git Hooks
Git hooks otomatik olarak kurulmuştur:

- **Pre-commit**: Her commit öncesi `cargo fmt` çalıştırır
- **Pre-push**: Her push öncesi formatting, clippy ve test kontrolü yapar

#### Manuel Kontroller
```bash
# Commit öncesi tüm kontrolleri çalıştır
make pre-commit

# Push öncesi tüm kontrolleri çalıştır
make pre-push
```

### 5. VS Code/Cursor Ayarları

Proje `.vscode/settings.json` dosyası ile yapılandırılmıştır:
- Dosya kaydedildiğinde otomatik formatting
- Clippy ile otomatik linting
- Import organizasyonu

### 6. GitHub Actions

Her push ve PR'da otomatik olarak:
- Code formatting kontrolü
- Clippy kontrolü
- Test çalıştırma

## Kullanım

### Geliştirme Sırasında
1. Kod yazarken VS Code/Cursor otomatik olarak formatlar
2. Commit yapmadan önce `make pre-commit` çalıştır
3. Push yapmadan önce `make pre-push` çalıştır

### Git Hooks ile Otomatik
```bash
# Hooks zaten kurulu, otomatik çalışır
git add .
git commit -m "Your message"  # Pre-commit hook çalışır
git push                      # Pre-push hook çalışır
```

### Manuel Kontrol
```bash
# Sadece formatting
make fmt

# Sadece clippy
make clippy

# Sadece test
make test

# Tüm kontroller
make pre-commit
```

## Sorun Giderme

### Formatting Sorunları
```bash
# Formatting sorunlarını otomatik düzelt
make fmt

# Formatting kontrolü
make check-fmt
```

### Clippy Uyarıları
```bash
# Clippy uyarılarını kontrol et
make clippy

# Uyarıları düzelt ve tekrar kontrol et
make clippy
```

### Test Hataları
```bash
# Tüm testleri çalıştır
make test

# Belirli bir crate'in testlerini çalıştır
cargo test -p parallel-mengene-core
```

## Yardımcı Komutlar

```bash
# Tüm komutları göster
make help

# Projeyi build et
make build

# Build artifacts'ları temizle
make clean
```
