use std::{io::Read};

use actix_web::{
    get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 4888;
    println!("正在啟動dummy-api...");
    println!("正在監聽port：{port}");
    HttpServer::new(|| {
        App::new()
            .service(response_json)
            
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

#[get("/{path}")]
async fn response_json(path: web::Path<String>) -> impl Responder {
    println!("正在讀取：{path}");
    let json = read_json(path.to_string());
    println!("返回內容：\n{json}");

    HttpResponse::Ok().content_type(ContentType::json()).body(
        json
    )
}