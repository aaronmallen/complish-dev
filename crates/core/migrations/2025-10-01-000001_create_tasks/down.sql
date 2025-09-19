DROP INDEX IF EXISTS idx_task_tags_tag_id;
DROP INDEX IF EXISTS idx_task_tags_task_id;
DROP TABLE IF EXISTS task_tags;

DROP INDEX IF EXISTS idx_task_relationships_kind;
DROP INDEX IF EXISTS idx_task_relationships_target_id;
DROP INDEX IF EXISTS idx_task_relationships_source_id;
DROP TABLE IF EXISTS task_relationships;

DROP INDEX IF EXISTS idx_task_work_logs_ended_at;
DROP INDEX IF EXISTS idx_task_work_logs_started_at;
DROP INDEX IF EXISTS idx_task_work_logs_source;
DROP INDEX IF EXISTS idx_task_work_logs_task_id;
DROP TABLE IF EXISTS task_work_logs;

DROP INDEX IF EXISTS idx_task_notes_created_at;
DROP INDEX IF EXISTS idx_task_notes_task_id;
DROP TABLE IF EXISTS task_notes;

DROP TRIGGER IF EXISTS auto_sequence_task;
DROP INDEX IF EXISTS idx_tasks_created_at;
DROP INDEX IF EXISTS idx_tasks_completed_at;
DROP INDEX IF EXISTS idx_tasks_due_at;
DROP INDEX IF EXISTS idx_tasks_workflow_status;
DROP INDEX IF EXISTS idx_tasks_priority;
DROP INDEX IF EXISTS idx_tasks_external_id;
DROP INDEX IF EXISTS idx_tasks_sequence_id;
DROP TABLE IF EXISTS tasks;
