use derive_new::new;
use serde::Serialize;

#[derive(Serialize, new)]
pub struct RouteSchema {
    pub endpoint: String,
    pub description: String,
}

#[derive(Serialize, new)]
pub struct MessageSchema {
    pub message: String,
}

#[derive(Serialize, new)]
pub struct RoutesSchema {
    pub endpoints: Vec<RouteSchema>,
}
