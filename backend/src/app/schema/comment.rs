use async_graphql::*;
use entity::{comment, prelude::*, user};
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct CommentQuery;
#[Object]
impl CommentQuery {
    pub async fn comment_list(&self, ctx: &Context<'_>, query: GraCommentListQuery) -> Result<GraCommentList> {
        let uuid = Uuid::parse_str(&query.post_uuid)?;
        let state = ctx.data::<Arc<crate::AppState>>()?;
        let db_post = Post::find_by_uuid(uuid)
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("post not exists".into())))?;

        let mut condition = Condition::all().add(comment::Column::PostId.eq(db_post.id));
        if let Some(ancestry) = query.ancestry {
            condition = condition.add(comment::Column::Ancestry.contains(&ancestry));
        } else {
            condition = condition.add(comment::Column::Ancestry.is_null());
        }
        let comment_paginator = Comment::find().filter(condition).paginate(&state.db_conn, query.page_size);
        let mut pagination: super::GraPagination = comment_paginator.num_items_and_pages().await?.into();
        pagination.page_no = Some(query.page_no);
        pagination.page_size = Some(query.page_size);

        let page_comment_list = comment_paginator
            .fetch_page(query.page_no - 1)
            .await?
            .into_iter()
            .map(|model| GraComment {
                parent_id: model.parent_id(),
                id: model.id,
                body: model.body,
                user_id: model.user_id,
                ancestry: model.ancestry,
                created_at: model.created_at,
                updated_at: model.updated_at,
            })
            .collect();

        Ok(GraCommentList {
            records: page_comment_list,
            pagination,
        })
    }
    pub async fn comment(&self, ctx: &Context<'_>, id: i32) -> Result<GraComment> {
        let state = ctx.data::<Arc<crate::AppState>>()?;
        let model = Comment::find_by_id(id)
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("comment not exists".into())))?;

            Ok(GraComment {
                parent_id: model.parent_id(),
                id: model.id,
                user_id: model.user_id,
                body: model.body,
                ancestry: model.ancestry,
                updated_at: model.updated_at,
                created_at: model.created_at,
            })
        
    }
}

#[derive(Default)]
pub struct CommentMutation;
#[Object]
impl CommentMutation {
    pub async fn comment_create(&self, ctx: &Context<'_>, input: GraCommentCreate) -> Result<i32> {
        let state = ctx.data::<Arc<crate::AppState>>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| crate::Error::Message("should login".into()))
            .map_err(|e| e.extend_with(|_, e| e.set("code", 401)))?;

        let parent_comment = {
            match input.parent_id {
                Some(parent_id) => Some(
                    comment::Entity::find_by_id(parent_id)
                        .one(&state.db_conn)
                        .await?
                        .ok_or_else(|| Error::new_with_source(crate::Error::Message("no parent comment".into())))?,
                ),
                _ => None,
            }
        };
        let parent_comment = {
            if let Some(parent_comment) = &parent_comment {
                Some(parent_comment)
            } else {
                None
            }
        };
        let uuid = Uuid::parse_str(&input.post_uuid)?;
        let db_post = Post::find_by_uuid(uuid)
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("post not exists".into())))?;

        let mut comment = comment::ActiveModel {
            body: Set(input.body),
            user_id: Set(claims.sub.user_id),
            post_id: Set(db_post.id),
            ..Default::default()
        };
        comment.set_parent(parent_comment);
        let comment: comment::Model = comment.insert(&state.db_conn).await?;
        Ok(comment.id)
    }
    pub async fn comment_update(&self, ctx: &Context<'_>, input: GraCommentUpdate) -> Result<bool> {
        let state = ctx.data::<Arc<crate::AppState>>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| crate::Error::Message("should login".into()))?;

        let comment = Comment::find()
            .filter(Condition::all().add(comment::Column::Id.eq(input.id)).add(comment::Column::UserId.eq(claims.sub.user_id)))
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| Error::new_with_source(crate::Error::Message("no comment".into())))?;
        let mut comment: comment::ActiveModel = comment.into();
        comment.body = Set(input.body);
        if let Some(parent_id) = input.parent_id {
            let parent_comment = comment::Entity::find_by_id(parent_id)
                .one(&state.db_conn)
                .await?
                .ok_or_else(|| crate::Error::Message("no parent comment".into()))?;
            comment.set_parent(Some(&parent_comment))
        }

        let _comment: comment::Model = comment.update(&state.db_conn).await?;
        Ok(true)
    }
    pub async fn comment_delete(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let state = ctx.data::<Arc<crate::AppState>>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| crate::Error::Message("should login".into()))
            .map_err(|e| e.extend_with(|_, e| e.set("code", 401)))?;

        let comment = Comment::find()
            .filter(Condition::all().add(comment::Column::Id.eq(id)).add(comment::Column::UserId.eq(claims.sub.user_id)))
            .one(&state.db_conn)
            .await?
            .ok_or_else(|| crate::Error::Message("no comment".into()))?;
        let ret = comment.delete(&state.db_conn).await?;
        Ok(ret.rows_affected > 0)
    }
}

#[derive(InputObject)]
pub struct GraCommentListQuery {
    post_uuid: String,
    #[graphql(default)]
    ancestry: Option<String>,
    #[graphql(default = 1)]
    page_no: u64,
    #[graphql(default = 20)]
    page_size: u64,
}

#[derive(SimpleObject, FromQueryResult)]
#[graphql(complex)]
pub struct GraComment {
    id: i32,
    #[graphql(skip)]
    body: String,
    user_id: i32,
    ancestry: Option<String>,
    parent_id: Option<i32>,
    updated_at: chrono::NaiveDateTime,
    created_at: chrono::NaiveDateTime,
}
#[ComplexObject]
impl GraComment {
    async fn body_html(&self) -> String {
        ::backend::markdown::render_markdown(&self.body)
    }
    async fn user(&self, ctx: &Context<'_>) -> Result<serde_json::Value> {
        let state = ctx.data::<Arc<crate::AppState>>()?;
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
pub struct GraCommentList {
    records: Vec<GraComment>,
    pagination: super::GraPagination,
}

#[derive(InputObject)]
pub struct GraCommentCreate {
    post_uuid: String,
    body: String,
    parent_id: Option<i32>,
}

#[derive(InputObject)]
pub struct GraCommentUpdate {
    id: i32,
    body: String,
    parent_id: Option<i32>,
}
