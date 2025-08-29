CREATE TABLE task_relationships (
  source_id TEXT NOT NULL,
  target_id TEXT NOT NULL,
  kind TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),

  PRIMARY KEY (source_id, target_id, kind),
  FOREIGN KEY (source_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  FOREIGN KEY (target_id) REFERENCES tasks (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE INDEX idx_task_relationships_source_id ON task_relationships (source_id);
CREATE INDEX idx_task_relationships_target_id ON task_relationships (target_id);
CREATE INDEX idx_task_relationships_kind ON task_relationships (kind);
CREATE INDEX idx_task_relationships_source_kind ON task_relationships (source_id, kind);
