// From knock-knock/src/api.rs.
// From utoipa/examples/{simple-axum, axum-todo}.
use crate::*;

use crate::appstate::HandlerAppState;
use crate::auth::make_jwt_token;
use crate::auth::Claims;
use crate::auth::Registration;
use axum_core::response::IntoResponse;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        questions,
        question,
        get_question,
        post_question,
        delete_question,
        update_question,
    ),
    components(
        schemas(Question, StoreError)
    ),
    tags(
        (name = "question", description = "Question API")
    )
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api/v1/questions",
    responses(
        (status = 200, description = "List questions", body = [Question])
    )
)]
pub async fn questions(State(appstate): HandlerAppState) -> Response {
    let questions: Result<Vec<Question>, sqlx::Error> =
        appstate.read().await.store.get_questions(None, 1).await;
    match questions {
        Ok(questions) => Json(questions).into_response(),
        Err(e) => StoreError::response(StatusCode::INTERNAL_SERVER_ERROR, e.into()),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/question",
    responses(
        (status = 200, description = "Return random question", body = Question),
        (status = 204, description = "Store is empty", body = StoreError)
    )
)]

pub async fn question(State(appstate): HandlerAppState) -> Response {
    match appstate.read().await.store.get_random().await {
        Ok(question) => question.into_response(),
        Err(e) => StoreError::response(StatusCode::NO_CONTENT, e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/question/{id}",
    responses(
        (status = 200, description = "Return specified question", body = Question),
        (status = 204, description = "No question with this id", body = StoreError),
    )
)]
pub async fn get_question(
    State(appstate): HandlerAppState,
    Path(question_id): Path<String>,
) -> Response {
    match appstate.read().await.store.get(&question_id).await {
        Ok(question) => question.into_response(),
        Err(e) => StoreError::response(StatusCode::NO_CONTENT, e),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/question/add",
    request_body(
        content = inline(Question),
        description = "Question to add"
    ),
    responses(
        (status = 201, description = "Added question", body = ()),
        (status = 400, description = "Bad request", body = StoreError)
    )
)]
pub async fn post_question(
    _claims: Claims,
    State(appstate): HandlerAppState,
    Json(question): Json<Question>,
) -> Response {
    match appstate.write().await.store.add(question).await {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(e) => StoreError::response(StatusCode::BAD_REQUEST, e),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/question/{id}",
    responses(
        (status = 200, description = "Deleted question", body = ()),
        (status = 400, description = "Bad request", body = StoreError),
    )
)]
pub async fn delete_question(
    _claims: Claims,
    State(appstate): HandlerAppState,
    Path(question_id): Path<String>,
) -> Response {
    match appstate.write().await.store.delete(&question_id).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => StoreError::response(StatusCode::BAD_REQUEST, e),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/question/{id}",
    request_body(
        content = inline(Question),
        description = "Question to update"
    ),
    responses(
        (status = 200, description = "Updated question", body = ()),
        (status = 400, description = "Bad request", body = StoreError),
        (status = 404, description = "Question not found", body = StoreError),
        (status = 422, description = "Unprocessable entity", body = StoreError),
    )
)]
pub async fn update_question(
    _claims: Claims,
    State(appstate): HandlerAppState,
    Path(question_id): Path<String>,
    Json(question): Json<Question>,
) -> Response {
    match appstate
        .write()
        .await
        .store
        .update(&question_id, question)
        .await
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(StoreErr::DatabaseQueryError(e)) => StoreError::response(
            StatusCode::UNPROCESSABLE_ENTITY,
            StoreErr::DatabaseQueryError(e),
        ),
        Err(StoreErr::QuestionNotFound(_)) => StoreError::response(
            StatusCode::NOT_FOUND,
            StoreErr::QuestionNotFound(question_id.clone()),
        ),
        Err(e) => StoreError::response(StatusCode::BAD_REQUEST, e),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body(
        content = inline(Registration),
        description = "Get an API key"
    ),
    responses(
        (status = 200, description = "JSON Web Token", body = AuthBody),
        (status = 401, description = "Registration failed", body = AuthError),
    )
)]
pub async fn register(
    State(appstate): HandlerAppState,
    Json(registration): Json<Registration>,
) -> Response {
    let appstate = appstate.read().await;
    match make_jwt_token(&appstate, &registration) {
        Err(e) => e.into_response(),
        Ok(token) => (StatusCode::OK, token).into_response(),
    }
}
