/*use strum_macros;

#[derive(diesel::AsExpression, diesel::FromSqlRow, strum::EnumString, PartialEq, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
#[sql_type = "crate::schema::models::skin_undertone"]
pub enum SkinUndertone {
    Cool = 0,
    Warm = 1,
    Neutral,
    Olive,
}

#[derive(diesel::AsExpression, diesel::FromSqlRow, strum::EnumString, PartialEq, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
#[sql_type = "crate::schema::models::skin_colour"]
pub enum SkinColour {
    Russet = 0,
    Peru = 1,
    Fawn,
    MellowApricot,
    NavajoWhite,
}*/
/*
#[derive(diesel_derive_enum::DbEnum, PartialEq, Debug)]
//#[ExistingTypePath = "crate::schema::models::hair_colour"]
#[DbValueStyle = "snake_case"]
pub enum HairColourE {
    Bald = 0,
    Black = 1,
    Blond,
    Brown,
    Gray,
    White,
    Auburn,
    Other, // = -1,
}
*/
use crate::models::profiles::{CreateProfiles, UpdateProfiles};

impl From<CreateProfiles> for UpdateProfiles {
    fn from(profile: CreateProfiles) -> Self {
        Self {
            username: Some(profile.username),
            profile_image_url: Some(profile.profile_image_url),
            male: Some(profile.male),
            height_mm: Some(profile.height_mm),
            weight_g: Some(profile.weight_g),
            bust_mm: Some(profile.bust_mm),
            waist_mm: Some(profile.waist_mm),
            hip_mm: Some(profile.hip_mm),
            skin_undertone: Some(profile.skin_undertone),
            skin_colour: Some(profile.skin_colour),
            hair_colour: Some(profile.hair_colour),
            created_at: None,
        }
    }
}
