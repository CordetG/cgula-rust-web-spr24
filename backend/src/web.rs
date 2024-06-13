use crate::routes::question::format_tags;
use crate::types::question::Question;
use crate::*;
use askama_axum::Template;
/// The IndexTemplate struct represents a template for rendering an index page with optional
/// question, tags, stylesheet, and error information.
/// Properties:
/// * `question`: The `question` property is an optional reference to a `Question` struct.
/// * `tags`: The `tags` property in the `IndexTemplate` struct is of type `Option<String>`. This means
/// it can either contain a `String` value or be empty (`None`). It is used to store tags related to the
/// question being displayed on the index page.
/// * `stylesheet`: The `stylesheet` property in the `IndexTemplate` struct is a reference to a static
/// string (`&'static str`). This property is used to specify the path or name of the stylesheet that
/// should be included when rendering the `index.html` template.
/// * `error`: The `error` property in the `IndexTemplate` struct is an optional field that holds a
/// message or description of an error that may have occurred. It allows for displaying error messages
/// to the user when rendering the template.
//#[derive(Template)]
//#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    question: Option<&'a Question>,
    tags: Option<String>,
    stylesheet: &'static str,
    error: Option<String>,
}

/// The `impl<'a> IndexTemplate<'a>` block is implementing methods for the `IndexTemplate`
/// struct. Let's break down what each method is doing:
impl<'a> IndexTemplate<'a> {
    /// The function `question` takes a reference to a `Question` struct and returns a new instance with
    /// some additional fields set.
    ///
    /// Arguments:
    ///
    /// * `question`: The `question` parameter in the `question` function is a reference to a `Question`
    /// struct.
    ///
    /// Returns:
    ///
    /// An instance of the struct that the `question` function belongs to.
    fn question(question: &'a Question) -> Self {
        Self {
            question: Some(question),
            tags: question.tags.as_ref().map(format_tags),
            stylesheet: "../../frontend/index.css",
            error: None,
        }
    }

    /// The function `error` creates a new instance with an error message.
    ///
    /// Arguments:
    ///
    /// * `error`: The `error` parameter in the `error` function is a String type that represents an error
    /// message or description.
    ///
    /// Returns:
    ///
    /// An instance of the struct that contains the error message provided as a parameter.
    fn error(error: String) -> Self {
        Self {
            question: None,
            tags: None,
            stylesheet: "/question.css",
            error: Some(error),
        }
    }
}

/// The IndexParams struct has an optional field for storing an ID as a string.
///
/// Properties:
///
/// * `id`: The `id` property in the `IndexParams` struct is an optional field of type `String`. This
/// means that an `IndexParams` instance may or may not have a value for the `id` field. If a value is
/// present, it will be a `String`.
#[derive(Deserialize)]
pub struct IndexParams {
    id: Option<String>,
}

// Reference from joke-repo

/*pub async fn handler_index(
    State(appstate): HandlerAppState,
    Query(params): Query<IndexParams>,
) -> Response {
    let appstate = appstate.read().await;
    let jokebase = &appstate.jokebase;

    let joke = if let Some(id) = params.id {
        jokebase.get(&id).await
    } else {
        match jokebase.get_random().await {
            Ok(joke) => return Redirect::to(&format!("/?id={}", joke.id)).into_response(),
            e => e,
        }
    };

    match joke {
        Ok(joke) => (StatusCode::OK, IndexTemplate::joke(&joke)).into_response(),
        Err(JokeBaseErr::JokeDoesNotExist(id)) => (
            StatusCode::OK,
            IndexTemplate::error(format!("cannot find joke {}", id)),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            IndexTemplate::error(e.to_string()),
        )
            .into_response(),
    }
}
 */

/// The `AddParams` struct represents parameters for adding a question with an ID, title, content,
/// and optional tags.
///
/// Properties:
///
/// * `id`: The `id` property in the `AddParams` struct is of type `String` and is used to store an
/// identifier for the item being added.
/// * `title`: The `title` property in the `AddParams` struct represents the title of the item being
/// added. It is of type `String`.
/// * `content`: The `AddParams` struct has the following properties:
/// * `tags`: The `tags` property in the `AddParams` struct is an optional field of type `String`. This
/// means that it can either be `Some(String)` if a value is provided, or `None` if no value is
/// provided.
#[derive(Deserialize)]
pub struct AddParams {
    id: String,
    title: String,
    content: String,
    tags: Option<String>,
}

/// The function `parse_tags` takes an optional string of tags, splits them by comma, trims whitespace,
/// and returns a HashSet of non-empty tags if any.
///
/// Arguments:
///
/// * `tags`: The `parse_tags` function takes an `Option<String>` as input, which represents an optional
/// string of tags separated by commas.
///
/// Returns:
///
/// The function `parse_tags` returns an `Option<HashSet<String>>`.
fn parse_tags(tags: Option<String>) -> Option<HashSet<String>> {
    let tags = tags?;
    if tags.is_empty() {
        return None;
    }
    let tags: HashSet<String> = tags.split(',').map(str::trim).map(str::to_string).collect();
    if tags.is_empty() {
        None
    } else {
        Some(tags)
    }
}

// Reference from joke repo
// * pub async fn handler_add(
//     State(appstate): HandlerAppState,
//     Query(params): Query<AddParams>,
//     session: Session,
// ) -> Response {
//     // XXX Condition user input.
//     let joke = Joke {
//         id: params.id.clone(),
//         whos_there: params.who,
//         answer_who: params.answer,
//         tags: parse_tags(params.tags),
//         source: parse_source(params.source),
//     };
//
//     let mut appstate = appstate.write().await;
//
//     match appstate.jokebase.add(joke).await {
//         Ok(()) => Redirect::to(&format!("/?id={}", params.id)).into_response(),
//         Err(JokeBaseErr::JokeBaseIoError(msg)) => {
//             (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
//         }
//         Err(JokeBaseErr::JokeExists(id)) => {
//             let error = Some(format!("joke {} already exists", id));
//             let _ = session.insert(SESSION_ERROR_KEY, error).await;
//             Redirect::to("/tell").into_response()
//         }
//         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
//     }
// }
