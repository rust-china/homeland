mod auth;
mod health;
mod post;

use async_graphql::*;

#[derive(Default, MergedObject)]
pub struct AppQuery(health::HealthQuery, post::PostQuery);

#[derive(Default, MergedObject)]
pub struct AppMutation(auth::AuthMutation, post::PostMutation);

#[derive(Default, MergedSubscription)]
pub struct AppSubscription(post::PostSubscription);

pub type AppSchema = Schema<AppQuery, AppMutation, AppSubscription>;

pub fn build_schema() -> AppSchema {
    Schema::build(AppQuery::default(), AppMutation::default(), AppSubscription::default()).finish()
}
