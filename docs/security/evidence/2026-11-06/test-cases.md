# Test case keamanan: temporary secret reveal

6 November 2026. Data dummy; tidak ada push/remote. Native GUI menggunakan
keystore Windows dan DB sementara. Unit tests memakai key sementara.
Kelompok example: **16 passed / 0 failed**, mencakup tes fungsi dan keamanan,
bukan16 celah keamanan.

| Case | Properti | Hasil/evidence |
|---|---|---|
| EYE-001 | Default hidden saat membuka example | PASS native E11; E09 kegagalan sebelum fix, E10 fix setelah reopen |
| EYE-002 | Reveal hanya field yang diklik | PASS native E12 cookie/E14 API key; unit menguji Request/Response/Params |
| EYE-003 | Klik kedua menutup nilai | PASS native E15 dan unit toggle dua kali |
| EYE-004 | Save menghapus reveal | PASS native E13 dan unit lifecycle |
| EYE-005 | Pindah panel menghapus reveal | PASS native E16 dan unit lifecycle |
| EYE-006 | Close/open editor default hidden | PASS unit; tidak diklaim sebagai crash lifecycle test |
| EYE-007 | Reveal tidak menjadwalkan request | PASS unit: Task units0, HTTP client tidak diinisialisasi; bukan packet capture seluruh OS |
| EYE-008 | Reveal tidak menulis ulang DB | PASS unit: ciphertext sebelum/sesudah identik |
| EYE-009 | Template/parent tidak berubah | PASS unit: {{TOKEN}} tetap template, JSON parent identik; visibility tidak dirty |
| EYE-010 | Row bergeser/ditambah/rename tidak mewariskan reveal | PASS unit; invalid index juga tidak panic/membuka nilai |
| EYE-011 | Nilai asli tetap utuh setelah save/reopen | PASS unit cookie/API key/bearer; encrypted payload AES256GCM |
| EYE-012 | Native example encrypted sesudah reveal/Save | PASS example-storage-after-eye-save.json; marker dummy tidak ditemukan pada DB/WAL/SHM |

[Log hasil lengkap](example-security-rerun-green.log). Test khusus yang ditambahkan:

- security_example_reveal_is_explicit_temporary_and_never_changes_saved_values
- security_example_eye_isolated_no_network_and_no_database_write
- security_example_eye_row_changes_cannot_reveal_a_different_secret
- security_example_secrets_remain_original_and_encrypted_after_reopen

[Regresi sebelum implementasi](example-reveal-red.log) gagal pada eye click.
[E09](screenshots/E09-example-headers-before-fix.jpg) adalah bukti native bug display.

Batas: full payload sudah didekripsi saat load; eye hanya visibility control,
bukan decrypt-on-click. Body raw tetap teks editable; bukan masking universal.
HTTP lokal bukan uji TLS/MITM. Full audit dan satu kegagalan fixture loopback
sebelumnya masih OPEN. Keyboard navigation belum diverifikasi dalam sesi ini.
