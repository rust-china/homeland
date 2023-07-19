//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "storage")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    #[sea_orm(unique)]
    pub path: String,
    pub size: i32,
    pub content_type: Option<String>,
    pub visited_count: i32,
    pub today_visited_count: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn update_visit(&self) -> ActiveModel {
        let mut active: ActiveModel = self.clone().into();
        active.visited_count = Set(self.visited_count + 1);
        if self.updated_at.format("%Y-%m-%d").to_string() == chrono::Utc::now().format("%Y-%m-%d").to_string() {
            active.today_visited_count = Set(self.today_visited_count + 1);
        } else {
            active.today_visited_count = Set(0);
        }
        active.updated_at = Set(chrono::Utc::now().naive_utc());
        active
    }
}
