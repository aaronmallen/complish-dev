// @generated automatically by Diesel CLI.

diesel::table! {
    tags (id) {
        id -> Text,
        label -> Text,
        metadata -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
