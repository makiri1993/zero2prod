use std::net::TcpListener;

use actix_web::{dev, web, App, HttpRequest, HttpServer, Responder};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check::health_check, subscriptions::subscribe};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<dev::Server, std::io::Error> {
    let connection = web::Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
