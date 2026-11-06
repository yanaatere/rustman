# Validasi manual Windows — data dummy

Tujuan: melengkapi unit/integration tests dengan pemeriksaan UI dan lifecycle.
Bukan sertifikasi production-ready. Jangan gunakan token atau database asli.

## Jalankan build lanjutan yang terisolasi

Folder utama D:/Dev/Rust/postman-clone/rustman sudah checkout
codex/security-production-hardening pada a766ace. Perubahan lanjutan belum
committed dan berada di worktree security. Binary berikut dibangun dari
perubahan lanjutan tersebut; tidak mengganti rustman.exe biasa.

Buka PowerShell dan jalankan:

```powershell
$securityWorktree = 'C:/Users/YSHA-PC/.codex/worktrees/security-production-hardening/rustman'
$env:RUSTMAN_DATA_DIR = (Get-Content "$securityWorktree/target/security-runtime-current.json" | ConvertFrom-Json).root
$env:TEMP = Join-Path $env:RUSTMAN_DATA_DIR 'temp'
$env:TMP = $env:TEMP
& 'D:/Dev/Rust/postman-clone/rustman/target/debug/rustman-security-smoke.exe'
```

Tutup jendela test lama terlebih dahulu. Gunakan hanya collection
**SECURITY SMOKE - SYNTHETIC** dan environment **SYNTHETIC ONLY**.
Pengaturan environment di atas hanya berlaku untuk terminal tersebut dan child
process-nya; tutup terminal setelah selesai.

## Checklist

1. **Startup/reopen**: collection, request dan environment dummy tampil. Tutup
   normal lalu buka ulang dengan perintah yang sama; data tetap lengkap, tanpa
   storage error. Catat bila ada dialog error, jangan reset database.
2. **Auth**: buka tab Auth. Token dummy tertutup secara default. Reveal lalu hide
   bekerja. Ganti ke `{{TOKEN}}`; preview snippet tidak menampilkan nilai token.
3. **Header/cookie**: isi Authorization, X-API-Key dan Cookie dengan nilai dummy.
   Periksa input sensitif dan preview; nilai tertutup. Periksa response headers
   dan Cookies ketika endpoint uji milik Anda mengembalikan Set-Cookie dummy.
4. **Send**: arahkan URL ke endpoint lokal milik Anda. Gunakan credential dummy.
   Kirim dengan token literal lalu `{{TOKEN}}`; verifikasi pada server bahwa nilai
   asli yang diterima sama. Respons harus tampil normal, tidak berubah menjadi
   teks masking. Jangan kirim ke API production.
5. **Save/history**: ubah nama/body/header, Save, tutup normal, buka lagi. Nilai
   asli tetap ada setelah reveal; history dapat membuka request yang benar.
6. **Git guard**: gunakan repo lokal dummy TANPA remote. Request dengan token
   literal harus ditolak sebelum snapshot/commit, dengan pesan yang terlihat.
   Ubah seluruh field credential menjadi referensi environment, termasuk body,
   cookie dan field nonaktif; snapshot normal harus tetap bisa dibuat. Jangan Push.
7. **Import**: paste beberapa cURL, termasuk paste berulang tanpa newline akhir.
   Jumlah preview harus sesuai, method/URL/header/body terpetakan benar. Uji input
   Unicode `一`, Thai dan Korea sebagai teks; hasil harus tetap utuh, bukan
   dianggap otomatis setara ASCII atau perintah executable.
8. **Browser preview**: hanya gunakan HTML statis dummy tanpa script/resource
   eksternal. Buka preview, pastikan isi benar. Setelah menutup aplikasi normal,
   file body.html milik test harus dibersihkan. Preview/ekspor sengaja unmasked;
   crash atau file yang dikunci browser masih bisa menyisakan file privat.
9. **Responsiveness**: ulang Send/cancel, pindah tab dan buka/tutup preview. UI
   tetap responsif; jangan lakukan OOM, stress tak terbatas atau pengujian RCE.

Catat tiap kegagalan: langkah, hasil harapan, hasil aktual, versi/build, dan
screenshot yang sudah direduksi datanya. Jangan lampirkan token/key/database asli.

## Bukti otomatis saat ini

Native app berhasil startup dan responsif. Migrasi fixture dengan native Windows
keystore menghasilkan prefix AES256GCM pada request/environment/session, dan
marker token/password dummy tidak ditemukan dalam DB/WAL/SHM. Ini belum
membuktikan seluruh checklist di atas. Kontrol visual otomatis gagal inisialisasi.
Focused suite terbaru masih 12 passed/1 failed pada transport loopback walaupun
fixture no_proxy; hasil ini masih harus diselidiki. Jangan klaim semua tes lulus.
