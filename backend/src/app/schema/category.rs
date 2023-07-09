use async_graphql::*;
use entity::{category, prelude::*};

#[derive(Default, Debug)]
pub struct CategoryQuery;
#[Object]
impl CategoryQuery {
    pub async fn categories(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] ancestry: Option<String>,
        #[graphql(default = 1)] page_no: u64,
        #[graphql(default = 20)] page_size: u64,
    ) -> Result<serde_json::Value> {
        let state = ctx.data::<crate::AppState>()?;

        let mut condition = Condition::all().add(category::Column::Ancestry.is_null());
        if let Some(ancestry) = ancestry {
            condition = condition.add(category::Column::Ancestry.contains(&ancestry));
        }
        let category_paginator = Category::find().filter(condition).paginate(&state.db_conn, page_size);
        let page_categories = category_paginator.fetch_page(page_no - 1).await?;
        let mut map = serde_json::Map::new();
        map.insert("total_count".into(), category_paginator.num_items().await?.into());
        map.insert("total_page".into(), category_paginator.num_pages().await?.into());
        map.insert("cur_page".into(), page_no.into());
        map.insert("page_size".into(), page_size.into());
        map.insert("data".into(), serde_json::json!(page_categories));

        Ok(serde_json::Value::Object(map))
    }
    pub async fn category(&self, ctx: &Context<'_>, id: i32) -> Result<serde_json::Value> {
        let state = ctx.data::<crate::AppState>()?;
        let db_category = Category::find()
            .select_only()
            .columns([
                category::Column::Id,
                category::Column::Name,
                category::Column::Code,
                category::Column::CreatedAt,
                category::Column::UpdatedAt,
            ])
            .filter(category::Column::Id.eq(id))
            .into_json()
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("category not exists".into())))?;
        Ok(db_category)
    }
}

#[derive(InputObject)]
pub struct CreateCategory {
    code: String,
    name: String,
    parent_id: i32,
}

#[derive(InputObject)]
pub struct UpdateCategory {
    id: i32,
    code: String,
    name: String,
    parent_id: Option<i32>,
}

#[derive(Default)]
pub struct CategoryMutation;
#[Object]
impl CategoryMutation {
    pub async fn create_category(&self, ctx: &Context<'_>, input: CreateCategory) -> Result<i32> {
        let state = ctx.data::<crate::AppState>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| crate::Error::Message("should login".into()))
            .map_err(|e| e.extend_with(|_, e| e.set("code", 401)))?;

        let parent_category = category::Entity::find_by_id(input.parent_id)
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("no parent category".into())))?;

        let mut category = category::ActiveModel {
            name: Set(input.name),
            code: Set(input.code),
            user_id: Set(Some(claims.sub.user_id)),
            ..Default::default()
        };
        category.set_parent(Some(parent_category));
        let category: category::Model = category.insert(&state.db_conn).await?;
        Ok(category.id)
    }
    pub async fn update_category(&self, ctx: &Context<'_>, input: UpdateCategory) -> Result<bool> {
        let state = ctx.data::<crate::AppState>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| crate::Error::Message("should login".into()))?;

        let category = Category::find()
            .filter(Condition::all().add(category::Column::Id.eq(input.id)).add(category::Column::UserId.eq(claims.sub.user_id)))
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("no category".into())))?;
        let mut category: category::ActiveModel = category.into();
        category.name = Set(input.name);
        category.code = Set(input.code);
        if let Some(parent_id) = input.parent_id {
            let parent_category = category::Entity::find_by_id(parent_id)
                .one(&state.db_conn)
                .await?
                .ok_or_else(|| crate::Error::Message("no parent category".into()))?;
            category.set_parent(Some(parent_category))
        }

        let _category: category::Model = category.update(&state.db_conn).await?;
        Ok(true)
    }
    pub async fn delete_category(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let state = ctx.data::<crate::AppState>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| crate::Error::Message("should login".into()))
            .map_err(|e| e.extend_with(|_, e| e.set("code", 401)))?;

        let category = Category::find()
            .filter(Condition::all().add(category::Column::Id.eq(id)).add(category::Column::UserId.eq(claims.sub.user_id)))
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| crate::Error::Message("no category".into()))?;
        let ret = category.delete(&state.db_conn).await?;
        Ok(ret.rows_affected > 0)
    }
}
