CREATE TABLE projects (
  id TEXT PRIMARY KEY NOT NULL,
  key TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  workflow_status TEXT NOT NULL,
  resolution TEXT,
  directories TEXT NOT NULL DEFAULT '[]',
  metadata TEXT NOT NULL DEFAULT '{}',
  completed_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z')
);

CREATE UNIQUE INDEX udx_projects_key ON projects (key) WHERE completed_at IS NULL;
CREATE INDEX idx_projects_workflow_status ON projects (workflow_status);
CREATE INDEX idx_projects_completed_at ON projects (completed_at);

CREATE TABLE project_tasks (
  project_id TEXT NOT NULL,
  task_id TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  PRIMARY KEY (project_id, task_id),
  FOREIGN KEY (project_id) REFERENCES projects(id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  FOREIGN KEY (task_id) REFERENCES tasks(id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_project_tasks_project_id ON project_tasks (project_id);
CREATE INDEX idx_project_tasks_task_id ON project_tasks (task_id);

CREATE TABLE project_updates (
  id TEXT PRIMARY KEY NOT NULL,
  project_id TEXT NOT NULL,
  description TEXT,
  status TEXT NOT NULL,
  metadata TEXT NOT NULL DEFAULT '{}',
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  FOREIGN KEY (project_id) REFERENCES projects(id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_project_updates_project_id ON project_updates (project_id);
CREATE INDEX idx_project_updates_status ON project_updates (status);
CREATE INDEX idx_project_updates_created_at ON project_updates (created_at);
