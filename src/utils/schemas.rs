use derive_new::new;
use serde::Serialize;

#[derive(Serialize, new)]
pub struct ParamsSchema {
    pub name: String,
    pub description: String,
    pub optional: bool
}

#[derive(Serialize, new)]
pub struct RouteSchema {
    #[new(value = "true")]
    pub status: bool,
    pub endpoint: String,
    pub description: String,
    pub parameters: Option<Vec<ParamsSchema>>,
}

#[derive(Serialize, new)]
pub struct MessageSchema {
    #[new(value = "true")]
    pub status: bool,
    pub message: String,
}

#[derive(Serialize, new)]
pub struct RoutesSchema {
    pub endpoints: Vec<RouteSchema>,
}
