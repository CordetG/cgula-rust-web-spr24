//! `question.rs`
//!
//! This module contains the `QuestionStruct` struct and its associated functions.
//!
//! The `QuestionStruct` struct represents a question with an id, title, content, and optional tags.
//! It is derived from the Properties, Clone, PartialEq, and Deserialize traits.
//!
//! The `get_question` function is an asynchronous function that retrieves a question from the server.
//! It takes an optional string as an argument, which may be used to get a specific question.
//! The function sends a GET request to the server and returns a message indicating the result of the operation.
//!
//! This module is particularly useful for managing questions in a Q&A application.

use crate::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
/// Represents a question with an id, title, content, and optional tags.
pub struct QuestionStruct {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Option<HashSet<String>>,
}

impl QuestionStruct {
    /// Retrieves a question from the server.
    ///
    /// # Arguments
    ///
    /// * `key` - An optional string that may be used to get a specific question.
    ///
    /// # Returns
    ///
    /// * `Msg` - A message indicating the result of the operation.
    pub async fn get_question(key: Option<String>) -> Msg {
        let request: String = match &key {
            None => "http://localhost:3000/api/v1/question".to_string(),
            Some(ref key) => format!("http://localhost:3060/api/v1/question/{}", key,),
        };
        let response: Result<http::Response, gloo_net::Error> =
            http::Request::get(&request).send().await;
        match response {
            Err(e) => Msg::GotQuestion(Err(e)),
            Ok(data) => Msg::GotQuestion(data.json().await),
        }
    }
}

/// Formats the tags of a question into a comma-separated string.
///
/// # Arguments
///
/// * `tags` - A reference to a HashSet of tags.
///
/// # Returns
///
/// * `String` - A comma-separated string of tags.
pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionProps {
    pub question: QuestionStruct,
}

#[function_component(Question)]
pub fn question(question: &QuestionProps) -> Html {
    let question = &question.question;
    html! { <>
        <div class="question">
            <span class="title">{question.title.clone()}</span><br/>
            <span class="content">{&question.content}</span><br/>
        </div>
        <span class="annotation">
            {format!("[id: {}", &question.id)}
            if let Some(ref tags) = question.tags {
                {format!("; tags: {}", &format_tags(tags))}
            }
            {"]"}
        </span>
    </> }
}
