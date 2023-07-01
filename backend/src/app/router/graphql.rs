use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    extract::{Extension, Path, Query, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub fn routes() -> Router<crate::AppState> {
    let schema = crate::app::schema::build_schema();
    Router::new()
        .route("/", get(get_graphiql).post(post_graphql))
        .route("/:version", get(get_graphiql))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(Extension(schema))
}

#[derive(serde::Deserialize)]
pub struct PlaygroundParams {
    version: String,
}
pub async fn get_graphiql(version: Option<Path<String>>, query: Option<Query<PlaygroundParams>>) -> impl IntoResponse {
    let mut is_v2 = false;
    if let Some(Path(version)) = version {
        if version == "v2" {
            is_v2 = true;
        }
    }
    if let Some(Query(query)) = query {
        if &query.version == "v2" {
            is_v2 = true;
        }
    }
    if is_v2 {
        return Html(
            async_graphql::http::GraphiQLSource::build()
                .endpoint("/graphql")
                .subscription_endpoint("/graphql/ws")
                .finish(),
        );
    }

    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql/ws"),
    ))
}

pub async fn post_graphql(State(state): State<crate::AppState>, Extension(schema): Extension<crate::app::schema::AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner().data(state)).await.into()
}
