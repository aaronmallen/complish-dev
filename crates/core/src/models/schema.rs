// @generated automatically by Diesel CLI.

diesel::table! {
    list_tasks (list_id, task_id) {
        list_id -> Text,
        task_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    lists (id) {
        id -> Text,
        name -> Text,
        directories -> Text,
        metadata -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        label -> Text,
        metadata -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    task_notes (id) {
        id -> Text,
        task_id -> Text,
        content -> Text,
        metadata -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    task_relationships (source_id, target_id, kind) {
        source_id -> Text,
        target_id -> Text,
        kind -> Text,
        metadata -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    task_tags (task_id, tag_id) {
        task_id -> Text,
        tag_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    task_work_logs (id) {
        id -> Text,
        task_id -> Text,
        note -> Nullable<Text>,
        source -> Nullable<Text>,
        metadata -> Text,
        started_at -> Timestamp,
        ended_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tasks (id) {
        id -> Text,
        sequence_id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        priority -> Text,
        workflow_status -> Text,
        estimation -> Nullable<Text>,
        resolution -> Nullable<Text>,
        metadata -> Text,
        due_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(list_tasks -> lists (list_id));
diesel::joinable!(list_tasks -> tasks (task_id));
diesel::joinable!(task_notes -> tasks (task_id));
diesel::joinable!(task_tags -> tags (tag_id));
diesel::joinable!(task_tags -> tasks (task_id));
diesel::joinable!(task_work_logs -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
  list_tasks,
  lists,
  tags,
  task_notes,
  task_relationships,
  task_tags,
  task_work_logs,
  tasks,
);
