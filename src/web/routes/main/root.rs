use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::web::{dto::{cars::put_car_request::PutCarRequest, user_claims::UserClaims, Claim}, errors::HttpError, extractors::{token::Token, validate_body::ValidatedJson}, models::cars::{Car, Cars}, AppState};

pub async fn get_cars(
    State(s): State<AppState>,
    Token(user): Token<Claim<UserClaims>>
) -> Result<Json<Value>, HttpError> {

    // we don't assume a user has many cars :)
    // no pagination

    let mut conn = s.pool.acquire().await?;
    let cars = Cars::for_user(
        &mut *conn,
        &user.data().user_id
    ).await?;

    Ok(
        Json(
            json!({
                "success": true,
                "cars": cars
            })
        )
    )
}

pub async fn put_car(
    State(s): State<AppState>,
    Token(user): Token<Claim<UserClaims>>,
    ValidatedJson(body): ValidatedJson<PutCarRequest>,
) -> Result<Json<Value>, HttpError> {

    let mut conn = s.pool.acquire().await?;
    let id = Car::add(
        &mut *conn,
        &body.name,
        &body.plate_no,
        &body.tank_size, 
        &user.data(),
        &body.size
    ).await?;

    Ok(
        Json(
            json!({
                "success": true,
                "car_id": id
            })
        )
    )

}