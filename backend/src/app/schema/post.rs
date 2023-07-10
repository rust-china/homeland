use async_graphql::*;
use entity::{post, prelude::*, user};
// use futures_util::StreamExt;
use std::time::Duration;
use tokio_stream::{Stream, StreamExt};

use super::SimpleBroker;

#[derive(Default, Debug)]
pub struct PostQuery;
#[Object]
impl PostQuery {
    pub async fn post_list(&self, ctx: &Context<'_>, query: GPostListQuery) -> Result<GPostList> {
        let state = ctx.data::<crate::AppState>()?;

        let mut condition = Condition::all();
        if let Some(title) = query.title {
            condition = condition.add(post::Column::Title.contains(&title));
        }
        let mut post_list = Post::find().filter(condition);
        match query.sort {
            Some(sort) if sort == GPostSort::Last => post_list = post_list.order_by_desc(post::Column::CreatedAt),
            Some(sort) if sort == GPostSort::Excellent => post_list = post_list.order_by_desc(post::Column::Score),
            Some(sort) if sort == GPostSort::Popular => post_list = post_list.order_by_desc(post::Column::CommentCount),
            Some(sort) if sort == GPostSort::LastComment => post_list = post_list.order_by_desc(post::Column::LastCommentAt),
            _ => (),
        }

        let post_paginator = post_list.paginate(&state.db_conn, query.page_size);
        let mut pagination: super::GPagination = post_paginator.num_items_and_pages().await?.into();
        pagination.page_no = Some(query.page_no);
        pagination.page_size = Some(query.page_size);

        let page_post_list = post_paginator
            .fetch_page(query.page_no - 1)
            .await?
            .into_iter()
            .map(|model| GPost {
                uuid: model.uuid.to_string(),
                title: model.title,
                body: None,
                user_id: model.user_id,
                like_count: model.like_count,
                comment_count: model.comment_count,
                last_comment_at: model.last_comment_at,
                created_at: model.created_at,
                updated_at: model.updated_at,
            })
            .collect();

        Ok(GPostList {
            records: page_post_list,
            pagination,
        })
    }
    pub async fn post(&self, ctx: &Context<'_>, uuid: String) -> Result<GPost> {
        let state = ctx.data::<crate::AppState>()?;
        let uuid = Uuid::parse_str(&uuid)?;
        let model = Post::find_by_uuid(uuid)
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("post not exists".into())))?;
        Ok(GPost {
            uuid: model.uuid.to_string(),
            title: model.title,
            body: Some(model.body),
            user_id: model.user_id,
            like_count: model.like_count,
            comment_count: model.comment_count,
            last_comment_at: model.last_comment_at,
            updated_at: model.updated_at,
            created_at: model.created_at,
        })
    }
}

#[derive(Default)]
pub struct PostMutation;
#[Object]
impl PostMutation {
    pub async fn post_create(&self, ctx: &Context<'_>, input: GPostCreate) -> Result<String> {
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

        SimpleBroker::publish(GSPostChange {
            mutation_type: MutationType::Created,
            uuid: post.uuid.to_string(),
        });

        Ok(post.uuid.to_string())
    }
    pub async fn post_update(&self, ctx: &Context<'_>, input: GPostUpdate) -> Result<bool> {
        let state = ctx.data::<crate::AppState>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("should login".into())))?;

        // let post = post::Entity::find_by_id(id).one(&state.db_conn).await?;
        let post = Post::find()
            .filter(
                Condition::all()
                    .add(post::Column::Uuid.eq(Uuid::parse_str(&input.uuid)?))
                    .add(post::Column::UserId.eq(claims.sub.user_id)),
            )
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("no post".into())))?;
        let mut post: post::ActiveModel = post.into();
        post.category_id = Set(input.category_id);
        post.title = Set(input.title);
        post.body = Set(input.body);
        let post: post::Model = post.update(&state.db_conn).await?;

        SimpleBroker::publish(GSPostChange {
            mutation_type: MutationType::Updated,
            uuid: post.uuid.to_string(),
        });

        Ok(true)
    }
    pub async fn post_delete(&self, ctx: &Context<'_>, uuid: String) -> Result<bool> {
        let state = ctx.data::<crate::AppState>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("should login".into())))?;

        let post = Post::find()
            .filter(
                Condition::all()
                    .add(post::Column::Uuid.eq(Uuid::parse_str(&uuid)?))
                    .add(post::Column::UserId.eq(claims.sub.user_id)),
            )
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("no post".into())))?;
        let ret = post.delete(&state.db_conn).await?;

        SimpleBroker::publish(GSPostChange {
            mutation_type: MutationType::Updated,
            uuid,
        });

        Ok(ret.rows_affected > 0)
    }
}

#[derive(Default)]
pub struct PostSubscription;
#[Subscription]
impl PostSubscription {
    pub async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        let interval = tokio::time::interval(Duration::from_secs(1));
        let stream = tokio_stream::wrappers::IntervalStream::new(interval);
        stream.map(move |_| {
            value += step;
            value
        })
    }
    pub async fn post_change(&self, uuids: Option<Vec<String>>) -> impl Stream<Item = GSPostChange> {
        SimpleBroker::<GSPostChange>::subscribe().filter(move |event| if let Some(uuids) = &uuids { uuids.contains(&event.uuid) } else { true })
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum GPostSort {
    Default,
    Last,
    Excellent,
    Popular,
    LastComment,
}
#[derive(InputObject)]
pub struct GPostListQuery {
    title: Option<String>,
    sort: Option<GPostSort>,
    #[graphql(default = 1)]
    page_no: u64,
    #[graphql(default = 20)]
    page_size: u64,
}

#[derive(SimpleObject, FromQueryResult)]
#[graphql(complex)]
pub struct GPost {
    uuid: String,
    title: String,
    #[graphql(skip)]
    body: Option<String>,
    user_id: i32,
    like_count: i32,
    comment_count: i32,
    last_comment_at: Option<chrono::NaiveDateTime>,
    updated_at: chrono::NaiveDateTime,
    created_at: chrono::NaiveDateTime,
}
#[ComplexObject]
impl GPost {
    async fn body(&self, ctx: &Context<'_>) -> String {
        let claims = ctx.data::<Option<crate::serve::jwt::Claims>>();
        if let Ok(Some(claims)) = claims {
            if claims.sub.user_id == self.user_id {
                if let Some(body) = &self.body {
                    return body.clone();
                }
            }
        }
        "".to_string()
    }
    async fn body_html(&self) -> String {
        if let Some(body) = &self.body {
            let parser = pulldown_cmark::Parser::new(body);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            html_output
        } else {
            "".to_string()
        }
    }
    async fn user(&self, ctx: &Context<'_>) -> Result<serde_json::Value> {
        let state = ctx.data::<crate::AppState>()?;
        let db_user = User::find_by_id(self.user_id)
            .select_only()
            .columns([
                user::Column::Id,
                user::Column::Username,
                user::Column::Name,
                user::Column::Email,
                user::Column::CreatedAt,
                user::Column::UpdatedAt,
            ])
            .into_json()
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| crate::Error::Message("user not exists".into()))?;
        Ok(db_user)
    }
}

#[derive(SimpleObject)]
pub struct GPostList {
    records: Vec<GPost>,
    pagination: super::GPagination,
}

#[derive(InputObject)]
pub struct GPostCreate {
    category_id: i32,
    title: String,
    body: String,
}

#[derive(InputObject)]
pub struct GPostUpdate {
    category_id: i32,
    uuid: String,
    title: String,
    body: String,
}

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
enum MutationType {
    Created,
    Updated,
    Deleted,
}
#[derive(Clone, PartialEq)]
pub struct GSPostChange {
    mutation_type: MutationType,
    uuid: String,
}
#[Object]
impl GSPostChange {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }
    async fn uuid(&self) -> &str {
        &self.uuid
    }
}

// struct OwnGuard {
//     user_id: i32,
// }
// #[async_trait::async_trait]
// impl Guard for OwnGuard {
//     async fn check(&self, ctx: &Context<'_>) -> Result<()> {
//         let claims = ctx
//             .data::<Option<crate::serve::jwt::Claims>>()?
//             .as_ref()
//             .ok_or_else(|| crate::Error::Message("Forbidden".into()))?;
//         if claims.sub.user_id == self.user_id {
//             Ok(())
//         } else {
//             Err(crate::Error::Message("Forbidden".into()).into())
//         }
//     }
// }
