//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use super::sea_orm_active_enums::UserRole;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "app_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub wechat_id: String,
    pub user_role: UserRole,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_info::Entity")]
    UserInfo,
    #[sea_orm(has_one = "super::we_chat_session::Entity")]
    WeChatSession,
}

impl Related<super::user_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserInfo.def()
    }
}

impl Related<super::we_chat_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WeChatSession.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
