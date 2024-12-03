pub(crate) const USERNAMES: [&'static str; 2] = ["username2", "username3"];
pub(crate) const PASSWORD: &'static str = "password";

#[macro_export]
macro_rules! get_model_app {
    () => {
        actix_web::test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(
                    rust_actix_diesel_auth_scaffold::POOL.clone(),
                ))
                .service(
                    actix_web::web::scope("/api/v0")
                        .wrap(actix_web::middleware::Compat::new(
                            actix_web_httpauth::middleware::HttpAuthentication::bearer(
                                rust_actix_diesel_auth_scaffold::middleware::bearer::validator,
                            ),
                        ))
                        .service(crate::routes::model::upsert)
                        .service(crate::routes::model::read)
                        .service(crate::routes::model::read_many),
                ),
        )
    };
}

pub(crate) mod test_model_api {
    pub(crate) fn post(token: &str, url: &'static str) -> actix_http::Request {
        actix_web::test::TestRequest::post()
            .uri("/api/v0/model")
            .append_header(("Authorization", format!("Bearer {}", token)))
            .set_json(crate::models::person_models::UpdatePersonModels {
                url: Some(Some(String::from(url))),
                ..crate::models::person_models::UpdatePersonModels::default()
            })
            .to_request()
    }

    pub(crate) fn get(token: &str, id: i32) -> actix_http::Request {
        actix_web::test::TestRequest::get()
            .uri(&format!("/api/v0/model/{id}"))
            .append_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    }
}
