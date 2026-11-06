# Bukti QA native Rustman — 6 November 2026

**16 screenshot asli**, format JPEG dari tool vision, tanpa perubahan isi gambar. Data dummy saja; tidak ada push remote.
E09 adalah kegagalan sebelum fix; E10–E16 mencatat perbaikan dan pengujian kontrol mata.

[Matriks 12 test case keamanan](test-cases.md) · [Metadata dan SHA256](screenshots.json)

| ID | Screenshot | Klaim yang didukung |
|---|---|---|
| E01 | [E01-git-rejection-toast.jpg](screenshots/E01-git-rejection-toast.jpg) | Git publication blocked toast visible with error icon and credential-free guidance; no commit or push attempted. |
| E02 | [E02-notification-dismissed.jpg](screenshots/E02-notification-dismissed.jpg) | Dismiss removes persistent Git rejection toast while status-bar rejection remains. |
| E03 | [E03-response-cookie-masked.jpg](screenshots/E03-response-cookie-masked.jpg) | Successful response shows sid cookie masked; Authorization field and generated snippet are also masked. |
| E04 | [E04-send-wire-assertions.jpg](screenshots/E04-send-wire-assertions.jpg) | Actual POST completed 200 OK; owned loopback server reports exact dummy bearer, JSON password and Unicode; masking did not replace wire payload. |
| E05 | [E05-response-headers-masked.jpg](screenshots/E05-response-headers-masked.jpg) | Set-Cookie and X-API-Key response values are masked in the real native response header table. |
| E06 | [E06-request-saved.jpg](screenshots/E06-request-saved.jpg) | Native Save reports Saved and clears the modified marker before close/reopen. |
| E07 | [E07-history-before-restart.jpg](screenshots/E07-history-before-restart.jpg) | Synthetic history list and saved echo request visible before normal close; this does not certify replaying every historical snapshot. |
| E08 | [E08-session-after-restart.jpg](screenshots/E08-session-after-restart.jpg) | A different native window restored the saved echo URL, synthetic environment, masked auth and history list after normal close/reopen. No response persistence is claimed. |
| E09 | [E09-example-headers-before-fix.jpg](screenshots/E09-example-headers-before-fix.jpg) | PRE-FIX FAILURE: saved example editor exposes synthetic Set-Cookie and X-API-Key values. All shown values are intentionally fake. Raw request body is the direct editing surface, not an automatic masked preview. |
| E10 | [E10-example-headers-masked-after-reopen.jpg](screenshots/E10-example-headers-masked-after-reopen.jpg) | POST-FIX PASS: saved example reloaded from native encrypted database; Set-Cookie and X-API-Key are masked, original response assertions and body intact. This build has not yet added per-field eye reveal. |
| E11 | [E11-example-eye-default-hidden.jpg](screenshots/E11-example-eye-default-hidden.jpg) | Final eye-control build: sensitive saved-example headers start masked, with small per-field eye icons and stable value columns. |
| E12 | [E12-example-eye-revealed.jpg](screenshots/E12-example-eye-revealed.jpg) | Explicit eye click reveals only synthetic Set-Cookie; X-API-Key remains hidden, Hide value hover tooltip appears, and editor is not marked dirty. |
| E13 | [E13-example-save-hides-values.jpg](screenshots/E13-example-save-hides-values.jpg) | Saving the example re-hides all revealed credential fields; original values remain encrypted and Example saved locally is visible. |
| E14 | [E14-eye-api-isolation.jpg](screenshots/E14-eye-api-isolation.jpg) | Security rerun: only X-API-Key was explicitly revealed; Set-Cookie stayed masked. No unsaved content marker appeared. |
| E15 | [E15-eye-second-click-hides.jpg](screenshots/E15-eye-second-click-hides.jpg) | Security rerun: a second click re-masks the API key and tooltip returns to Show value. |
| E16 | [E16-eye-panel-change-hides.jpg](screenshots/E16-eye-panel-change-hides.jpg) | Security rerun: leaving response Headers for Body and returning clears reveal; cookie and API key are both masked again. |

## Bukti pendamping

- [Build awal](build-and-scope.json), [build masking intermediate](example-mask-intermediate-build.json), [build mata](eye-build.json): hash executable dan kaitan screenshot. Source masih uncommitted.
- [Server lokal](wire-results.jsonl): pemeriksaan payload dummy, tanpa token asli.
- [Storage/cleanup normal-close](normal-close-storage-summary.json), [example native](example-storage-native.json), [setelah reveal/Save](example-storage-after-eye-save.json).
- [Git sebelum fix](git-toast-red.log), [Git setelah fix](git-feedback-green.log): 4 passed.
- [Reveal sebelum implementasi](example-reveal-red.log), [security rerun](example-security-rerun-green.log): 16 passed.
- Timestamp manifest adalah waktu ekspor UTC, bukan timestamp capture tersertifikasi. Hash memastikan file cocok dengan manifest, bukan tanda tangan keaslian pihak ketiga.

## Batas

Bukan sertifikasi production-ready. Full suite sebelumnya masih memiliki satu kegagalan fixture loopback dan kasus audit lain OPEN. Tidak ada uji TLS/MITM, crash cleanup, atau keyboard QA dalam sesi ini. Eye adalah visibility control, bukan lazy DB decryption; raw body editor tetap teks asli. Nilai yang terlihat pada E09/E12/E14 sengaja dummy.
