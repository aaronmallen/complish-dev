CREATE TABLE tasks (
  id TEXT PRIMARY KEY NOT NULL,
  sequence_id INTEGER,
  external_id TEXT,
  title TEXT NOT NULL,
  description TEXT,
  priority TEXT NOT NULL,
  workflow_status TEXT NOT NULL,
  estimation TEXT,
  resolution TEXT,
  metadata TEXT NOT NULL DEFAULT '{}',
  due_at TIMESTAMP,
  completed_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z')
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
CREATE INDEX idx_tasks_priority ON tasks (priority);
CREATE INDEX idx_tasks_workflow_status ON tasks (workflow_status);
CREATE INDEX idx_tasks_due_at ON tasks (due_at);
CREATE INDEX idx_tasks_completed_at ON tasks (completed_at);
CREATE INDEX idx_tasks_created_at ON tasks (created_at);

CREATE TABLE task_notes (
  id TEXT PRIMARY KEY NOT NULL,
  task_id TEXT NOT NULL,
  content TEXT NOT NULL,
  metadata TEXT NOT NULL DEFAULT '{}',
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  FOREIGN KEY (task_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_task_notes_task_id ON task_notes (task_id);
CREATE INDEX idx_task_notes_created_at ON task_notes (created_at);

CREATE TABLE task_work_logs (
  id TEXT PRIMARY KEY NOT NULL,
  task_id TEXT NOT NULL,
  note TEXT,
  source TEXT,
  metadata TEXT NOT NULL DEFAULT '{}',
  started_at TIMESTAMP NOT NULL,
  ended_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  FOREIGN KEY (task_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_task_work_logs_task_id ON task_work_logs (task_id);
CREATE INDEX idx_task_work_logs_source ON task_work_logs (source);
CREATE INDEX idx_task_work_logs_started_at ON task_work_logs (started_at);
CREATE INDEX idx_task_work_logs_ended_at ON task_work_logs (ended_at);

CREATE TABLE task_relationships (
  source_id TEXT NOT NULL,
  target_id TEXT NOT NULL,
  kind TEXT NOT NULL,
  metadata TEXT NOT NULL DEFAULT '{}',
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  PRIMARY KEY (source_id, target_id, kind),
  FOREIGN KEY (source_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  FOREIGN KEY (target_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_task_relationships_source_id ON task_relationships (source_id);
CREATE INDEX idx_task_relationships_target_id ON task_relationships (target_id);
CREATE INDEX idx_task_relationships_kind ON task_relationships (kind);

CREATE TABLE task_tags (
  task_id TEXT NOT NULL,
  tag_id TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  PRIMARY KEY (task_id, tag_id),
  FOREIGN KEY (task_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_task_tags_task_id ON task_tags (task_id);
CREATE INDEX idx_task_tags_tag_id ON task_tags (tag_id);

