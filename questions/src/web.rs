use crate::*;

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

impl<'a> IndexTemplate<'a> {
    fn question(question: &'a Question) -> Self {
        Self {
            question: Some(question),
            tags: question.tags.as_ref().map(format_tags),
            stylesheet: "/knock-knock.css",
            error: None,
        }
    }
}
