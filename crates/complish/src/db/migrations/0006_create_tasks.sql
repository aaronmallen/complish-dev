CREATE TABLE tasks (
  id TEXT PRIMARY KEY,
  sequence_id INTEGER,
  external_id TEXT,
  project_id TEXT,
  title TEXT NOT NULL,
  description TEXT,
  priority TEXT NOT NULL,
  workflow_status TEXT NOT NULL,
  estimation TEXT,
  resolution TEXT,
  due_at TIMESTAMP,
  completed_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  FOREIGN KEY (project_id) REFERENCES projects (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE TRIGGER auto_sequence_task
  AFTER INSERT ON tasks
  FOR EACH ROW
  WHEN NEW.sequence_id IS NULL
BEGIN
  UPDATE tasks
  SET sequence_id = (
    SELECT COALESCE(MAX(sequence_id), 0) + 1
    FROM tasks
    WHERE id != NEW.id
  )
  WHERE id = NEW.id;
END;

CREATE INDEX idx_tasks_sequence_id ON tasks (sequence_id);
CREATE INDEX idx_tasks_external_id ON tasks (external_id);
CREATE INDEX idx_tasks_project_id ON tasks (project_id);
CREATE INDEX idx_tasks_priority ON tasks (priority);
CREATE INDEX idx_tasks_workflow_status ON tasks (workflow_status);
CREATE INDEX idx_tasks_due_at ON tasks (due_at);
CREATE INDEX idx_tasks_completed_at ON tasks (completed_at);
CREATE INDEX idx_tasks_created_at ON tasks (created_at);
