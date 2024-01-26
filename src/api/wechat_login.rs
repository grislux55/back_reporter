use rocket::{http::Status, post, serde::json::Json, State};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::orm::entities::{app_user, prelude::*, we_chat_session};

const WECHAT_API: &str = "https://api.weixin.qq.com/sns/jscode2session";

#[derive(Deserialize)]
pub struct WeChatLoginRequest {
    pub wechat_code: String,
}

#[derive(Deserialize)]
pub struct WeChatLoginAPIResponse {
    pub session_key: String,
    pub unionid: String,
    pub errmsg: String,
    pub openid: String,
    pub errcode: i32,
}

#[derive(Serialize)]
pub struct WeChatLoginResponse {
    pub token: Uuid,
}

async fn get_token(
    db: &State<DatabaseConnection>,
    resp: &WeChatLoginAPIResponse,
) -> anyhow::Result<Uuid> {
    let db = db as &DatabaseConnection;

    let user = match AppUser::find()
        .filter(app_user::Column::WechatId.eq(&resp.openid))
        .one(db)
        .await?
    {
        Some(user) => user,
        None => {
            let user = app_user::ActiveModel {
                wechat_id: Set(resp.openid.clone()),
                ..Default::default()
            };
            user.insert(db).await?
        }
    };

    let token = match WeChatSession::find()
        .filter(we_chat_session::Column::UserId.eq(user.id))
        .one(db)
        .await?
    {
        Some(token) => {
            let token = we_chat_session::ActiveModel {
                last_session: Set(resp.session_key.clone()),
                ..token.into()
            };
            token.update(db).await?
        }
        None => {
            let token = we_chat_session::ActiveModel {
                user_id: Set(user.id),
                last_session: Set(resp.session_key.clone()),
                ..Default::default()
            };
            token.insert(db).await?
        }
    };

    Ok(token.last_token)
}

#[post("/wechat-login", format = "json", data = "<info>")]
pub async fn wechat_login_service(
    db: &State<DatabaseConnection>,
    info: Json<WeChatLoginRequest>,
) -> Result<Json<WeChatLoginResponse>, Status> {
    let client = reqwest::Client::new();
    let response = client
        .get(WECHAT_API)
        .query(&[
            ("appid", crate::APPID),
            ("secret", crate::SECRET),
            ("js_code", &info.wechat_code),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await;

    match response {
        Ok(res) if res.status().is_success() => match res.json::<WeChatLoginAPIResponse>().await {
            Ok(json_value) if json_value.errcode == 0 => match get_token(db, &json_value).await {
                Ok(token) => Ok(Json(WeChatLoginResponse { token })),
                Err(e) => {
                    dbg!(e);
                    Err(Status::BadGateway)
                }
            },
            Ok(json_value) => match json_value.errcode {
                -1 => Err(Status::ServiceUnavailable),
                40029 => Err(Status::BadRequest),
                40226 => Err(Status::Forbidden),
                45011 => Err(Status::TooManyRequests),
                _ => Err(Status::NotImplemented),
            },
            Err(e) => {
                dbg!(e);
                Err(Status::BadGateway)
            }
        },
        Ok(res) => {
            dbg!(res);
            Err(Status::InternalServerError)
        }
        Err(e) => {
            dbg!(e);
            Err(Status::InternalServerError)
        }
    }
}
