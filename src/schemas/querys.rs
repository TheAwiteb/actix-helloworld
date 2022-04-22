use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
/// Name schema
pub struct NameQuery {
    /// name ( optional )
    pub name: Option<String>,
}
