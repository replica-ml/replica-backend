use actix_web::{get, post};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use rust_actix_diesel_auth_scaffold::errors::AuthError;
use rust_actix_diesel_auth_scaffold::DbPool;

use crate::models::profiles::{CreateProfiles, Profiles, UpdateProfiles};
use crate::schema::profiles::dsl::profiles;
use crate::schema::profiles::{alias, username};

/// Get profile
#[utoipa::path(
    responses(
        (status = 200, description = "Profile for user associated with access token"),
        (status = 404, description = "Not found: User does not have associated profile")
    ),
    security(("password"=[]))
)]
#[get("/profile")]
pub async fn read(
    pool: actix_web::web::Data<DbPool>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<Profiles>, AuthError> {
    let mut conn = pool.get()?;

    // 0. check token username vs username in request
    if let Some((username_s, _)) = credentials.token().split_once(":") {
        use diesel::ExpressionMethods;
        let profile = profiles.filter(username.eq(username_s)).first(&mut conn)?;

        return Ok(actix_web::web::Json(profile));
    }
    Err(AuthError::NotFound("User does not have associated profile"))
}

/// Upsert Profile
#[utoipa::path(
    responses(
        (status = 200, description = "Profile created"),
        (status = 401, description = "Unauthorised: You tried to create a profile for another user")
    ),
    security(("password"=[]))
)]
#[post("/profile")]
pub async fn upsert(
    pool: actix_web::web::Data<DbPool>,
    form: actix_web::web::Json<CreateProfiles>,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
) -> Result<actix_web::web::Json<Profiles>, AuthError> {
    let mut conn = pool.get()?;

    // 0. check token username vs username in request
    if let Some((username_s, _)) = credentials.token().split_once(":") {
        if form.username != String::from(username_s) {
            return Err(AuthError::Unauthorised(
                "You tried to create a profile for another user",
            ));
        }
    }
    // 1. upsert profile
    // let inner = form.into_inner();
    let new_profile_vals = form.into_inner();
    // let new_profile_vals = CreateProfiles::from(&inner);
    log::info!("new_profile_vals = {:?}", &new_profile_vals);
    let profile = diesel::insert_into(profiles)
        .values(&new_profile_vals.clone())
        .on_conflict((alias, username))
        .do_update()
        .set(&UpdateProfiles::from(new_profile_vals))
        .returning(Profiles::as_returning())
        .get_result(&mut conn)?;

    Ok(actix_web::web::Json(profile))
}
