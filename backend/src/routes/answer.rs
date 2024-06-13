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

async fn add_answer(store: Store, new_answer: Answer) -> impl IntoResponse {
    /*let answer: Answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("questionId").unwrap().to_string()),
    };*/

    store.add_answer(new_answer.clone()).await;

    (
        StatusCode::OK,
        axum::Json(json!({ "message": "Answer added" })),
    )
}
