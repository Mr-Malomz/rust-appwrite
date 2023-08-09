use super::{
    model::{APIErrorResponse, APIResponse, Project, ProjectRequest, ProjectResponse},
    services::AppwriteService,
};
use actix_web::{
    delete, get, patch, post,
    web::{Json, Path},
    HttpResponse,
};
use reqwest::StatusCode;

#[post("/project")]
pub async fn create_project(data: Json<ProjectRequest>) -> HttpResponse {
    let new_project = ProjectRequest {
        name: data.name.clone(),
        description: data.description.clone(),
    };

    let project_details = AppwriteService::create_project(new_project).await;

    match project_details {
        Ok(data) => HttpResponse::Accepted().json(APIResponse::<ProjectResponse> {
            status: StatusCode::CREATED.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}

#[get("/project/{id}")]
pub async fn get_project(path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(APIErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: "failure".to_string(),
            data: Some("invalid ID".to_string()),
        });
    };

    let project_details = AppwriteService::get_project(id).await;

    match project_details {
        Ok(data) => HttpResponse::Accepted().json(APIResponse::<Project> {
            status: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}

#[patch("/project/{id}")]
pub async fn update_project(
    updated_project: Json<ProjectRequest>,
    path: Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(APIErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: "failure".to_string(),
            data: Some("invalid ID".to_string()),
        });
    };

    let data = ProjectRequest {
        name: updated_project.name.clone(),
        description: updated_project.description.clone(),
    };

    let project_details = AppwriteService::update_project(data, id).await;

    match project_details {
        Ok(data) => HttpResponse::Accepted().json(APIResponse::<ProjectResponse> {
            status: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}

#[delete("/project/{id}")]
pub async fn delete_project(path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(APIErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: "failure".to_string(),
            data: Some("invalid ID".to_string()),
        });
    };
    let project_details = AppwriteService::delete_project(id).await;

    match project_details {
        Ok(data) => HttpResponse::Accepted().json(APIResponse::<String> {
            status: StatusCode::ACCEPTED.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}
