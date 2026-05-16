//! obddesk — operator workstation for OBD-II diagnostics.
//!
//! Generator scaffolds this file once; thereafter it is developer-
//! owned. The four models registered via
//! `_generated::admin::build_admin()` are documented in the project
//! README.

mod _generated;

use std::env;

use rustio_admin::admin::AdminTheme;
use rustio_admin::templates::Templates;
use rustio_admin::{auth, middleware, register_admin_routes, Db, Result, Router, Server};

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();

    // 1. Postgres connection.
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        rustio_admin::Error::Internal("DATABASE_URL must be set (see .env)".to_string())
    })?;
    let db = Db::connect(&database_url).await?;

    // 2. Framework auth + audit tables (idempotent; safe to call on
    //    every boot). Creates rustio_users, rustio_sessions,
    //    permissions, audit log.
    auth::init_tables(&db).await?;

    // 3. Build the admin from generator output, then layer on
    //    project-specific branding and a high-contrast text theme.
    //    The framework's default `--rio-text` / `--rio-text-muted`
    //    are tuned for a ten-hour operator shift; this project
    //    runs them a few stops darker because diagnostic readouts
    //    are skim-read against table backgrounds and small text.
    let admin = _generated::admin::build_admin()
        .app_name("OBD Desk")
        .app_tagline("Operator workstation for OBD-II diagnostics")
        .public_url("http://127.0.0.1:8000")
        .theme(AdminTheme {
            // Darker primary text — moves from the framework's
            // soft slate (~#2D3033) to a near-black graphite.
            text: Some("#0F1115".to_string()),
            // Darker muted text — moves from ~#5A6168 to a
            // deeper steel so secondary labels stay readable.
            text_muted: Some("#3B4148".to_string()),
            // Slightly firmer border for table-row separation.
            border: Some("#C9CFD5".to_string()),
            // Leave accent / bg / surface at framework defaults.
            ..AdminTheme::new()
        });

    // 4. Seed per-model view / add / change / delete permissions.
    admin.seed_permissions(&db).await?;

    // 5. Embedded templates (no disk override).
    let templates = Templates::new(None)?;

    // 6. Canonical middleware chain. Order is load-bearing
    //    (DESIGN_SESSIONS.md / DESIGN_AUDIT.md).
    let router = Router::new()
        .middleware(middleware::logger)
        .middleware(middleware::correlation_id)
        .middleware(middleware::security_headers)
        .middleware(middleware::csrf_protect);

    // 7. Mount admin URLs.
    let router = register_admin_routes(router, admin, db.clone(), templates);

    // 8. Serve.
    let addr = "127.0.0.1:8000".parse().expect("valid listen address");
    println!("rustio: admin running at http://{addr}/admin");
    Server::new(router, addr).run().await?;
    Ok(())
}
