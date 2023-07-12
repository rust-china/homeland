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

        let post_category = category::ActiveModel {
            code: Set("POST".to_owned()),
            name: Set("文章".to_owned()),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;
        {
            let children = vec![
                category::ActiveModel {
                    code: Set("RUST".to_owned()),
                    name: Set("Rust".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("CRATE".to_owned()),
                    name: Set("Crate".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("WASM".to_owned()),
                    name: Set("WebAssembly".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("TEST".to_owned()),
                    name: Set("测试".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("DEVTOOL".to_owned()),
                    name: Set("开发工具".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("DATABASE".to_owned()),
                    name: Set("数据库".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("FRONTEND".to_owned()),
                    name: Set("前端".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("DEPLOYMENT".to_owned()),
                    name: Set("部署".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("OPEN_SOURCE".to_owned()),
                    name: Set("开源项目".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("OS".to_owned()),
                    name: Set("操作系统".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("TRANSLATION".to_owned()),
                    name: Set("翻译".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("BOOK".to_owned()),
                    name: Set("书籍".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("Operation".to_owned()),
                    name: Set("运维".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("QUESTION".to_owned()),
                    name: Set("新手问题".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("PRODUCTION_POPULARIZATION".to_owned()),
                    name: Set("产品推广".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("JUST_SAY_IT".to_owned()),
                    name: Set("随便说说".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("ANNOUNCEMENT".to_owned()),
                    name: Set("公告".to_owned()),
                    ..Default::default()
                },
                category::ActiveModel {
                    code: Set("OTHERS".to_owned()),
                    name: Set("其他".to_owned()),
                    ..Default::default()
                },
            ];
            for mut child in children.into_iter() {
                child.set_parent(Some(&post_category));
                child.insert(&transaction).await?;
            }
        }

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

        category::ActiveModel {
            code: Set("NO_POINT".to_owned()),
            name: Set("NoPoint".to_owned()),
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
