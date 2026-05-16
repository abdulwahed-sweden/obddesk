-- Reload the DiagnosticCode catalogue from the deduped 4311-row
-- snapshot of `odb-codes-to-json/output/data.json` (SAE J2012 /
-- ISO 15031). Soft-FK refs in `detected_faults` (e.g. P0171,
-- P0420, P0455) point at codes that exist in this snapshot.
TRUNCATE diagnostic_codes RESTART IDENTITY;
\COPY diagnostic_codes (code, short_title, detailed_description, system_category, code_type, severity_level) FROM 'data/dtc_codes_real.csv' WITH (FORMAT csv, HEADER true);
SELECT COUNT(*) AS dtcs_loaded FROM diagnostic_codes;
