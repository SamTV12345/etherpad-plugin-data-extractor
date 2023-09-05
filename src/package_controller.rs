use actix_web::{get, HttpResponse};

#[get("/packages")]
pub async fn get_packages() -> Result<HttpResponse, actix_web::Error>{

    return Ok(HttpResponse::Ok().body("Hello, world!"));
}