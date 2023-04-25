use actix_web::{get, web, App, HttpServer, HttpRequest, Responder, Result};
use actix_files::NamedFile;
use serde::Serialize;
use actix_web_lab::web::spa;
use std::path::PathBuf;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "../front-end/build/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(greet)
        .service(
            spa()
                .index_file("../front-end/build/index.html")
                .static_resources_mount("/static")
                .static_resources_location("../front-end/build/static/")
                .finish(),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
