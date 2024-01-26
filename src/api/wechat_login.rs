use rocket::{http::Status, post, serde::json::Json, State};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::orm::entities::{app_user, prelude::*, sea_orm_active_enums::UserRole, we_chat_session};

const WECHAT_API: &str = "https://api.weixin.qq.com/sns/jscode2session";

#[derive(Deserialize)]
pub struct WeChatLoginRequest {
    pub wechat_code: String,
}

#[derive(Deserialize)]
pub struct WeChatLoginAPIResponse {
    pub session_key: Option<String>,
    pub unionid: Option<String>,
    pub errmsg: Option<String>,
    pub openid: Option<String>,
    pub errcode: Option<i32>,
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

    let openid = resp.openid.clone().unwrap();

    let user = match AppUser::find()
        .filter(app_user::Column::WechatId.eq(openid.clone()))
        .one(db)
        .await?
    {
        Some(user) => user,
        None => {
            let user = app_user::ActiveModel {
                wechat_id: Set(openid),
                user_role: Set(UserRole::Normal),
                ..Default::default()
            };
            user.insert(db).await?
        }
    };

    let session_key = resp.session_key.clone().unwrap();

    let token = match WeChatSession::find()
        .filter(we_chat_session::Column::UserId.eq(user.id))
        .one(db)
        .await?
    {
        Some(token) => {
            let token = we_chat_session::ActiveModel {
                last_session: Set(session_key),
                ..token.into()
            };
            token.update(db).await?
        }
        None => {
            let token = we_chat_session::ActiveModel {
                user_id: Set(user.id),
                last_session: Set(session_key),
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

    dbg!(&response);

    match response {
        Ok(res) if res.status().is_success() => {
            let text_res = match res.text().await {
                Ok(text) => {
                    dbg!(&text);
                    text
                }
                Err(e) => {
                    dbg!(e);
                    return Err(Status::BadGateway);
                }
            };
            let json_res = serde_json::from_str::<WeChatLoginAPIResponse>(&text_res);

            match json_res {
                Ok(json_value) if json_value.errcode.is_none() => {
                    match get_token(db, &json_value).await {
                        Ok(token) => Ok(Json(WeChatLoginResponse { token })),
                        Err(e) => {
                            dbg!(e);
                            Err(Status::BadGateway)
                        }
                    }
                }
                Ok(json_value) => match json_value.errcode {
                    Some(-1) => Err(Status::ServiceUnavailable),
                    Some(40029) => Err(Status::BadRequest),
                    Some(40226) => Err(Status::Forbidden),
                    Some(45011) => Err(Status::TooManyRequests),
                    _ => Err(Status::NotImplemented),
                },
                Err(e) => {
                    dbg!(e);
                    Err(Status::BadGateway)
                }
            }
        }
        _ => Err(Status::InternalServerError),
    }
}
