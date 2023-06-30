use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Name).string())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::GithubExtra).json())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Post::Uuid).string().not_null().primary_key())
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Body).string().not_null())
                    .col(ColumnDef::new(Post::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Post::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Post::Table).to_owned()).await?;
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Name,
    Email,
    GithubExtra,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Post {
    Table,
    Uuid,
    Title,
    Body,
    CreatedAt,
    UpdatedAt,
}
