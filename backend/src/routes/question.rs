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
use crate::types::pagination::Pagination;
use crate::types::question::{Question, QuestionId};
use tracing::{event, Level};

use axum::extract::{self, path, Extension, Path, State};
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};

use serde_json::json;
use std::collections::{HashMap, HashSet};
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
) -> Result<impl IntoResponse, StatusCode> {
    event!(target: "backend", Level::INFO, "querying questions");
    let mut pagination: crate::types::pagination::Pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }
    match store.get_questions().await {
        Ok(questions) => Ok(axum::Json(questions)),
        Err(e) => {
            event!(Level::ERROR, error = %e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
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

/// The function `format_tags` takes a HashSet of strings and returns a formatted string with the tags
/// separated by commas.
///
/// Arguments:
///
/// * `tags`: The `format_tags` function takes a reference to a `HashSet` of `String` values as input.
/// It then converts the `HashSet` into a vector of string references and joins them together with a
/// comma and space to create a single formatted string.
///
/// Returns:
///
/// A formatted string containing the tags from the HashSet, separated by commas.
pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

impl Question {
    pub fn new(
        id: QuestionId,
        title: &str,
        content: &str,
        tags: &[&str],
        source: Option<&str>,
    ) -> Self {
        let id: QuestionId = id;
        let title: String = title.into();
        let content: String = content.into();
        let tags: Option<HashSet<String>> = if tags.is_empty() {
            None
        } else {
            Some(tags.iter().copied().map(String::from).collect())
        };
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

impl From<&Question> for String {
    fn from(question: &Question) -> Self {
        let mut text: String = "Question:\n".into();
        text += &format!("{}.\n", question.title);
        text += &format!("{}\n", question.content);
        text += "\n";

        let mut annote: Vec<String> = vec![format!("id: {}", question.id)];
        if let Some(tags) = &question.tags {
            annote.push(format!("tags: {}", format_tags(tags)));
        }
        let annote: String = annote.join("; ");
        text += &format!("[{}]\n", annote);
        text
    }
}

impl IntoResponse for &Question {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response()
    }
}
