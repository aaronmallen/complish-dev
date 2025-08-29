CREATE TABLE project_tags (
  project_id TEXT NOT NULL,
  tag_id TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  PRIMARY KEY (project_id, tag_id),
  FOREIGN KEY (project_id) REFERENCES projects (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_project_tags_project_id ON project_tags (project_id);
CREATE INDEX idx_project_tags_tag_id ON project_tags (tag_id);
