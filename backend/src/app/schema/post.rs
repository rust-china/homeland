use entity::{post, prelude::*};
use async_graphql::*;
use futures_util::StreamExt;
use std::time::Duration;

#[derive(Default, Debug)]
pub struct PostQuery;
#[Object]
impl PostQuery {
    pub async fn posts(
        &self,
        ctx: &Context<'_>,
        title: Option<String>,
        #[graphql(default = 1)] page_no: u64,
        #[graphql(default = 20)] page_size: u64,
    ) -> Result<serde_json::Value> {
        let state = ctx.data::<crate::AppState>()?;

        let mut condition = Condition::all();
        if let Some(title) = title {
            condition = condition.add(post::Column::Title.contains(&title));
        }
        let post_paginator = Post::find().filter(condition).paginate(&state.db_conn, page_size);
        let page_posts = post_paginator.fetch_page(page_no - 1).await?;
        let mut map = serde_json::Map::new();
        map.insert("total_count".into(), post_paginator.num_items().await?.into());
        map.insert("total_page".into(), post_paginator.num_pages().await?.into());
        map.insert("cur_page".into(), page_no.into());
        map.insert("page_size".into(), page_size.into());
        map.insert("data".into(), serde_json::json!(page_posts));

        Ok(serde_json::Value::Object(map))
    }
    pub async fn post(&self, ctx: &Context<'_>, uuid: String) -> Result<serde_json::Value> {
        let state = ctx.data::<crate::AppState>()?;
        let db_post = Post::find()
            .select_only()
            .columns([
                // post::Column::Id,
                post::Column::Uuid,
                post::Column::Body,
                post::Column::CreatedAt,
                post::Column::UpdatedAt,
            ])
            .filter(post::Column::Uuid.eq(uuid))
            .into_json()
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("post not exists".into())))?;
        Ok(db_post)
    }
}

#[derive(InputObject)]
pub struct CreatePost {
	category_id: i32,
	title: String,
    body: String,
}

#[derive(InputObject)]
pub struct UpdatePost {
    category_id: i32,
	uuid: String,
	title: String,
    body: String,
}

#[derive(Default)]
pub struct PostMutation;
#[Object]
impl PostMutation {
    pub async fn create_post(&self, ctx: &Context<'_>, input: CreatePost) -> Result<serde_json::Value> {
        let state = ctx.data::<crate::AppState>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("should login".into())))?;

        let post = post::ActiveModel {
            title: Set(input.title),
            body: Set(input.body),
            category_id: Set(input.category_id),
            user_id: Set(claims.sub.user_id),
            ..Default::default()
        };
        let post: post::Model = post.insert(&state.db_conn).await?;
        Ok(serde_json::json!(post))
    }
    pub async fn update_post(&self, ctx: &Context<'_>, input: UpdatePost) -> Result<serde_json::Value> {
        let state = ctx.data::<crate::AppState>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("should login".into())))?;

        // let post = post::Entity::find_by_id(id).one(&state.db_conn).await?;
        let post = Post::find()
            .filter(Condition::all().add(post::Column::Uuid.eq(Uuid::parse_str(&input.uuid)?)).add(post::Column::UserId.eq(claims.sub.user_id)))
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("no post".into())))?;
        let mut post: post::ActiveModel = post.into();
        post.category_id = Set(input.category_id);
        post.title = Set(input.title);
        post.body = Set(input.body);
        let post: post::Model = post.update(&state.db_conn).await?;
        Ok(serde_json::json!(post))
    }
    pub async fn delete_post(&self, ctx: &Context<'_>, uuid: String) -> Result<bool> {
        let state = ctx.data::<crate::AppState>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("should login".into())))?;

        let post = Post::find()
            .filter(Condition::all().add(post::Column::Uuid.eq(Uuid::parse_str(&uuid)?)).add(post::Column::UserId.eq(claims.sub.user_id)))
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("no post".into())))?;
        let ret = post.delete(&state.db_conn).await?;
        Ok(ret.rows_affected > 0)
    }
}

#[derive(Default)]
pub struct PostSubscription;

#[Subscription]
impl PostSubscription {
    pub async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl tokio_stream::Stream<Item = i32> {
        let mut value = 0;
        let interval = tokio::time::interval(Duration::from_secs(1));
        let stream = tokio_stream::wrappers::IntervalStream::new(interval);
        stream.map(move |_| {
            value += step;
            value
        })
    }
}
