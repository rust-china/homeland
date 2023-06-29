use async_graphql::*;
use futures_util::StreamExt;
use std::time::Duration;

#[derive(SimpleObject)]
pub struct Post {
    id: ID,
    title: String,
    content: String,
}

#[derive(Default)]
pub struct PostQuery;
#[Object]
impl PostQuery {
    pub async fn posts(&self, _ctx: &Context<'_>) -> Vec<Post> {
        let post = Post {
            id: 1.into(),
            title: "first title".into(),
            content: "first content".into(),
        };
        vec![post]
    }
}

#[derive(Default)]
pub struct PostMutation;
#[Object]
impl PostMutation {
    pub async fn create_blog(&self, _ctx: &Context<'_>, _title: String, _content: String) -> ID {
        1.into()
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
