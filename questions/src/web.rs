use crate::*;
use askama_axum::Template;

/// The IndexTemplate struct in Rust represents a template for rendering an index page with optional
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
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    question: Option<&'a Question>,
    tags: Option<String>,
    stylesheet: &'static str,
    error: Option<String>,
}

/// The `impl<'a> IndexTemplate<'a>` block in Rust is implementing methods for the `IndexTemplate`
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
            stylesheet: "/question.css",
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
