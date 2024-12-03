/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::users::Users;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `profiles`
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::QueryableByName,
    PartialEq,
    diesel::Associations,
    diesel::Identifiable,
)]
#[diesel(table_name=profiles, primary_key(alias), belongs_to(Users, foreign_key=username))]
pub struct Profiles {
    /// Field representing column `alias`
    pub alias: String,
    /// Field representing column `username`
    pub username: String,
    /// Field representing column `profile_image_url`
    pub profile_image_url: Option<String>,
    /// Field representing column `male`
    pub male: Option<bool>,
    /// Field representing column `height_mm`
    pub height_mm: Option<i32>,
    /// Field representing column `weight_g`
    pub weight_g: Option<i32>,
    /// Field representing column `bust_mm`
    pub bust_mm: Option<i32>,
    /// Field representing column `waist_mm`
    pub waist_mm: Option<i32>,
    /// Field representing column `hip_mm`
    pub hip_mm: Option<i32>,
    /// Field representing column `skin_undertone`
    pub skin_undertone: Option<String>,
    /// Field representing column `skin_colour`
    pub skin_colour: Option<String>,
    /// Field representing column `hair_colour`
    pub hair_colour: Option<String>,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `profiles` for [`Profiles`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable, utoipa::ToSchema,
)]
#[diesel(table_name=profiles)]
pub struct CreateProfiles {
    /// Field representing column `alias`
    pub alias: String,
    /// Field representing column `username`
    pub username: String,
    /// Field representing column `profile_image_url`
    pub profile_image_url: Option<String>,
    /// Field representing column `male`
    pub male: Option<bool>,
    /// Field representing column `height_mm`
    pub height_mm: Option<i32>,
    /// Field representing column `weight_g`
    pub weight_g: Option<i32>,
    /// Field representing column `bust_mm`
    pub bust_mm: Option<i32>,
    /// Field representing column `waist_mm`
    pub waist_mm: Option<i32>,
    /// Field representing column `hip_mm`
    pub hip_mm: Option<i32>,
    /// Field representing column `skin_undertone`
    pub skin_undertone: Option<String>,
    /// Field representing column `skin_colour`
    pub skin_colour: Option<String>,
    /// Field representing column `hair_colour`
    pub hair_colour: Option<String>,
}

impl Default for CreateProfiles {
    fn default() -> Self {
        Self {
            alias: String::new(),
            username: String::new(),
            profile_image_url: None,
            male: None,
            height_mm: None,
            weight_g: None,
            bust_mm: None,
            waist_mm: None,
            hip_mm: None,
            skin_undertone: None,
            skin_colour: None,
            hair_colour: None,
        }
    }
}

/// Update Struct for a row in table `profiles` for [`Profiles`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=profiles)]
pub struct UpdateProfiles {
    /// Field representing column `username`
    pub username: Option<String>,
    /// Field representing column `profile_image_url`
    pub profile_image_url: Option<Option<String>>,
    /// Field representing column `male`
    pub male: Option<Option<bool>>,
    /// Field representing column `height_mm`
    pub height_mm: Option<Option<i32>>,
    /// Field representing column `weight_g`
    pub weight_g: Option<Option<i32>>,
    /// Field representing column `bust_mm`
    pub bust_mm: Option<Option<i32>>,
    /// Field representing column `waist_mm`
    pub waist_mm: Option<Option<i32>>,
    /// Field representing column `hip_mm`
    pub hip_mm: Option<Option<i32>>,
    /// Field representing column `skin_undertone`
    pub skin_undertone: Option<Option<String>>,
    /// Field representing column `skin_colour`
    pub skin_colour: Option<Option<String>>,
    /// Field representing column `hair_colour`
    pub hair_colour: Option<Option<String>>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Result of a `.paginate` function
#[derive(Debug, serde::Serialize)]
pub struct PaginationResult<T> {
    /// Resulting items that are from the current page
    pub items: Vec<T>,
    /// The count of total items there are
    pub total_items: i64,
    /// Current page, 0-based index
    pub page: i64,
    /// Size of a page
    pub page_size: i64,
    /// Number of total possible pages, given the `page_size` and `total_items`
    pub num_pages: i64,
}

impl Profiles {
    /// Insert a new row into `profiles` with a given [`CreateProfiles`]
    pub fn create(db: &mut ConnectionType, item: &CreateProfiles) -> diesel::QueryResult<Self> {
        use crate::schema::profiles::dsl::*;

        diesel::insert_into(profiles)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `profiles`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_alias: String) -> diesel::QueryResult<Self> {
        use crate::schema::profiles::dsl::*;

        profiles.filter(alias.eq(param_alias)).first::<Self>(db)
    }

    /// Update a row in `profiles`, identified by the primary key with [`UpdateProfiles`]
    pub fn update(
        db: &mut ConnectionType,
        param_alias: String,
        item: &UpdateProfiles,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::profiles::dsl::*;

        diesel::update(profiles.filter(alias.eq(param_alias)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `profiles`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_alias: String) -> diesel::QueryResult<usize> {
        use crate::schema::profiles::dsl::*;

        diesel::delete(profiles.filter(alias.eq(param_alias))).execute(db)
    }
}

impl Default for Profiles {
    fn default() -> Self {
        Self {
            alias: String::new(),
            username: String::new(),
            profile_image_url: None,
            male: None,
            height_mm: None,
            weight_g: None,
            bust_mm: None,
            waist_mm: None,
            hip_mm: None,
            skin_undertone: None,
            skin_colour: None,
            hair_colour: None,
            created_at: Default::default(),
        }
    }
}
