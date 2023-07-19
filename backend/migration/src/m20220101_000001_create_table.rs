use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // user
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(User::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Name).string())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Avatar).string())
                    .col(ColumnDef::new(User::GithubData).json())
                    .col(ColumnDef::new(User::ExtraData).json())
                    .col(ColumnDef::new(User::State).integer().not_null().default(1))
                    .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;
        let username_idx = sea_query::Index::create().name("idx-user-username").table(User::Table).col(User::Username).to_owned();
        manager.create_index(username_idx).await?;

        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Category::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Category::UserId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-category-user_id")
                            .from(Category::Table, Category::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Category::Code).string().not_null().unique_key())
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .col(ColumnDef::new(Category::Position).integer().not_null().default(0))
                    .col(ColumnDef::new(Category::Ancestry).string())
                    .col(ColumnDef::new(Category::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Category::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;
        // post
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Post::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Post::Uuid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Post::UserId).integer().not_null())
                    .foreign_key(ForeignKey::create().name("fk-post-user_id").from(Post::Table, Post::UserId).to(User::Table, User::Id))
                    .col(ColumnDef::new(Post::CategoryId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-category_id")
                            .from(Post::Table, Post::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Body).text().not_null())
                    .col(ColumnDef::new(Post::Score).integer().not_null().default(0))
                    .col(ColumnDef::new(Post::ReadCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Post::LikeCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Post::CommentCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Post::ExtraData).json())
                    .col(ColumnDef::new(Post::LastCommentAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Post::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Post::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Post::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;
        let title_idx = sea_query::Index::create().name("idx-post-title").table(Post::Table).col(Post::Title).to_owned();
        manager.create_index(title_idx).await?;
        // comment
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Comment::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Comment::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-user_id")
                            .from(Comment::Table, Comment::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Comment::PostId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-post_id")
                            .from(Comment::Table, Comment::PostId)
                            .to(Post::Table, Post::Id),
                    )
                    .col(ColumnDef::new(Comment::LikeCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Comment::CommentCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Comment::Ancestry).string())
                    .col(ColumnDef::new(Comment::Body).string().not_null())
                    .col(ColumnDef::new(Comment::ExtraData).json())
                    .col(ColumnDef::new(Comment::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Comment::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Comment::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Like::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Like::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Like::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-like-user_id")
                            .from(Like::Table, Like::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Like::LikeAbleId).integer().not_null())
                    .col(ColumnDef::new(Like::LikeAbleType).string().not_null())
                    .col(ColumnDef::new(Like::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Like::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;

        // notification
        manager
            .create_table(
                Table::create()
                    .table(Notification::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Notification::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Notification::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-notification-user_id")
                            .from(Notification::Table, Notification::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Notification::PostId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-notification-post_id")
                            .from(Notification::Table, Notification::PostId)
                            .to(Post::Table, Post::Id),
                    )
                    .col(ColumnDef::new(Notification::NotificationableId).integer())
                    .col(ColumnDef::new(Notification::NotificationableType).string())
                    .col(ColumnDef::new(Notification::Read).boolean().not_null().default(false))
                    .col(ColumnDef::new(Notification::ExtraData).json())
                    .col(ColumnDef::new(Notification::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Notification::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;

        // storage
        manager
            .create_table(
                Table::create()
                    .table(Storage::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Storage::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Storage::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-storage-user_id")
                            .from(Storage::Table, Storage::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Storage::Path).string().not_null().unique_key())
                    .col(ColumnDef::new(Storage::ContentType).string())
                    .col(ColumnDef::new(Storage::Size).not_null().integer())
                    .col(ColumnDef::new(Storage::VisitedCount).not_null().integer().default(0))
                    .col(ColumnDef::new(Storage::TodayVisitedCount).not_null().integer().default(0))
                    .col(ColumnDef::new(Storage::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Storage::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Category::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Post::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Comment::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Notification::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Storage::Table).to_owned()).await?;
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
    Avatar,
    GithubData,
    ExtraData,
    State,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Category {
    Table,
    Id,
    UserId,
    Code,
    Name,
    Position,
    Ancestry,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Post {
    Table,
    Id,
    Uuid,
    UserId,
    CategoryId,
    Title,
    Body,
    Score,
    ReadCount,
    LikeCount,
    CommentCount,
    LastCommentAt,
    ExtraData,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Comment {
    Table,
    Id,
    UserId,
    PostId,
    Body,
    CommentCount,
    LikeCount,
    Ancestry,
    ExtraData,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Notification {
    Table,
    Id,
    UserId,
    PostId,
    NotificationableType,
    NotificationableId,
    Read,
    ExtraData,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Like {
    Table,
    Id,
    UserId,
    LikeAbleType,
    LikeAbleId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Storage {
    Table,
    Id,
    UserId,
    Path,
    Size,
    ContentType,
    VisitedCount,
    TodayVisitedCount,
    CreatedAt,
    UpdatedAt,
}
