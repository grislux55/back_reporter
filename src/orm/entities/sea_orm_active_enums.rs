//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "normal")]
    Normal,
    #[sea_orm(string_value = "subadmin")]
    Subadmin,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "validated")]
pub enum Validated {
    #[sea_orm(string_value = "blocked")]
    Blocked,
    #[sea_orm(string_value = "pass")]
    Pass,
    #[sea_orm(string_value = "pending")]
    Pending,
}
