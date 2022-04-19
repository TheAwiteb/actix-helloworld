use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NameQuery {
    pub name: Option<String>,
}
