CREATE TABLE tags (
  id TEXT NOT NULL PRIMARY KEY,
  label TEXT NOT NULL,
  metadata TEXT NOT NULL DEFAULT '{}',
  created_at TIMESTAMP NOT NULL DEFAULT (datatime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datatime('now', 'utc') || 'Z')
);

CREATE UNIQUE INDEX udx_tags_label ON tags (label);
