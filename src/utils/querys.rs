use serde::Deserialize;

#[derive(Deserialize)]
pub struct NameQuery {
    pub name: Option<String>,
}
