-- A small fleet used to populate the Vehicle / ScanSession /
-- DetectedFault model lists for demo purposes. Real deployments
-- replace this with their own intake feed.

INSERT INTO vehicles (vin, make_name, model_name, model_year, engine_displacement_cc, fuel_type, created_at) VALUES
  ('YV1RZ8FJ7N2123456', 'Volvo',   'XC60',    2022, 1969, 'Petrol',   NOW()),
  ('VF1RFA00766123456', 'Renault', 'Megane',  2020, 1332, 'Petrol',   NOW()),
  ('1HGCV1F30NA123456', 'Honda',   'Accord',  2022, 1498, 'Hybrid',   NOW())
ON CONFLICT (vin) DO NOTHING;

INSERT INTO scan_sessions (session_reference, vehicle_vin, mileage_km, performed_by_technician, session_notes, created_at) VALUES
  ('SCAN-2026-0001', 'YV1RZ8FJ7N2123456', 38420, 'A. Karlsson', 'Customer reported intermittent check-engine light during cold starts.', NOW()),
  ('SCAN-2026-0002', '1HGCV1F30NA123456', 12150, 'M. Yilmaz',   'Routine 12-month service inspection — no symptoms reported.',           NOW())
ON CONFLICT (session_reference) DO NOTHING;

INSERT INTO detected_faults (fault_reference, session_reference, diagnostic_code, is_resolved, resolution_summary, created_at) VALUES
  ('FLT-2026-00001', 'SCAN-2026-0001', 'P0171', false, '',                                                     NOW()),
  ('FLT-2026-00002', 'SCAN-2026-0001', 'P0455', true,  'Customer fuel cap was loose. Tightened; code cleared.', NOW()),
  ('FLT-2026-00003', 'SCAN-2026-0002', 'P0420', false, '',                                                     NOW())
ON CONFLICT (fault_reference) DO NOTHING;
