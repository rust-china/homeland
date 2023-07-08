mod auth;
mod category;
mod health;
mod post;

use async_graphql::extensions::{Analyzer, ApolloTracing};
use async_graphql::*;

#[derive(Default, MergedObject)]
pub struct AppQuery(health::HealthQuery, auth::AuthQuery, category::CategoryQuery, post::PostQuery);

#[derive(Default, MergedObject)]
pub struct AppMutation(auth::AuthMutation, category::CategoryMutation, post::PostMutation);

#[derive(Default, MergedSubscription)]
pub struct AppSubscription(post::PostSubscription);

pub type AppSchema = Schema<AppQuery, AppMutation, AppSubscription>;

pub fn build_schema() -> AppSchema {
    Schema::build(AppQuery::default(), AppMutation::default(), AppSubscription::default())
        .extension(Analyzer) // 启用 Analyzer 扩展
        .extension(ApolloTracing) // 启用 ApolloTracing 扩展
        // .limit_depth(5) // 深度 限制最大深度为 5
        // .limit_complexity(5) // 复杂度 限制最大深度为 5
        .finish()
}
