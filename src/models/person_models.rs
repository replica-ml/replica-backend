/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::pg::PgConnection;

/// Struct representing a row in table `person_models`
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::QueryableByName,
    PartialEq,
    diesel::Identifiable,
)]
#[diesel(table_name=person_models, primary_key(id))]
pub struct PersonModels {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `url`
    pub url: Option<String>,
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

/// Create Struct for a row in table `person_models` for [`PersonModels`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable, utoipa::ToSchema,
)]
#[diesel(table_name=person_models)]
pub struct CreatePersonModels {
    /// Field representing column `url`
    pub url: Option<String>,
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

impl Default for CreatePersonModels {
    fn default() -> Self {
        Self {
            url: None,
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

/// Update Struct for a row in table `person_models` for [`PersonModels`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=person_models)]
pub struct UpdatePersonModels {
    /// Field representing column `url`
    pub url: Option<Option<String>>,
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

impl PersonModels {
    /// Insert a new row into `person_models` with a given [`CreatePersonModels`]
    pub fn create(db: &mut ConnectionType, item: &CreatePersonModels) -> diesel::QueryResult<Self> {
        use crate::schema::person_models::dsl::*;

        diesel::insert_into(person_models)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `person_models`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::person_models::dsl::*;

        person_models.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `person_models`, identified by the primary key with [`UpdatePersonModels`]
    pub fn update(
        db: &mut ConnectionType,
        param_id: i32,
        item: &UpdatePersonModels,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::person_models::dsl::*;

        diesel::update(person_models.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `person_models`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::person_models::dsl::*;

        diesel::delete(person_models.filter(id.eq(param_id))).execute(db)
    }
}

impl Default for PersonModels {
    fn default() -> Self {
        Self {
            id: 0,
            url: None,
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
