use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, QuestionId};

use axum::extract::{self, path, Extension, Path, State};
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};

use serde_json::json;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> axum::Json<Vec<Question>> {
    let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
    if !params.is_empty() {
        let pagination: crate::types::pagination::Pagination = extract_pagination(params).unwrap();
        let res: &[Question] = &res[pagination.start..pagination.end];
        axum::Json(res.into())
    } else {
        axum::Json(res)
    }
}

pub async fn update_question(
    Path(id): Path<String>,
    store: Store,
    question: axum::Json<Question>,
) -> Result<impl IntoResponse, StatusCode> {
    match store
        .questions
        .write()
        .await
        .get_mut(&QuestionId(id.clone()))
    {
        Some(q) => {
            *q = question.0;
            Ok("Question updated")
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_question(
    Path(id): Path<String>,
    store: Store,
) -> Result<impl IntoResponse, StatusCode> {
    match store
        .questions
        .write()
        .await
        .remove(&QuestionId(id.clone()))
    {
        Some(_) => Ok("Question deleted"),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn add_question(store: Store, question: axum::Json<Question>) -> impl IntoResponse {
    let question = question.0;
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    StatusCode::OK
}
