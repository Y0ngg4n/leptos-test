// @generated automatically by Diesel CLI.

diesel::table! {
    pet_sitters (id) {
        id -> Integer,
        name -> Text,
        capacity -> Integer,
        status -> Text,
        description -> Text,
        duration -> Integer,
    }
}
