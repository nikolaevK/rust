use actix_web:: {
    http:: {
        header::ContentType, StatusCode
    },
    HttpResponse, ResponseError
};

use derive_more::Display;
#[derive(Debug, Display)]
pub enum TodoError {
    NoTodosFound,
    TodoCreationFailure,
    NoSuchTodoFound,
}

impl ResponseError for TodoError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TodoError::NoTodosFound => StatusCode::NOT_FOUND,
            TodoError::TodoCreationFailure => StatusCode::BAD_REQUEST,
            TodoError::NoSuchTodoFound => StatusCode::NOT_FOUND
        }
    }
}