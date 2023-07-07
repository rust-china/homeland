use entity::category;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, query::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let transaction = db.begin().await?;

        category::ActiveModel {
            code: Set("POST".to_owned()),
            name: Set("文章".to_owned()),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;
        category::ActiveModel {
            code: Set("RECRUIT".to_owned()),
            name: Set("招聘".to_owned()),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;
        category::ActiveModel {
            code: Set("WIKI".to_owned()),
            name: Set("Wiki".to_owned()),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
