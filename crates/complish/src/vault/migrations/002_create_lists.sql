CREATE TABLE lists (
  id INTEGER PRIMARY KEY,
  name TEXT UNIQUE NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z')
);

CREATE INDEX idx_lists_name ON lists (name);

INSERT INTO lists (id, name)
VALUES
  (1, 'Today'),
  (2, 'Next'),
  (3, 'Someday');