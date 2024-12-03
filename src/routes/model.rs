use actix_web::{get, post};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use rust_actix_diesel_auth_scaffold::errors::AuthError;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::person_models::{CreatePersonModels, PersonModels};
use crate::schema::person_models::dsl::person_models;

#[derive(serde::Deserialize, serde::Serialize)]
struct ModelsVecObj {
    models: Vec<PersonModels>,
}

/// Get Model by id
#[utoipa::path(
    responses(
        (status = 200, description = "Model found from database"),
        (status = 404, description = "Not found")
    ),
    params(
        ("id", description = "Model id"),
    )
)]
#[get("/model/{id}")]
pub async fn read(
    pool: actix_web::web::Data<DbPool>,
    id: actix_web::web::Path<i32>,
) -> Result<actix_web::web::Json<PersonModels>, AuthError> {
    let mut conn = pool.get()?;
    Ok(actix_web::web::Json(
        person_models.find(id.into_inner()).first(&mut conn)?,
    ))
}

/// Get PersonModels
#[utoipa::path(
    responses(
        (status = 200, description = "PersonModels found in database"),
        (status = 404, description = "Not found")
    )
)]
#[get("/models")]
pub async fn read_many(
    pool: actix_web::web::Data<DbPool>,
) -> Result<actix_web::web::Json<ModelsVecObj>, AuthError> {
    let mut conn = pool.get()?;

    let models_vec: Vec<PersonModels> = person_models
        .select(PersonModels::as_select())
        .load(&mut conn)?;

    Ok(actix_web::web::Json(ModelsVecObj { models: models_vec }))
}

/// Upsert Model
#[utoipa::path(
    responses(
        (status = 200, description = "Model created"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("password"=[]))
)]
#[post("/model")]
pub async fn upsert(
    pool: actix_web::web::Data<DbPool>,
    form: actix_web::web::Json<CreatePersonModels>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<PersonModels>, AuthError> {
    let mut conn = pool.get()?;

    // 0. check token username vs username in request
    if let Some((_username_s, _)) = credentials.token().split_once(":") {
        // 1. upsert model
        let new_model_vals = form.into_inner();
        let model = diesel::insert_into(person_models)
            .values(&new_model_vals)
            .returning(PersonModels::as_returning())
            .get_result(&mut conn)?;
        return Ok(actix_web::web::Json(model));
    }

    Err(AuthError::HttpError(500))
}
