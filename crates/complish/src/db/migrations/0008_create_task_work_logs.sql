CREATE TABLE task_work_logs (
  id TEXT PRIMARY KEY,
  task_id TEXT NOT NULL,
  note TEXT,
  source TEXT,
  started_at TIMESTAMP NOT NULL,
  ended_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  FOREIGN KEY (task_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_task_work_logs_task_id ON task_work_logs (task_id);
CREATE INDEX idx_task_work_logs_started_at ON task_work_logs (started_at);
CREATE INDEX idx_task_work_logs_ended_at ON task_work_logs (ended_at);
