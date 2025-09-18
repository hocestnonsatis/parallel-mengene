## Parallel Mengene GUI Roadmap (v2.0.0)

### Hedef
CLI’yi korurken, WinRAR benzeri kullanıcı arayüzlü, çok platformlu (Windows/macOS/Linux) bir GUI sürümünü üretime hazır hale getirip v2.0.0 olarak yayınlamak.

### Teknoloji Seçimi
- Backend: mevcut `parallel-mengene-core` (Rust) aynen kullanılacak.
- GUI çerçevesi (öneri sırasi):
  - Tauri (Rust backend + web UI; küçük footprint, kolay paketleme/auto-update)
  - Iced (pure Rust), eframe/egui (Rust) alternatifleri
  - Yalnız Windows gerekiyorsa WinUI/Qt bağları değerlendirilebilir
- UI: React veya Svelte (Tauri ile)

### Milestones

#### Milestone 1: Mimari ve Altyapı
- GUI projesi iskeleti oluştur (Tauri workspace’e ekle)
- Core API’yi GUI’ye uygun servis katmanına ayır (async API, iptal/ileri‑sarma)
- Uzun işlemler için job/queue altyapısı (progress, pause/resume, cancel)
- CI: tag’larda çoklu platform build (GUI artefaktları)

#### Milestone 2: Temel UI Prototipi
- Ana pencere: kaynak/dizin seçimi, hedef, algoritma/level seçenekleri
- İş listesi: ekle/çıkar, sırala, durum (queued/running/paused/done/failed)
- İlerleme çubuğu, hız, kalan süre tahmini, CPU kullanımı göstergeleri
- Hata bildirimi ve log paneli

#### Milestone 3: Özellik Paritesi ve Kullanılabilirlik
- Tüm algoritmalar ve ayarların UI’dan yönetimi
- Sürükle‑bırak, dosya/dizin bağlam menüsü (Windows Explorer “Compress with …”)
- Büyük dosyalar için chunked işlem, disk/memory guard’ları
- Oturum kalıcılığı: son ayarlar, preset profilleri (Hızlı/Küçük/Custom)

#### Milestone 4: Gelişmiş UX
- Sık kullanılan hedefler, preset yönetimi (içe/dışa aktar)
- Batch raporlama, özet istatistikler, HTML/JSON rapor çıktısı
- Çoklu iş paralelliği ve önceliklendirme; throttle (CPU limitleme)

#### Milestone 5: Sistem Entegrasyonları
- Windows: Shell extension/Context Menu, dosya ilişkilendirme (.pmz vb.), Installer (WiX/MSI ya da NSIS), opsiyonel auto‑update
- macOS: .app, notarization, file association, context menu service
- Linux: .deb/.rpm/AppImage, desktop file, mime types
- Çoklu dil (i18n) ve erişilebilirlik (klavye, screen reader)

#### Milestone 6: Güvenilirlik ve Performans
- Soak test, büyük dataset’lerle yük testleri
- Crash/telemetry opsiyonel ve anonim (kapalı varsayılan)
- Profiling ve hot path optimizasyonları (progress granülaritesi, I/O backpressure)

#### Milestone 7: Dokümantasyon ve Destek
- Kullanıcı kılavuzu, “ilk kullanım” turu, troubleshooting
- SSS, sürüm notları, migration guide (CLI → GUI)
- Web sitesi/README güncellemeleri ve ekran görüntüleri

#### Milestone 8: v2.0.0 Yayını
- RC’ler: imzalı installer/DMG/AppImage build’i
- Final: `v2.0.0` tag, CI ile otomatik artefaktlar + GitHub Release
- Dağıtım: Release sayfası, hash/signature, minimum sistem gereksinimleri

### CI/CD Planı
- Tag push’ta:
  - Core + GUI build (Windows/macOS/Linux)
  - İmzalı installer/DMG/AppImage üretimi
  - Artefakt yükleme, release notları, opsiyonel symbol/pdb upload
- Nightly (opsiyonel): Canary build’ler

### Yaklaşık Zamanlama (kabaca)
- M1–M2: 1–2 hafta (iskelet ve prototip)
- M3–M4: 2–3 hafta (özellik paritesi ve UX)
- M5: 1–2 hafta (entegrasyon ve paketleme)
- M6–M7: 1–2 hafta (stabilizasyon ve doküman)
- M8: 1 hafta (RC→GA)
- Toplam: 6–10 hafta (ekip büyüklüğüne göre değişir)

### Açık Sorular / Kararlar
- GUI framework kesinleştirme (Tauri vs Iced/egui)
- Auto‑update stratejisi (örn. tauri‑updater, Squirrel, WinSparkle)
- Telemetry kapsamı ve varsayılan (kapalı önerilir)
- Dosya formatı uzantısı ve metadata (örn. `.pmz`)


