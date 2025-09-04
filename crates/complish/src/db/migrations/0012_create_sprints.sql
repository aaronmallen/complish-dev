CREATE TABLE sprints (
  id TEXT PRIMARY KEY,
  started_at TIMESTAMP NOT NULL,
  ended_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z')
);

CREATE INDEX idx_sprints_started_at ON sprints (started_at);
CREATE INDEX idx_sprints_ended_at ON sprints (ended_at);
