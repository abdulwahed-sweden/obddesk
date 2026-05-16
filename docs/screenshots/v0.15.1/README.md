# obddesk screenshots — rustio-admin v0.15.1

Captured 2026-05-17 against the live admin running at
`http://127.0.0.1:8000/admin` on `rustio-admin = "0.15.1"` (dark-frame
chrome refinement; see the [release notes](https://github.com/abdulwahed-sweden/rustio-admin/releases/tag/v0.15.1)).

Each PNG is 1440 × 900 px, headless Chrome 148, default zoom.

## What each shot demonstrates

| File | URL | Key visual signals |
|---|---|---|
| [`01-login.png`](01-login.png) | `/admin/login` | Topbar in deep slate `#1F2A37` holds a minimal sign-in form on the neutral cool grey canvas. Email input shows the accent focus ring; "Sign in" button shows the new vertical gradient + inset highlight + accent shadow; footer matches chrome with `[PRODUCTION]` environment badge. |
| [`02-dashboard.png`](02-dashboard.png) | `/admin` | Full chrome (topbar + sidebar + footer). Sidebar section labels (`MODELS`, `AUTH`) render in 11 px tracked-allcaps. Brand mark "OBD Desk" in heavy 800 weight. Dashboard cards float on the cooler canvas with firmer borders + two-layer shadows. |
| [`03-diagnostic-codes.png`](03-diagnostic-codes.png) | `/admin/diagnostic_codes` | 4311 real OBD-II DTCs across 87 pages. Primary `code` column reads in `text-strong` weight 500 — anchors the row for skim reading. Uppercase table headers (`CODE`, `SHORT_TITLE`, `SYSTEM_CATEGORY`, `CODE_TYPE`, `SEVERITY_LEVEL`, `ACTIONS`) in 600 weight + tracked-allcaps. "+ Add DiagnosticCode" primary button shows the new gradient. |
| [`04-vehicles.png`](04-vehicles.png) | `/admin/vehicles` | 3 vehicles with their full VINs rendered in monospaced tabular-num for column alignment. The "Add Vehicle" button's lifted teal-emerald gradient pops cleanly against the lighter content area. |
| [`05-scan-sessions.png`](05-scan-sessions.png) | `/admin/scan_sessions` | `SESSION_REFERENCE` column anchored bold; ISO-8601 timestamps in mono+tabular so dates line up vertically. |
| [`06-detected-faults.png`](06-detected-faults.png) | `/admin/detected_faults` | The "Filters" dropdown — auto-surfaced from the project's `ModelAdmin::list_filter = ["is_resolved"]` override. Zebra striping visible against the deeper canvas. |
| [`07-sidebar-closeup.png`](07-sidebar-closeup.png) | `/admin` (crop) | Close-up of the overridden sidebar. Brand lockup ( OBD-II connector glyph + `OBD Desk` wordmark + `DIAGNOSTICS` subtitle ) on top; the `DIAGNOSTICS` section groups the four obddesk catalogues each with a thematic glyph — 4-rectangle grid for `Diagnostic codes`, car silhouette for `Vehicles`, scope-with-waveform for `Scan sessions`, alert triangle for `Detected faults`. `AUTH` section below holds framework-default `users` / `users-2` / `clock` icons. |

## What changed vs. earlier versions

| Surface | v0.14.x | v0.15.0 | **v0.15.1 (these shots)** |
|---|---|---|---|
| Page canvas | `#EEF1F6` (light blue) | `#E4E8EE` (slate) | `#E5E7EB` neutral cool grey |
| Topbar / sidebar / footer | `#FFFFFF` (same as cards) | `#DCE0E7` (pale grey) | **`#1F2A37` deep slate** |
| Active sidebar wash | accent at 10 % alpha | 12 % alpha | 12 % alpha with **lifted accent** `#3FAA9D` (chrome-scope cascade) |
| Table headers | bold 700 | semibold 600 + tracked | unchanged from 0.15.0 |
| Primary cell | regular weight | weight 500 + text-strong | unchanged from 0.15.0 |
| Input fields | flat 1 px border | + inset shadow | unchanged from 0.15.0 |
| Primary button | flat accent fill | gradient + inset + accent shadow + focus ring | unchanged from 0.15.0 |

## Reproducing these shots

Server prerequisites — see the project README.

```sh
cd ~/Desktop/obddesk-shots && python3 capture.py
```

(That script lives in `/tmp/obddesk-shots/capture.py` in the dev
environment that generated these — it drives Chrome's CDP via
`websockets` to log in and capture each page at 1440 × 900.)
