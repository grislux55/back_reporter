use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_user_table::AppUser;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInfo::Id)
                            .uuid()
                            .primary_key()
                            .unique_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(UserInfo::Creator).uuid().not_null())
                    .col(ColumnDef::new(UserInfo::IdNo).string_len(20).not_null())
                    .col(ColumnDef::new(UserInfo::Name).string_len(32).not_null())
                    .col(ColumnDef::new(UserInfo::Phone).string_len(20).not_null())
                    .col(ColumnDef::new(UserInfo::Address).string_len(64).not_null())
                    .col(ColumnDef::new(UserInfo::Image).uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_info_user_id")
                    .from(UserInfo::Table, UserInfo::Creator)
                    .to(AppUser::Table, AppUser::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(UserInfo::Table)
                    .name("fk_user_info_user_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(UserInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserInfo {
    Table,
    Id,
    Creator,
    IdNo,
    Name,
    Phone,
    Address,
    Image,
}
