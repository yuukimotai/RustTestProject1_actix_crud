//172.17.0.3 mysql
use actix_web::{ App, HttpServer };

use actix_crud::routes;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {App::new()
        .configure(routes::routes)
    })
    .bind(("172.17.0.3", 8080))?
    .run()
    .await
}
