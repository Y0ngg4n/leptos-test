use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::pet_sitters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PetSitters {
    pub id: i32,
    pub name: String,
    pub capacity: i32,
    pub status: String,
    pub description: String,
    pub duration: i32,
}
