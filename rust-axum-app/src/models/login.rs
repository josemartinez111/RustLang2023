// FILE: models/login.rs
// ___________________________________________________________

use serde::Deserialize;
// ___________________________________________________________

#[derive(Debug, Deserialize)]
pub(crate) struct LoginPayload {
  pub(crate) username: String,
  pub(crate) pwd: String
}
// ___________________________________________________________
