use actix_web::{get, App, HttpServer};
use actix_files::NamedFile;
use actix_web::HttpRequest;
use std::path::PathBuf;
use actix_web::middleware::Logger;
use env_logger::Env;

#[get("/{path:.*}")]
async fn login(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("path").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = "0.0.0.0";
    let port = 8080;
    println!("server is running on -> http://{server}:{port}");    
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(login)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
        .bind((server, port))?
        .run()
        .await
}