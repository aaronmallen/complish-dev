CREATE TABLE projects (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  key TEXT NOT NULL,
  description TEXT,
  workflow_status TEXT NOT NULL,
  resolution TEXT,
  updates TEXT NOT NULL DEFAULT '[]',
  color TEXT NOT NULL,
  completed_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z')
);

CREATE INDEX idx_projects_key ON projects (key);
CREATE INDEX idx_projects_workflow_status ON projects (workflow_status);
CREATE INDEX idx_projects_resolution ON projects (resolution);
CREATE INDEX idx_projects_completed_at ON projects (completed_at);
