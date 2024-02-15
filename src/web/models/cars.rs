use serde::{Deserialize, Serialize};
use sqlx::PgExecutor;
use nanoid::nanoid;

use crate::web::dto::{cars::put_car_request::CarSize, user_claims::UserClaims};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Car {
    id: String,
    name: String,
    plate_no: String,
    tank_size: i32, // postgres ints are signed
    size: CarSize,
    owner_id: String,
    image_url: Option<String>
}

impl Car {
    pub async fn add(
        e: impl PgExecutor<'_>,
        name: &str,
        plate_no: &str,
        tank_size: &i32,
        owner: &UserClaims,
        size: &CarSize,
    ) -> Result<String, sqlx_core::Error> {
        
        let id = nanoid!(32);
        sqlx::query("
            insert into cars (
                id,
                name,
                plate_no,
                tank_size,
                size,
                owner_id
                -- owner_data
            ) values ($1, $2, $3, $4, $5, $6)
        ")
        .bind(&id)
        .bind(name)
        .bind(plate_no)
        .bind(tank_size)
        .bind(size)
        .bind(&owner.user_id)
        // .bind(json!(owner))
        .execute(e)
        .await?;

        Ok(id)

    }
}

pub struct Cars;

impl Cars {
    pub async fn for_user(
        e: impl PgExecutor<'_>,
        user_id: &str 
    ) -> Result<Vec<Car>, sqlx_core::Error> {

        let results: Vec<Car> = sqlx::query_as("
            select * from cars where owner_id = $1
        ")
        .bind(&user_id)
        .fetch_all(e)
        .await?;

        Ok(results)

    }
}