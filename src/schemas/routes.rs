use crate::errors::ErrorResponse;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, new)]
pub struct ParamsSchema {
    /// Parameter name ( required )
    pub name: String,
    /// Parameter description ( required )
    pub description: String,
    /// Parameter optional ( required )
    pub optional: bool,
}

#[derive(Serialize, Deserialize, new)]
pub struct RouteSchema {
    /// Endpoint path ( required )
    pub endpoint: String,
    /// Endpoint method ( required )
    pub method: String,
    /// Endpoint description ( required )
    pub description: String,
    /// Endpoint parameters [`ParamsSchema`] ( optional )
    pub parameters: Option<Vec<ParamsSchema>>,
    /// Endpoint errors [`ErrorResponse`] ( optional )
    pub errors: Option<Vec<ErrorResponse>>,
}

#[derive(Serialize, Deserialize, new)]
pub struct MessageSchema {
    #[new(value = "true")]
    /// Response status ( required )
    pub status: bool,
    /// Message ( required )
    pub message: String,
}

#[derive(Serialize, Deserialize, new)]
pub struct RoutesSchema {
    #[new(value = "true")]
    /// Response status ( required )
    pub status: bool,
    /// Endpoint [`RouteSchema`] ( required )
    pub endpoints: Vec<RouteSchema>,
}
