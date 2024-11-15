use actix_web::{
    delete,
    error,
    get,
    http::{header::ContentType, StatusCode},
    post,
    put,
    web::{Form, Json, Path},
    HttpResponse,
};
use derive_more::{Display, Error};
use crate::db::*;
use crate::model::*;
use crate::prelude::*;

#[derive(Debug, Display, Error)]
enum HttpError {
    #[display("bad request")]
    BadClientData,
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpError::BadClientData => StatusCode::BAD_REQUEST,
        }
    }
}

#[post("/todo")]
pub async fn create(params: Form<CreateForm>) -> actix_web::Result<Json<Task>, HttpError> {
    if params.description.is_empty() {
        Err(HttpError::BadClientData)
    } else {
        let todo = add_task(params.into_inner().description).await.unwrap();
        Ok(Json(todo))
    }
}

#[get("/todo/{id}")]
pub async fn get(id: Path<String>) -> Result<Json<Task>> {
    let task = get_task(id.into_inner()).await?;

    Ok(Json(task))
}

#[put("/todo/{id}")]
pub async fn update(id: Path<String>) -> Result<Json<Task>> {
    let updated = toggle_task(id.into_inner()).await?;

    Ok(Json(updated))
}

#[delete("/todo/{id}")]
pub async fn delete(id: Path<String>) -> Result<Json<AffectedRows>> {
    let deleted = delete_task(id.into_inner()).await?;

    Ok(Json(deleted))
}

#[get("/todos")]
pub async fn list() -> Result<Json<Vec<Task>>> {
    let todos = get_all_tasks().await?;

    Ok(Json(todos))
}