use crate::errors::ErrorResponse;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, new)]
pub struct ParamsSchema {
    pub name: String,
    pub description: String,
    pub optional: bool,
}

#[derive(Serialize, Deserialize, new)]
pub struct RouteSchema {
    pub endpoint: String,
    pub method: String,
    pub description: String,
    pub parameters: Option<Vec<ParamsSchema>>,
    pub errors: Option<Vec<ErrorResponse>>,
}

#[derive(Serialize, Deserialize, new)]
pub struct MessageSchema {
    #[new(value = "true")]
    pub status: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, new)]
pub struct RoutesSchema {
    #[new(value = "true")]
    pub status: bool,
    pub endpoints: Vec<RouteSchema>,
}
