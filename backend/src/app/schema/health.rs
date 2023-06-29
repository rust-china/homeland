use async_graphql::*;

#[derive(Debug, Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    async fn health(&self) -> bool {
        true
    }
}
