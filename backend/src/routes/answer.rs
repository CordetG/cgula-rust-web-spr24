use axum::extract::{self, path, Extension, Path, State};
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};
use serde_json::json;
use std::collections::HashMap;

use crate::store::Store;
use crate::types::{
    answer::{Answer, AnswerId},
    question::QuestionId,
};

async fn add_answer(
    Extension(store): Extension<Store>,
    Json(params): Json<HashMap<String, String>>,
) -> impl IntoResponse {
    let answer: Answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("questionId").unwrap().to_string()),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    (
        StatusCode::OK,
        axum::Json(json!({ "message": "Answer added" })),
    )
}
