use std::io::Read;

mod cli;
use cli::CommandLineArgs;

use actix_web::{
    get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder
};
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let CommandLineArgs {
        listening_port
    } = CommandLineArgs::from_args();

    let port = match listening_port {
        Some(port) => port,
        None => 4888,
    };

    println!("正在啟動dummy-api...");
    println!("正在監聽port：{port}");
    println!("http://127.0.0.1:{port}", );
    HttpServer::new(|| {
        App::new()
            .service(response_json)
            .route("/", web::get().to(index_content))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

fn read_json(file_path: String) -> String{
    
    let mut file = std::fs::File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return contents;
}

#[get("/api/{path}")]
async fn response_json(path: web::Path<String>) -> impl Responder {
    println!("正在讀取：{path}");
    let json = read_json(path.to_string());
    println!("返回內容：\n{json}");

    HttpResponse::Ok().content_type(ContentType::json()).body(
        json
    )
}

async fn index_content() -> impl Responder {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body("歡迎使用dummy-api")
}