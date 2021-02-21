use actix_web::Result;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;

pub async fn check() -> Result<HttpResponse> {
  Ok(HttpResponse::build(StatusCode::OK).body("OK"))
}
