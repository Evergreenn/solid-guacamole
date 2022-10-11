#[macro_use]
extern crate dotenv_codegen;
extern crate derive_more;

use std::path::PathBuf;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::dev::ServiceRequest;
use actix_web::{http, web, App, Error, HttpRequest, HttpServer, Result};
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

async fn static_page(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./files/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
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
            .route("/pwa", web::get().to(static_page))
            .service(create_token)
            .service(login)
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .service(crate::routes::courses::get_courses)
                    .service(crate::routes::courses::add_course)
                    .service(crate::routes::courses::get_subscription)
                    .service(crate::routes::courses::update_course)
                    .service(crate::routes::students::course_registration)
                    .service(crate::routes::students::update_student)
                    .service(crate::routes::students::course_registration)
                    .service(crate::routes::students::me),
            )
    })
    .bind(dotenv!("API_URL"))?
    .workers(1)
    .run();
    println!("Server running at http://{}/", dotenv!("API_URL"));

    server.await
}
