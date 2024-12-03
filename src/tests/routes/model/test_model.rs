use actix_http::body::MessageBody;

use crate::get_model_app;
use crate::models::person_models::PersonModels;
use crate::tests::routes::helpers::retrieve_token;
use crate::tests::routes::model::helpers::{test_model_api, PASSWORD, USERNAMES};

const URL: &'static str = "uri://my_redirect";

#[actix_web::test]
async fn test_upsert_model() {
    const USERNAME: &'static str = USERNAMES[0];
    let app = get_model_app!().await;
    let token = retrieve_token(USERNAME, PASSWORD).await;
    let req = test_model_api::post(&token, URL);
    let resp = actix_web::test::call_service(&app, req).await;
    let status = resp.status();
    let bytes = resp.into_body().try_into_bytes().unwrap();
    assert_eq!(status, actix_web::http::StatusCode::OK);
    /* let resp_body_as_str = std::str::from_utf8(&resp_body_as_bytes).unwrap();
    println!("resp_body_as_str = {:#?}", resp_body_as_str); */
    let model: PersonModels = serde_json::from_slice(&bytes).unwrap();
    let expect = PersonModels {
        id: model.id,
        url: Some(String::from(URL)),
        created_at: model.created_at,
        ..PersonModels::default()
    };
    assert_eq!(model, expect);
}

#[actix_web::test]
async fn test_read_model() {
    const USERNAME: &'static str = USERNAMES[1];
    let app = get_model_app!().await;
    let token = retrieve_token(USERNAME, PASSWORD).await;
    let req = test_model_api::post(&token, URL);
    let upserted_model: PersonModels = actix_web::test::call_and_read_body_json(&app, req).await;

    let req = test_model_api::get(&token, upserted_model.id);
    let read_model: PersonModels = actix_web::test::call_and_read_body_json(&app, req).await;
    let expect = PersonModels {
        id: read_model.id,
        created_at: read_model.created_at,
        url: Some(String::from(URL)),
        ..PersonModels::default()
    };
    assert_eq!(upserted_model, read_model);
    assert_eq!(read_model, expect);
}
