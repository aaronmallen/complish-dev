// @generated automatically by Diesel CLI.

diesel::table! {
    accomplishments (id) {
        id -> Text,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
