use actix_web::{App, HttpServer};
use api::handlers::{
    create_project_handler, delete_project_handler, get_project_handler, update_project_handler,
};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(create_project_handler)
            .service(get_project_handler)
            .service(update_project_handler)
            .service(delete_project_handler)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
