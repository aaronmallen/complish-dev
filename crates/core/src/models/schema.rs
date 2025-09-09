// @generated automatically by Diesel CLI.

diesel::table! {
    journal_entries (id) {
        id -> Text,
        content -> Text,
        metadata -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
    project_tasks (project_id, task_id) {
        project_id -> Text,
        task_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    project_updates (id) {
        id -> Text,
        project_id -> Text,
        description -> Nullable<Text>,
        status -> Text,
        metadata -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    projects (id) {
        id -> Text,
        key -> Text,
        name -> Text,
        description -> Nullable<Text>,
        workflow_status -> Text,
        resolution -> Nullable<Text>,
        directories -> Text,
        metadata -> Text,
        completed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sprint_tasks (sprint_id, task_id) {
        sprint_id -> Text,
        task_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sprints (id) {
        id -> Text,
        metadata -> Text,
        started_at -> Timestamp,
        ended_at -> Timestamp,
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
diesel::joinable!(project_tasks -> projects (project_id));
diesel::joinable!(project_tasks -> tasks (task_id));
diesel::joinable!(project_updates -> projects (project_id));
diesel::joinable!(sprint_tasks -> sprints (sprint_id));
diesel::joinable!(sprint_tasks -> tasks (task_id));
diesel::joinable!(task_notes -> tasks (task_id));
diesel::joinable!(task_tags -> tags (tag_id));
diesel::joinable!(task_tags -> tasks (task_id));
diesel::joinable!(task_work_logs -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
  journal_entries,
  list_tasks,
  lists,
  project_tasks,
  project_updates,
  projects,
  sprint_tasks,
  sprints,
  tags,
  task_notes,
  task_relationships,
  task_tags,
  task_work_logs,
  tasks,
);
