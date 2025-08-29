CREATE TABLE journal_entries (
  id TEXT PRIMARY KEY,
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z'),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'utc') || 'Z')
);

CREATE INDEX idx_journal_entries_created_at ON journal_entries (created_at);
