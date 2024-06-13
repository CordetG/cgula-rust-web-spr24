//! # Question Routes
//!
//! This module contains the HTTP routes for operations related to questions.
//!
//! It includes the following routes:
//! * `get_questions`: Fetches a list of questions from the store.
//! * `update_question`: Updates a specific question in the store.
//! * `delete_question`: Deletes a specific question from the store.
//! * `add_question`: Adds a new question to the store.
//!
//! Each route function takes a shared state of the application and other necessary parameters,
//! performs the corresponding operation, and returns a response that can be converted into an HTTP response.

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

/// Fetches a list of questions from the store.
///
/// This function reads the questions from the store and returns them as a JSON response.
/// If pagination parameters are provided, it will return a paginated list of questions.
///
/// # Arguments
///
/// * `params` - A hashmap containing query parameters. It can include pagination parameters.
/// * `store` - The shared state of the application, which includes the list of questions.
///
/// # Returns
///
/// This function returns a JSON response containing a list of questions. If pagination parameters
/// are provided, the list will be paginated according to those parameters.
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

/// Updates a specific question in the store.
///
/// This function takes an ID, a shared state of the application, and a JSON body containing the updated question.
/// It then updates the question in the store with the provided ID.
///
/// # Arguments
///
/// * `id` - The ID of the question to update.
/// * `store` - The shared state of the application, which includes the list of questions.
/// * `question` - The updated question, provided as a JSON body in the request.
///
/// # Returns
///
/// This function returns a response that can be converted into an HTTP response.
/// If the question is successfully updated, it returns a 200 OK response.
/// If the question with the provided ID is not found, it returns a 404 Not Found response.
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

/// Deletes a specific question from the store.
///
/// This function takes an ID and a shared state of the application.
/// It then removes the question with the provided ID from the store.
///
/// # Arguments
///
/// * `id` - The ID of the question to delete.
/// * `store` - The shared state of the application, which includes the list of questions.
///
/// # Returns
///
/// This function returns a response that can be converted into an HTTP response.
/// If the question is successfully deleted, it returns a 200 OK response with a message "Question deleted".
/// If the question with the provided ID is not found, it returns a 404 Not Found response.
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

/// Adds a new question to the store.
///
/// This function takes a shared state of the application and a JSON body containing the new question.
/// It then adds the new question to the store.
///
/// # Arguments
///
/// * `store` - The shared state of the application, which includes the list of questions.
/// * `question` - The new question, provided as a JSON body in the request.
///
/// # Returns
///
/// This function returns a response that can be converted into an HTTP response.
/// If the question is successfully added, it returns a 200 OK response.
pub async fn add_question(store: Store, question: axum::Json<Question>) -> impl IntoResponse {
    let question = question.0;
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    StatusCode::OK
}
