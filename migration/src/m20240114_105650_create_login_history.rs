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
                    .table(WeChatSession::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WeChatSession::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(WeChatSession::UserId)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WeChatSession::LastLogin)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(WeChatSession::LastSession)
                            .string_len(30)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(WeChatSession::LastToken)
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
                    .from(WeChatSession::Table, WeChatSession::UserId)
                    .to(AppUser::Table, AppUser::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(WeChatSession::Table)
                    .name("fk_login_history_user_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(WeChatSession::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WeChatSession {
    Table,
    Id,
    UserId,
    LastLogin,
    LastSession,
    LastToken,
}
