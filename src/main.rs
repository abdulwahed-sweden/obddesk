//! obddesk — operator workstation for OBD-II diagnostics.
//!
//! Generator scaffolds this file once; thereafter it is developer-
//! owned. The four models registered via
//! `_generated::admin::build_admin()` are documented in the project
//! README.

mod _generated;

use std::env;
use std::path::PathBuf;

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
    //    project-specific branding. The `AdminTheme` text/border
    //    overrides that this project carried under rustio-admin
    //    0.14 are now redundant — v0.15.0's framework defaults
    //    deliver the same contrast (Principles 9–11). The block
    //    is gone; the visual is identical or slightly improved.
    let admin = _generated::admin::build_admin()
        .app_name("OBD Desk")
        .app_tagline("Operator workstation for OBD-II diagnostics")
        .public_url("http://127.0.0.1:8000");

    // 4. Seed per-model view / add / change / delete permissions.
    admin.seed_permissions(&db).await?;

    // 5. Templates — point at the project's `templates/` directory
    //    so files under `templates/admin/*.html` override the
    //    framework's embedded copies for that page. Files not
    //    overridden continue to load from the embedded set.
    //    The override path can be relocated via
    //    `RUSTIO_TEMPLATE_DIR` for ops who want to deploy
    //    templates alongside the binary without baking them in.
    let template_dir = env::var("RUSTIO_TEMPLATE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("templates"));
    let templates = Templates::new(Some(template_dir))?;

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
