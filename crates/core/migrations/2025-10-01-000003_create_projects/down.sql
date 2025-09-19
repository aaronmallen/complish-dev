DROP INDEX IF EXISTS idx_project_updates_created_at;
DROP INDEX IF EXISTS idx_project_updates_status;
DROP INDEX IF EXISTS idx_project_updates_project_id;
DROP TABLE IF EXISTS project_updates;

DROP INDEX IF EXISTS idx_project_tasks_task_id;
DROP INDEX IF EXISTS idx_project_tasks_project_id;
DROP TABLE IF EXISTS project_tasks;

DROP INDEX IF EXISTS idx_projects_completed_at;
DROP INDEX IF EXISTS idx_projects_workflow_status;
DROP INDEX IF EXISTS udx_projects_key;
DROP TABLE IF EXISTS projects;
