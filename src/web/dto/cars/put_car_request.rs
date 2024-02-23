use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::web::models::cars::Car;

#[derive(Deserialize, Serialize, sqlx::Type, ToSchema, Clone)]
#[sqlx(type_name="car_t", rename_all="lowercase")] // this is for sqlx, type_name is the type inside postgres
#[serde(rename_all="lowercase")] // this is for the validator crate, when serving http requests
pub enum CarSize {
    SMALL,
    MEDIUM,
    LARGE
}

#[derive(Validate, Deserialize, Serialize, ToSchema)]
pub struct PutCarRequest {
    pub name: String,
    pub size: CarSize,
    #[validate(length(max=7, min=7))]
    pub plate_no: String,
    #[validate(range(min=0))]
    pub tank_size: i32 // i32 because postgres ints are signed
}

#[derive(Serialize, ToSchema)]
pub struct PutCarResponse {
    pub success: bool,
    pub car_id: String,
    pub car_token: String
}

#[derive(Serialize, ToSchema)]
pub struct GetCarsResponse {
    pub success: bool,
    pub cars: Vec<Car>,
}