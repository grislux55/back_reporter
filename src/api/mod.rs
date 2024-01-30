use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Request,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::orm::entities::{prelude::WeChatSession, we_chat_session};

pub mod user_info;
pub mod wechat_login;

pub struct BearerToken {
    pub token: Uuid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(header) if header.starts_with("Bearer ") => match Uuid::parse_str(&header[7..]) {
                Ok(token) => Outcome::Success(BearerToken { token }),
                Err(_) => Outcome::Error((Status::Unauthorized, ())),
            },
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

pub async fn validate_token(
    db: &DatabaseConnection,
    token: &Uuid,
) -> Result<we_chat_session::Model, Status> {
    match WeChatSession::find()
        .filter(we_chat_session::Column::LastToken.eq(*token))
        .one(db)
        .await
    {
        Ok(Some(op))
            if chrono::Local::now().naive_local() - op.last_login > chrono::Duration::hours(6) =>
        {
            Ok(op)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(Status::InternalServerError)
        }
        _ => Err(Status::InternalServerError),
    }
}
