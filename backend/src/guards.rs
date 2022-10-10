use chrono::{prelude::*, Duration};
use rocket::http::{Cookie, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::serde_json::from_str;
use rocket::serde::{Deserialize, Serialize};

fn expire() -> i64 {
    Duration::weeks(1).num_milliseconds()
}

fn is_expired(time: i64) -> bool {
    time + expire() < Utc::now().timestamp_millis()
}

pub fn create_from_now() -> i64 {
    Utc::now().timestamp_millis()
}

#[derive(Debug)]
pub enum SessionError {
    MissingOrInvalid,
    Expired,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Admin {
    pub login: i64,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = SessionError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let cookie: Option<Admin> = cookies
            .get_private("session")
            .and_then(|cookie| from_str(cookie.value()).ok());

        match cookie {
            None => Outcome::Failure((Status::Unauthorized, SessionError::MissingOrInvalid)),
            Some(admin) if is_expired(admin.login) => {
                cookies.remove_private(Cookie::named("session"));
                Outcome::Failure((Status::Gone, SessionError::Expired))
            }
            Some(admin) => Outcome::Success(admin),
        }
    }
}
