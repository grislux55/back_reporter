use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(UserRole::Table)
                    .values(UserRole::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AppUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppUser::Id)
                            .uuid()
                            .primary_key()
                            .unique_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(
                        ColumnDef::new(AppUser::WechatId)
                            .string_len(30)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(AppUser::UserRole)
                            .enumeration(UserRole::Table, UserRole::iter().skip(1))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AppUser::Table).to_owned())
            .await?;

        let res = manager
            .drop_type(Type::drop().name(UserRole::Table).to_owned())
            .await;

        let db = manager.get_connection();

        db.execute_unprepared("DROP EXTENSION IF EXISTS \"uuid-ossp\"")
            .await?;

        res
    }
}

#[derive(DeriveIden)]
pub enum AppUser {
    Table,
    Id,
    WechatId,
    UserRole,
}

#[derive(DeriveIden, EnumIter)]
pub enum UserRole {
    Table,
    Admin,
    Subadmin,
    Normal,
}
