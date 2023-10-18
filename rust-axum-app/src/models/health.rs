// FILE: models/health.rs
// ___________________________________________________________

use serde::Deserialize;
// ___________________________________________________________


#[derive(Debug, Deserialize)]
pub struct HealthParams {
    pub name: Option<String>,
}
// ___________________________________________________________
