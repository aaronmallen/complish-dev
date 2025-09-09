CREATE TABLE lists (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  directories TEXT NOT NULL DEFAULT '[]',
  metadata TEXT NOT NULL DEFAULT '{}',
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z')
);

CREATE INDEX idx_lists_name ON lists (name);

CREATE TABLE list_tasks (
  list_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  PRIMARY KEY (list_id, task_id),
  FOREIGN KEY (list_id) REFERENCES lists (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_list_tasks_list_id ON list_tasks (list_id);
CREATE INDEX idx_list_tasks_task_id ON list_tasks (task_id);
