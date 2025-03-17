use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use actix_files::NamedFile;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[get("/login")]
async fn login() -> impl Responder {
    NamedFile::open("login/index.html")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = "0.0.0.0";
    let port = 8080;
    println!("server is running on -> http://{server}:{port}");    
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(login)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
        .bind((server, port))?
        .run()
        .await
}