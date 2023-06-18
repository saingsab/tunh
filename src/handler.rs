use crate::{
    model::{AppState, QueryOption, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
};
use actix_web::{delete, get, patch, post, web::{self, Json}, HttpResponse, Responder, error};
use chrono::prelude::*;
use uuid::Uuid;

#[get("/healthcheker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/todos")]
pub async fn todo_list_handler(
    opts: web::Query<QueryOption>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todos = data.todo_db.lock().unwrap();

    let limit = opts.limit.unwrap_or(1);
    let offset = (opts.page.unwrap_or(1) -1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        result: todos.len(),
        todos
    };
    HttpResponse::Ok().json(json_response)
}

#[get("/todos/{id}")]
async fn get_todo_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let vec = data.todo_db.lock().unwrap();

    let id = path.into_inner();
    let todo = vec.iter().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with ID: {} not found", id),
        };
        return HttpResponse::NotFound().json(error_response);
    }
    let todo = todo.unwrap();
    let json_response = SingleTodoResponse {
        status: "sucess".to_string(),
        data: TodoData { todo: todo.clone() },
    };

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(todo_list_handler)
        .service(get_todo_handler);

    conf.service(scope);
}