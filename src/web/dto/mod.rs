use std::{env, time::{SystemTime, UNIX_EPOCH}};

use serde::{Deserialize, Serialize};

pub mod cars;
pub mod user_claims;
pub mod car_claims;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claim<C> 
    where C: Send + Serialize + 'static
{
    exp: usize,
    data: C
}

impl<C> Claim<C>
    where C: Send + Serialize + 'static
{

    pub fn from(item: C, no_exp: bool,) -> Claim<C> {

        let jwt_timeout: Result<u64, _> = { 
            if no_exp { Ok(86400 * 365 * 1000) } // a thousand years
            else { 
                env::var("JWT_TIMEOUT_SECS").unwrap_or("7200".to_string()).parse() // or 2h
            }
        };
        let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + jwt_timeout.ok().unwrap(); // we have the default

        Claim {
            exp: exp as usize, // suck it 32-bit computers...
            data: item
        }

    }

    pub fn exp(&self) -> usize {
        self.exp
    }

    pub fn data(&self) -> &C {
        &self.data
    }

}