use actix_web::{get, web, App, HttpServer, Responder, Result, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use env_logger::Env;
use actix_files::Files;
use std::path::PathBuf;

#[get("/")]
async fn login() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html;charset=utf-8")
        .body(include_str!("../login/index.html")))
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = "0.0.0.0";
    let port = 8080;
    println!("server is running on -> http://{server}:{port}");    
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(login).service(Files::new("/", PathBuf::from("./login")).index_file("index.html").show_files_listing())
            .wrap(Logger::default())
            .wrap(Logger::new("%t %a %r %s %Dms"))
    })
        .bind((server, port))?
        .run()
        .await
}