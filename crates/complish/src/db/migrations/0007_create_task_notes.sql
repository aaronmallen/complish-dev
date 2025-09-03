CREATE TABLE task_notes (
  id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  FOREIGN KEY (task_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_task_notes_task_id ON task_notes (task_id);
CREATE INDEX idx_task_notes_created_at ON task_notes (created_at);
