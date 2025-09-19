DROP INDEX IF EXISTS idx_sprint_tasks_task_id;
DROP INDEX IF EXISTS idx_sprint_tasks_sprint_id;
DROP TABLE IF EXISTS sprint_tasks;

DROP INDEX IF EXISTS idx_sprints_ended_at;
DROP INDEX IF EXISTS idx_sprints_started_at;
DROP TABLE IF EXISTS sprints;
