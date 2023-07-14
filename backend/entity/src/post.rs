//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub user_id: i32,
    pub category_id: i32,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub body: String,
    pub score: i32,
    pub read_count: i32,
    pub like_count: i32,
    pub comment_count: i32,
    pub last_comment_at: Option<DateTime>,
    pub extra_data: Option<Json>,
    pub deleted_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Category,
    #[sea_orm(has_many = "super::comment::Entity")]
    Comment,
    #[sea_orm(has_many = "super::notification::Entity")]
    Notification,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl Related<super::notification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Notification.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

//  https://www.sea-ql.org/SeaORM/docs/next/generate-entity/entity-structure/
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            uuid: Set(uuid::Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}

impl Entity {
    pub fn find_by_uuid(uuid: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Uuid.eq(uuid))
    }
}
