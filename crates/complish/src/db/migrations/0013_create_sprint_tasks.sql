CREATE TABLE sprint_tasks (
  sprint_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  PRIMARY KEY (sprint_id, task_id),
  FOREIGN KEY (sprint_id) REFERENCES sprints (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_sprint_tasks_sprint_id ON sprint_tasks (sprint_id);
CREATE INDEX idx_sprint_tasks_task_id ON sprint_tasks (task_id);
