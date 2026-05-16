# obddesk

**Operator workstation for OBD-II diagnostics.** A small Rust admin
service for cataloguing Diagnostic Trouble Codes (DTCs), registering
vehicles, recording scan sessions, and tracking detected faults
through to resolution.

Built on [`rustio-admin`](https://github.com/abdulwahed-sweden/rustio-admin)
v0.14.0 — the Builder MVP — so the data model is the source of truth,
the admin surface is generated, and every schema change rides through
an append-only event log.

---

## Domain

Four core models, each with a clear single purpose:

| Model | Purpose | Identifying field |
|---|---|---|
| `DiagnosticCode` | Canonical DTC catalogue (e.g. `P0171`, `B1234`). Every code carries a short title, detailed description, system category (Powertrain / Body / Chassis / Network), code type (Generic / Manufacturer), and severity level (Critical / Warning / Info). | `code` (UNIQUE) |
| `Vehicle` | Registered vehicle: VIN, make, model name, model year, engine displacement (cc), fuel type. | `vin` (UNIQUE) |
| `ScanSession` | One diagnostic event: reference id, the vehicle's VIN, current mileage, the technician who ran the scan, and free-form session notes. | `session_reference` (UNIQUE) |
| `DetectedFault` | A DTC observed within a scan session. Links a session reference to a diagnostic code, tracks resolution status, and stores a resolution summary. | `fault_reference` (UNIQUE) |

Inter-model references are stored as **plain text foreign-key fields**
in this release — explicit relations (`belongs_to`) land in a future
`rustio-admin` release (v0.16+). The chosen keys are stable strings
(`vin`, `session_reference`, `code`, `fault_reference`) so the
soft-FK approach is auditable and convertible to hard FKs without
data migration.

## What the Builder generated

```
src/
├── _generated/
│   ├── admin.rs                     # build_admin() registers all 4 models
│   ├── mod.rs
│   └── models/
│       ├── diagnostic_code.rs       # DiagnosticCode + Model + ModelAdmin impls
│       ├── vehicle.rs               # Vehicle + Model + ModelAdmin impls
│       ├── scan_session.rs          # ScanSession + Model + ModelAdmin impls
│       └── detected_fault.rs        # DetectedFault + Model + ModelAdmin impls
└── main.rs                          # developer-owned entry point
```

```
migrations/
└── 0001_initial.sql                 # CREATE TABLE for each model
```

```
.rustio/
├── draft.toml                       # declarative source of truth
├── history.jsonl                    # append-only build event log
└── builder.lock                     # pinned Builder version (0.14.0)
```

## Getting it running

Prerequisites: Rust ≥ 1.88, PostgreSQL 14+.

```sh
# 1. Bring up Postgres (any 14+ instance is fine)
createdb obddesk_dev

# 2. Set the connection string
echo 'DATABASE_URL=postgres://postgres@localhost/obddesk_dev' > .env

# 3. Apply the migration
cargo run -p rustio-admin-cli -- migrate apply

# 4. Seed the first administrator
cargo run -p rustio-admin-cli -- user create \
    --email admin@obddesk.local --role administrator

# 5. Start the admin
cargo run
```

The admin panel lands at <http://127.0.0.1:8000/admin>. Sign in with
the account from step 4. The four models surface in the left
sidebar.

## Evolving the schema

Schema changes flow through the Builder, not through hand edits.
This release ships with the initial-migration set; future field
additions require Builder support for incremental migrations
(targeted at `rustio-admin` v0.16):

```sh
# v0.16+ workflow (not yet supported)
rustio add field DiagnosticCode common_causes text
rustio add field DiagnosticCode typical_fix text
rustio commit                          # emits 0002_*.sql
```

For now, the schema is fixed at the v0.14.0 MVP snapshot. Project
re-bootstrap is the supported path for new fields.

## Source attribution

This project synthesises four prior experiments by the same author:

- **`obd-simulator`** (Python) — mock OBD-II vehicle data generator,
  no hardware required. Inspires the `Vehicle` model shape.
- **`obd2-diagnostic-tool`** (Tauri + React) — desktop DTC viewer
  with 850+ codes. Defines the `DiagnosticCode` field set adopted
  here.
- **`odb-codes-to-json`** (Python) — DTC catalogue parser
  (4,000+ codes) covering SAE J2012 + ISO 15031, multilingual
  metadata. Establishes the `system_category` / `code_type` /
  `severity_level` taxonomy.
- **`odb-database-codes`** (FastAPI + Postgres) — DTC catalogue
  service with AI-assisted explanations. Validates the
  database-backed admin shape this project adopts in Rust.

`obddesk` is the Rust-native, admin-first re-implementation of the
catalogue surface, with vehicle and scan-session models added so the
DTC knowledge base can be exercised against real operational data.

## Roadmap

Tied to upstream `rustio-admin`:

| Milestone | Capability |
|---|---|
| **v0.1** (this release) | Four models, initial migration, generated admin |
| **v0.2** | Hard FK relations (`Vehicle ←→ ScanSession ←→ DetectedFault ←→ DiagnosticCode`) once Builder supports `belongs_to` |
| **v0.3** | Incremental schema evolution via `rustio commit` (depends on rustio-admin v0.16) |
| **v0.4** | DTC catalogue import from the `odb-codes-to-json` JSON output |
| **v0.5** | Vehicle / session search + per-vehicle fault history view |
| **v0.6** | Read-only API surface for the `obd2-diagnostic-tool` desktop app to consume |

## License

MIT.
