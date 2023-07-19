use axum::{
    extract::{Multipart, Path, State},
    http::header,
    response::{AppendHeaders, IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use entity::{prelude::*, storage};
use std::sync::Arc;

pub fn routes() -> Router<Arc<crate::AppState>> {
    Router::new()
        .route(
            "/*path",
            get(|Path(path): Path<String>, State(state): State<Arc<crate::AppState>>| async move {
                if let Ok(Some(db_storage)) = Storage::find()
                    .filter(storage::Column::Path.eq(path.clone()))
                    .into_model::<storage::Model>()
                    .one(&state.db_conn)
                    .await
                {
                    if db_storage.today_visited_count < 10_000 {
                        let storage = db_storage.update_visit();
                        storage.update(&state.db_conn).await?;

                        let signed_url = state.oss_client.signature_url(path, std::time::Duration::from_secs(7200)).await.unwrap();
                        Ok((AppendHeaders([(header::CACHE_CONTROL, format!("max-age={}", 7200))]), Redirect::temporary(&signed_url)))
                    } else {
                        Err(crate::Error::Message(format!("request too times: {}", path)))
                    }
                } else {
                    Err(crate::Error::Message("path not found".into()))
                }
            }),
        )
        .route("/upload", post(upload))
}

pub(crate) async fn upload(State(state): State<Arc<crate::AppState>>, claims: crate::serve::jwt::Claims, mut multipart: Multipart) -> Result<impl IntoResponse, crate::Error> {
    let sub = claims.sub;
    if let Some(file) = multipart.next_field().await.map_err(|err| crate::Error::Message(err.to_string()))? {
        let filename = file.file_name().unwrap().to_string(); // 上传的文件名
        let content_type = file.content_type().ok_or_else(|| crate::Error::Message("require file type".into()))?.to_owned();
        let bytes = file.bytes().await.map_err(|err| crate::Error::Message(err.to_string()))?; // 上传的文件的内容
        let digest = md5::compute(&bytes);
        let path = format!("production/{:?}", digest);

        if let Ok(Some(db_storage)) = Storage::find()
            .filter(storage::Column::Path.eq(path.clone()))
            .into_model::<storage::Model>()
            .one(&state.db_conn)
            .await
        {
            let mut json: serde_json::Value = serde_json::from_str(r#"{"code":0, "data": {}}"#).unwrap();
            json["data"]["path"] = serde_json::json!(format!("/api/storage/{}", db_storage.path));
            json["data"]["filename"] = serde_json::json!(format!("{}", filename));
            return Ok(Json(json));
        }

        state
            .oss_client
            .put(&path, &bytes, &content_type)
            .await
            .map_err(|err| crate::Error::Message(err.to_string()))?;

        let storage: storage::ActiveModel = storage::ActiveModel {
            user_id: Set(sub.user_id),
            path: Set(path),
            size: Set(bytes.len() as i32),
            content_type: Set(Some(content_type)),
            ..Default::default()
        };
        let ret = storage.insert(&state.db_conn).await?;
        let mut json: serde_json::Value = serde_json::from_str(r#"{"code":0, "data": {}}"#).unwrap();
        json["data"]["path"] = serde_json::json!(format!("/api/storage/{}", ret.path));
        json["data"]["filename"] = serde_json::json!(format!("{}", filename));
        Ok(Json(json))
    } else {
        Err(crate::Error::Message("require file".into()))
    }
}
