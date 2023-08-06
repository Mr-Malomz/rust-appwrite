use actix_web::{App, HttpServer};
use api::handlers::{create_project, delete_project, get_project, update_project};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(create_project)
            .service(get_project)
            .service(update_project)
            .service(delete_project)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
