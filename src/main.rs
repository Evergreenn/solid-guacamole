#[macro_use]
extern crate dotenv_codegen;
extern crate derive_more;

use actix_cors::Cors;
use actix_web::dev::ServiceRequest;
use actix_web::{http, web, App, Error, HttpServer};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

mod claim;
mod repositories;
mod routes;
mod security;

use routes::students::*;

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match claim::decode_jwt(credentials.token()) {
        Ok(o) => {
            req.attach(o.permissions);
            Ok(req)
        }
        Err(e) => Err((e, req)), // Err(e) => Err(CustomError::from(String::from("Invalid authentication ")))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").unwrap();
    repositories::bootstrap::init();

    let server = HttpServer::new(|| {
        let auth = HttpAuthentication::bearer(validator);

        dbg!("booting server");

        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(create_token)
            .service(login)
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .service(crate::routes::courses::get_courses)
                    .service(crate::routes::courses::add_course)
            )
    })
    .bind(dotenv!("API_URL"))?
    .workers(1)
    .run();
    println!("Server running at http://{}/", dotenv!("API_URL"));

    server.await
}
