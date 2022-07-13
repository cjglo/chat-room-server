use actix_web::{web, App, HttpRequest, HttpServer, Responder};



async fn welcome_message(req: HttpRequest) -> impl Responder {
    
    //let username = req.match_info().get("username");

    format!("Hello!")
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(welcome_message))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
