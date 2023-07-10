use async_graphql::*;
use entity::{category, prelude::*};

#[derive(Default, Debug)]
pub struct CategoryQuery;
#[Object]
impl CategoryQuery {
    pub async fn category_list(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] ancestry: Option<String>,
        #[graphql(default = 1)] page_no: u64,
        #[graphql(default = 20)] page_size: u64,
    ) -> Result<GCategoryList> {
        let state = ctx.data::<crate::AppState>()?;

        let mut condition = Condition::all().add(category::Column::Ancestry.is_null());
        if let Some(ancestry) = ancestry {
            condition = condition.add(category::Column::Ancestry.contains(&ancestry));
        }
        let category_paginator = Category::find().filter(condition).paginate(&state.db_conn, page_size);
        let mut pagination: super::GPagination = category_paginator.num_items_and_pages().await?.into();
        pagination.page_no = Some(page_no);
        pagination.page_size = Some(page_size);

        let page_category_list = category_paginator
            .fetch_page(page_no - 1)
            .await?
            .into_iter()
            .map(|model| GCategory {
                id: model.id,
                name: model.name,
                code: model.code,
                created_at: model.created_at,
                updated_at: model.updated_at,
            })
            .collect();

        Ok(GCategoryList {
            records: page_category_list,
            pagination,
        })
    }
    pub async fn category(&self, ctx: &Context<'_>, id: i32) -> Result<GCategory> {
        let state = ctx.data::<crate::AppState>()?;
        let g_category = Category::find()
            .select_only()
            .columns([
                category::Column::Id,
                category::Column::Name,
                category::Column::Code,
                category::Column::CreatedAt,
                category::Column::UpdatedAt,
            ])
            .filter(category::Column::Id.eq(id))
            .into_model::<GCategory>()
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("category not exists".into())))?;
        Ok(g_category)
    }
}

#[derive(Default)]
pub struct CategoryMutation;
#[Object]
impl CategoryMutation {
    pub async fn category_create(&self, ctx: &Context<'_>, input: GCategoryCreate) -> Result<i32> {
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
    pub async fn category_update(&self, ctx: &Context<'_>, input: GCategoryUpdate) -> Result<bool> {
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
    pub async fn category_delete(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
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

#[derive(SimpleObject, FromQueryResult)]
pub struct GCategory {
    id: i32,
    name: String,
    code: String,
    updated_at: chrono::NaiveDateTime,
    created_at: chrono::NaiveDateTime,
}

#[derive(SimpleObject)]
pub struct GCategoryList {
    records: Vec<GCategory>,
    pagination: super::GPagination,
}

#[derive(InputObject)]
pub struct GCategoryCreate {
    code: String,
    name: String,
    parent_id: i32,
}

#[derive(InputObject)]
pub struct GCategoryUpdate {
    id: i32,
    code: String,
    name: String,
    parent_id: Option<i32>,
}
