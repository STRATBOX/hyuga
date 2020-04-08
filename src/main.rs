use actix_web::{web, middleware, App, Error, HttpRequest, HttpResponse, HttpServer,  Responder};
use chrono::Utc;
use dotenv::dotenv;
use futures::future::{ready, Ready};
use serde::{Serialize};
use listenfd::ListenFd;
use ulid::Ulid;
use std::env;
use log::{info};

mod user;

struct AppState {
    app_name: &'static str
}

#[derive(Serialize)]
struct Ping {
    id: String,
    service: String,
    message: String,
    created_on: i64
}

impl Responder for Ping {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
        ))
    }
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let p: Ping = Ping {
        id: Ulid::new().to_string().to_lowercase(),
        service: data.app_name.to_string(),
        message: String::from("running..."),
        created_on: Utc::now().timestamp_nanos()
    };
    p
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", name)
}

#[actix_rt::main]   
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(AppState {
                app_name: "hyuga"
            })
            .service(web::scope("/users").configure(user::endpoints))
            .route("/", web::get().to(index))
            .route("/{name}", web::get().to(greet))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set!");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server...");
    server.run().await
}