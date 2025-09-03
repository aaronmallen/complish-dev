CREATE TABLE project_updates (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  description TEXT,
  status TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  FOREIGN KEY (project_id) REFERENCES projects (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_project_updates_project_id ON project_updates (project_id);
CREATE INDEX idx_project_updates_status ON project_updates (status);
CREATE INDEX idx_project_updates_created_at ON project_updates (created_at);
