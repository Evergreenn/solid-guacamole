#[macro_use]
extern crate dotenv_codegen;

extern crate derive_more;

use actix_web::dev::ServiceRequest;
use actix_web::{http, web, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_cors::Cors;
use actix_web_grants::permissions::AttachPermissions;

mod claim;
mod security;
mod routes;
mod repositories;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
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

    dotenv::from_filename(".env.local").unwrap();
    // auth_repository::manage::init();

    // let url = "mysql://root:root@localhost:3306/titan";

    // let pool = Pool::new(url).unwrap();
    // let mut conn = pool.get_conn().unwrap();
    // embedded::migrations::runner().run(&mut conn).unwrap();


    HttpServer::new(|| {
        let auth = HttpAuthentication::bearer(validator);

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
                    .service(crate::routes::post::create_community_file)
                   
            )
    })
    .bind(dotenv!("API_URL"))?
    .workers(1)
    .run()
    .await
}