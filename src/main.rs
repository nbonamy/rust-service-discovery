use actix_files as fs;
use actix_web::web;
use actix_web::middleware;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::web::resource;

mod database;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	// init logging
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	HttpServer::new(|| {
		App::new()
			.wrap(middleware::Logger::default())
			.service(resource("/healthcheck").route(web::get().to(handlers::healthcheck::check)))
			.service(resource("/list").route(web::get().to(handlers::webservice::list)))
			.service(resource("/update").route(web::post().to(handlers::webservice::update)))
			.service(fs::Files::new("/", "pub").show_files_listing())
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
