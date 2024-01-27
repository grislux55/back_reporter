#![allow(clippy::blocks_in_conditions)]

use rocket::{delete, post, put, FromForm};
use rocket::{get, http::Status, serde::json::Json, State};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::orm::entities::prelude::UserInfo as UserInfoDb;
use crate::orm::entities::user_info as user_info_db;
use crate::orm::entities::{app_user, prelude::*};

use super::{validate_token, BearerToken};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Option<Uuid>,
    pub creator: Uuid,
    pub id_no: String,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub image: Option<Uuid>,
}

#[derive(FromForm)]
pub struct UserInfoRequest {
    pub start: u64,
    pub count: u64,
}

#[get("/user-info/query?<query..>")]
pub async fn query_user_info(
    db: &State<DatabaseConnection>,
    token: BearerToken,
    query: UserInfoRequest,
) -> Result<Json<Vec<UserInfo>>, Status> {
    let db = db as &DatabaseConnection;
    let token = token.token;

    let ret = validate_token(db, &token).await?;

    let ret = match AppUser::find()
        .filter(app_user::Column::Id.eq(ret.user_id))
        .one(db)
        .await
    {
        Ok(op) => op,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let user = match ret {
        Some(user) => user,
        None => return Err(Status::Unauthorized),
    };

    let user_infos = match UserInfoDb::find()
        .filter(user_info_db::Column::Creator.eq(user.id))
        .offset(query.start)
        .limit(query.count)
        .all(db)
        .await
    {
        Ok(op) => op
            .into_iter()
            .map(|x| UserInfo {
                id: Some(x.id),
                creator: x.creator,
                id_no: x.id_no,
                name: x.name,
                phone: x.phone,
                address: x.address,
                image: x.image,
            })
            .collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    Ok(Json(user_infos))
}

#[post("/user-info/add", data = "<user_info>")]
pub async fn add_user_info(
    db: &State<DatabaseConnection>,
    token: BearerToken,
    user_info: Json<UserInfo>,
) -> Result<Json<UserInfo>, Status> {
    let db = db as &DatabaseConnection;
    let token = token.token;

    let ret = validate_token(db, &token).await?;

    let ret = match AppUser::find()
        .filter(app_user::Column::Id.eq(ret.user_id))
        .one(db)
        .await
    {
        Ok(op) => op,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let user = match ret {
        Some(user) => user,
        None => return Err(Status::Unauthorized),
    };

    let UserInfo {
        id: _,
        creator: _,
        id_no,
        name,
        phone,
        address,
        image,
    } = user_info.into_inner();

    let user_info = user_info_db::ActiveModel {
        creator: Set(user.id),
        id_no: Set(id_no),
        name: Set(name),
        phone: Set(phone),
        address: Set(address),
        image: Set(image),
        ..Default::default()
    };

    let user_info = match user_info.insert(db).await {
        Ok(op) => op,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    Ok(Json(UserInfo {
        id: Some(user_info.id),
        creator: user_info.creator,
        id_no: user_info.id_no,
        name: user_info.name,
        phone: user_info.phone,
        address: user_info.address,
        image: user_info.image,
    }))
}

#[delete("/user-info/delete?<user_info_id>")]
pub async fn delete_user_info(
    db: &State<DatabaseConnection>,
    token: BearerToken,
    user_info_id: Uuid,
) -> Result<Status, Status> {
    let db = db as &DatabaseConnection;
    let token = token.token;

    let ret = validate_token(db, &token).await?;

    let ret = match AppUser::find()
        .filter(app_user::Column::Id.eq(ret.user_id))
        .one(db)
        .await
    {
        Ok(op) => op,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let user = match ret {
        Some(user) => user,
        None => return Err(Status::Unauthorized),
    };

    let user_info = match UserInfoDb::delete_many()
        .filter(user_info_db::Column::Id.eq(user_info_id))
        .filter(user_info_db::Column::Creator.eq(user.id))
        .exec(db)
        .await
    {
        Ok(op) => op,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    if user_info.rows_affected == 0 {
        return Err(Status::NotFound);
    }

    Ok(Status::Ok)
}

#[derive(Serialize, Deserialize)]
pub struct ModifyingUserInfo {
    pub id: Uuid,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub image: Option<Uuid>,
}

#[put("/user-info/set", data = "<user_info>")]
pub async fn set_user_info(
    db: &State<DatabaseConnection>,
    token: BearerToken,
    user_info: Json<ModifyingUserInfo>,
) -> Result<Status, Status> {
    let db = db as &DatabaseConnection;
    let token = token.token;

    let ret = validate_token(db, &token).await?;

    let ret = match AppUser::find()
        .filter(app_user::Column::Id.eq(ret.user_id))
        .one(db)
        .await
    {
        Ok(op) => op,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let user = match ret {
        Some(user) => user,
        None => return Err(Status::Unauthorized),
    };

    let ModifyingUserInfo {
        id,
        phone,
        address,
        image,
    } = user_info.into_inner();

    let user_info = match UserInfoDb::find()
        .filter(user_info_db::Column::Id.eq(id))
        .filter(user_info_db::Column::Creator.eq(user.id))
        .one(db)
        .await
    {
        Ok(op) => op,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let mut user_info = match user_info {
        Some(user_info) => user_info.into_active_model(),
        None => return Err(Status::NotFound),
    };

    if let Some(phone) = phone {
        user_info.phone = Set(phone);
    }

    if let Some(address) = address {
        user_info.address = Set(address);
    }

    if let Some(image) = image {
        user_info.image = Set(Some(image));
    }

    match user_info.update(db).await {
        Ok(_) => Ok(Status::Ok),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(Status::InternalServerError)
        }
    }
}
