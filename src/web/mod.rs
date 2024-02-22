mod errors;
pub mod extractors;
pub mod middlewares;
mod routes;
pub mod dto;
pub mod models;
mod util;

use std::{env, time::Duration};

use axum::{extract::FromRef, http::HeaderMap, response::IntoResponse, routing::{get, put}, Json, Router};
use log::info;
use serde_json::{json, Value};
use sqlx::postgres::{PgPoolOptions, Postgres};
use utoipa::{openapi::security::{Http, HttpAuthScheme, SecurityScheme}, Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::web::{dto::cars::put_car_request::{CarSize, GetCarsResponse, PutCarRequest, PutCarResponse}, errors::HttpError, models::cars::Car};


#[derive(Clone, FromRef)]
pub struct AppState {
    pool: sqlx::Pool<Postgres>,
}
impl AppState {
    pub async fn new() -> Result<AppState, anyhow::Error> {
        info!("acquiring pool...");

        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(3))
            .max_connections(1)
            .connect(&env::var("CONN_URI").unwrap())
            .await?;

        Ok(AppState { pool })
    }
}

pub async fn build_app() -> Router {

    #[derive(OpenApi)]
    #[openapi(
        info(description = "Cars endpoints"),
        paths(
            routes::main::root::put_car,
            routes::main::root::get_cars,
        ), 
        modifiers(&SecurityAddon),
        components(
        schemas(
                PutCarRequest,
                PutCarResponse,
                CarSize,
                Car,
                GetCarsResponse
            )
        )
    )]
    struct ApiDoc;
    struct SecurityAddon;
    
    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components: &mut utoipa::openapi::Components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
        }
    }

    async fn build_json_schema() -> impl IntoResponse {

        let str_docs = ApiDoc::openapi().to_pretty_json().unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        (headers, str_docs)
    
    }

    let state = AppState::new().await.unwrap();
    info!("state ok");
    let app = Router::new()
        .route("/", put(routes::main::root::put_car))
        .route("/", get(routes::main::root::get_cars))
        .route("/json-schema", get(build_json_schema))
        .with_state(state.clone());
    app
}
