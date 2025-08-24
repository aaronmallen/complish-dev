CREATE TABLE tasks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  list_id INTEGER NOT NULL,
  subject TEXT NOT NULL,
  description TEXT,
  status TEXT NOT NULL,
  completed_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  FOREIGN KEY (list_id) REFERENCES lists(id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_tasks_list_id ON tasks (list_id);
CREATE INDEX idx_tasks_status ON tasks (status);
CREATE INDEX idx_tasks_completed_at ON tasks (completed_at);