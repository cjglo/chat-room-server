use actix_web::{web, App, HttpRequest, HttpServer, Responder};



async fn health_check(req: HttpRequest) -> impl Responder {
    
    //let username = req.match_info().get("username");

    format!("Healthy and Here!")
}


async fn use_code(req: HttpRequest) -> impl Responder {
    
    let id = req.match_info().get("id").unwrap_or("noId");
    let code = req.match_info().get("code").unwrap_or("noCode");

    /* let id = match id {
        Some(id) => id,
        None => return format!("No Valid Id")
    };
    let code = match code {
        Some(code) => code,
        None => return format!("No Doce Code!")
    };
    */

    format!("code was: {code}, and id was {id}")
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check))
            .route("/pur", web::get().to(use_code))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
