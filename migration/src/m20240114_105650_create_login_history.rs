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
                    .table(LoginHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LoginHistory::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LoginHistory::UserId)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoginHistory::LastLogin)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(LoginHistory::LastToken)
                            .uuid()
                            .unique_key()
                            .not_null()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_login_history_user_id")
                    .from(LoginHistory::Table, LoginHistory::UserId)
                    .to(AppUser::Table, AppUser::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(LoginHistory::Table)
                    .name("fk_login_history_user_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(LoginHistory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LoginHistory {
    Table,
    Id,
    UserId,
    LastLogin,
    LastToken,
}
