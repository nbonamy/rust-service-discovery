use actix_web::web;
use actix_web::error;
use actix_web::HttpResponse;
use actix_web::Error;
use super::database;

pub async fn list() -> Result<HttpResponse, Error> {
	let vec = database::list();
	Ok(
		HttpResponse::Ok()
			.content_type("application/json")
			.body(serde_json::to_string(&vec).unwrap())
	)
}

pub async fn update(body: web::Bytes) -> Result<HttpResponse, Error> {
	let jsonstr = std::str::from_utf8(&body).unwrap();
	match serde_json::from_str::<database::Service>(jsonstr) {
		Ok(service) => {
			let result = database::update(&service);
			Ok(
				HttpResponse::Ok()
					.content_type("application/json")
					.body(serde_json::to_string(&result).unwrap())
			)
		},
		Err(e) => {
			Err(error::ErrorBadRequest(format!("{{\"error\": \"{}\"}}", e)))
		}
	}
}
