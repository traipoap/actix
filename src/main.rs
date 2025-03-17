use actix_web::{get, App, HttpServer, Result, error};
use actix_files::NamedFile;
use actix_web::HttpRequest;
use std::path::Path;
use actix_web::middleware::Logger;
use env_logger::Env;

// ประกาศตัวแปรสำหรับ base path
const STATIC_DIR: &str = "./login";


#[get("/{path:.*}")]
async fn login(req: HttpRequest) -> Result<NamedFile> {
    // ดึงค่า path ที่ร้องขอจาก URL
    let requested_path: String = req.match_info().query("path").parse().unwrap_or_default();

    // สร้าง PathBuf จาก base path
    let base_path = Path::new(STATIC_DIR);

    // รวม base path กับ path ที่ร้องขอ
    let path = base_path.join(requested_path);

    // ตรวจสอบว่า path ที่ร้องขออยู่ใน base path จริงๆ
    if !path.starts_with(base_path) {
        return Err(error::ErrorForbidden("Access denied"));
    }

    // ตรวจสอบว่าไฟล์มีอยู่จริง
    if !path.exists() {
        return Err(error::ErrorNotFound("File not found"));
    }

    // เปิดไฟล์และส่งกลับเป็น response
    NamedFile::open(path).map_err(|_| error::ErrorInternalServerError("Failed to open file"))
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