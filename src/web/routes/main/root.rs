use axum::{extract::State, Json};

use crate::web::{dto::{car_claims::CarClaims, cars::put_car_request::{CarSize, GetCarsResponse, PutCarRequest, PutCarResponse}, user_claims::UserClaims, Claim}, errors::HttpError, extractors::{token::Token, validate_body::ValidatedJson}, models::cars::{Car, Cars}, AppState};

#[utoipa::path(
    get,
    path="/",
    responses(
        (status = 200, description = "List of all registered cars for the authenticated user (inside the bearer)", body = GetCarsResponse),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_cars(
    State(s): State<AppState>,
    Token(user): Token<Claim<UserClaims>>
) -> Result<Json<GetCarsResponse>, HttpError> {

    // we assume a user has few cars :)
    // no pagination

    let mut conn = s.pool.acquire().await?;
    let cars = Cars::for_user(
        &mut *conn,
        &user.data().user_id
    ).await?;

    Ok(
        Json(
            GetCarsResponse {
                success: true,
                cars: cars
            })
    )
}

#[utoipa::path(
    put,
    path="/",
    responses(
        (status = 200, description = "Car added successfully", body = PutCarResponse),
        (status = 409, description = "Car already exists")
    ),
    params(
        ("name" = String, Path, description = "Car name"),
        ("size" = CarSize, Path, description = "Car size (small, medium, large)"),
        ("plate_no" = String, Path, description = "Car plate number. Must be 7 chars long"),
        ("tank_size" = i32, Path, description = "Car tank size. Must be a positive integer"),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn put_car(
    State(s): State<AppState>,
    Token(user): Token<Claim<UserClaims>>,
    ValidatedJson(body): ValidatedJson<PutCarRequest>,
) -> Result<Json<PutCarResponse>, HttpError> {

    let mut conn = s.pool.begin().await?;
    let id = Car::add(
        &mut *conn,
        &body.name,
        &body.plate_no,
        &body.tank_size, 
        &user.data(),
        &body.size
    ).await?;

    let consumption = match body.size {
        CarSize::SMALL => 0.10,
        CarSize::MEDIUM => 0.15,
        CarSize::LARGE => 0.20        
    };

    let car_token = Token::<CarClaims>::generate(CarClaims {
        tank_size: body.tank_size,
        car_id: id.clone(),
        consumption,
        owner: user.data().user_id.to_owned()
    }).await?;

    conn.commit().await?;

    Ok(
        Json(
            PutCarResponse {
                success: true,
                car_id: id,
                car_token,
            }
        )
    )

}