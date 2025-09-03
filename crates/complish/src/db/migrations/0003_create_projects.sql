CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    key TEXT NOT NULL,
    description TEXT,
    workflow_status TEXT NOT NULL,
    resolution TEXT,
    completed_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
    updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z')
);

CREATE UNIQUE INDEX idx_projects_key ON projects (key) WHERE completed_at IS NULL;
CREATE INDEX idx_projects_workflow_status ON projects (workflow_status);
CREATE INDEX idx_projects_resolution ON projects (resolution);
CREATE INDEX idx_projects_completed_at ON projects (completed_at);
CREATE INDEX idx_projects_created_at ON projects (created_at);
