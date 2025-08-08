# Complish CLI MVP Specification

## Overview

Complish is a command-line productivity tool that helps users track tasks, capture daily commits, and maintain a work
journal - all stored locally as markdown files. The tool focuses on simple task management combined with automatic git
activity tracking and daily reflection.

**Core Philosophy:** Local-first, markdown-based system that gives users complete ownership of their productivity data
while providing intelligent daily summaries and date-range reports.

## Core Features

### CLI Task Management

#### Task Operations

* `complish add "task subject"` - Create new task (defaults to someday list)
* `complish add "task subject" --list today` - Create task in specific list
* `complish list` - Show all active tasks across all lists
* `complish list today` - Show tasks in specific list
* `complish complete <task-id>` - Mark task as complete
* `complish delete <task-id>` - Remove task
* `complish edit <task-id>` - Edit task description
* `complish move <task-id> <list>` - Move task between lists

#### Three-List System

* **Today:** Tasks the user wants to accomplish today
* **Next:** Tasks prioritized for upcoming work
* **Someday:** Default list for all tasks (backlog)

#### Task Storage

* Tasks organized by list in directory structure
* Each task stored as individual markdown file
* Filename format: `abc-01.md`, `def-02.md` (short identifiers)
* Task metadata in frontmatter (YAML)
* Task content in markdown body

#### Task Structure

```markdown
---
completed: DateTime?
created: DateTime
display_id: String
due_date: DateTime?
id: Integer
priority: Integer?
project_id: String?
status: String
tags: String[]?
updated: DateTime
---
# <SUBJECT>

## Description

[DESCRIPTION]
```

#### Project Structure

```markdown
---
completed: DateTime?
created: DateTime
directories: String[]?
name: String
updated: DateTime
---
# <NAME>

## Description

[DESCRIPTION]

## Tasks

- [<TASK_ID>](<PATH/TO/TASK.md>)
```

### Git Activity Tracking

#### Daily Git Scraping

* Scan user's machine for git repositories
* Collect all commits made by user on current day
* Store commit data in daily activity files
* Run automatically or via `complish sync` command

#### Git Data Collection

* Repository name and path
* Commit hash, message, timestamp
* Files changed (summary)
* Branch information

#### Storage Format

* Daily git activity stored in `YYYY-MM-DD-git.md`
* Structured markdown with commit details

### Daily Journal

#### Journal Entry

* `complish journal` - Open editor for daily reflection
* `complish journal "quick entry"` - Add quick journal entry
* One journal file per day: `YYYY-MM-DD-journal.md`
* Free-form markdown content

### Daily Reports

#### Automatic Daily Summary

* Generated at end of day or on-demand
* Combines: completed tasks, git commits, journal entries
* Stored as `YYYY-MM-DD-report.md`
* Structured markdown format for easy reading

#### Report Content

* Tasks completed that day
* Git activity summary (repos worked on, commit count)
* Journal entries
* [Inference] Basic stats (tasks completed, repos touched)

### Date Range Summaries

#### Range Query Command

* `complish summary --from YYYY-MM-DD --to YYYY-MM-DD`
* `complish summary --last-week`
* `complish summary --last-month`

#### Range Report Generation

* Aggregates all daily reports in date range
* Creates comprehensive markdown summary
* [Inference] May include patterns, productivity insights
* Output can be piped to LLM tools for further analysis

## File System Structure

```plaintext
~/.complish/
├── tasks/
│   ├── today/
│   │   ├── abc-01.md
│   │   └── abc-02.md
│   ├── next/
│   │   ├── abc-03.md
│   │   └── def-01.md
│   └── someday/
│       ├── def-02.md
│       └── def-03.md
├── projects/
│   ├── abc.md
│   ├── def.md
│   └── ...
├── journals/
│   ├── 2025-08-09-journal.md
│   └── ...
└── summaries/
    ├── 2025-08-01-to-2025-08-07-summary.md
    └── ...
```

### Configuration

Stored in user's default config directory (e.g., `~/.config/complish/` on Linux/macOS, `%APPDATA%/complish/` on Windows)

## Core Commands

> [!NOTE]
> All commands listed below are theoretical and represent the intended functionality. The final CLI implementation may
> use different command names, arguments, and structures based on development decisions and user experience testing.

### Task Management

* `complish add <description> [--list today|next|someday]` - Create task
* `complish list [today|next|someday]` - List tasks (all lists if no argument)
* `complish complete <id>` - Complete task
* `complish delete <id>` - Delete task
* `complish edit <id>` - Edit task
* `complish move <id> <list>` - Move task between lists

### Daily Activities

* `complish journal [entry]` - Add/edit journal entry
* `complish sync` - Scan for git commits
* `complish report` - Generate daily report

### Summaries

* `complish summary --from <date> --to <date>` - Date range summary
* `complish summary --last-week` - Last 7 days
* `complish summary --last-month` - Last 30 days

### Utility

* `complish init` - Initialize complish directory
* `complish status` - Show daily overview
* `complish config` - Manage settings

## Technical Implementation

### Platform

* **Language:** Rust CLI
* **Storage:** Local markdown files
* **Git Integration:** shell commands
* **Configuration:** TOML files

### Git Repository Discovery

* Scan common development directories
* Recursive search from configurable root paths
* Cache repository locations for performance
* Respect `.gitignore` and hidden directories

### Data Format Standardization

* Consistent YAML frontmatter across all files
* Markdown body for human-readable content
* ISO 8601 timestamps
* UTF-8 encoding

## MVP Scope

### Included in MVP

* Basic task CRUD operations
* Daily git commit collection
* Simple journal entries
* Daily report generation
* Date range summaries
* Local markdown storage
* Command-line interface

### Excluded from MVP

* Cloud synchronization
* Web interface
* Team collaboration
* Advanced git analysis
* Task scheduling/reminders
* Time tracking
* Task dependencies
* Multi-user support

## Post-MVP Roadmap

### Stage 2: Cloud Sync

* Complish "vault" synchronization
* Conflict resolution for concurrent edits
* Backup and restore functionality
* Cross-device access

### Stage 3: Enhanced Interfaces

* Web application
* Desktop application
* Mobile companion apps
* GUI task management

### Stage 4: Advanced Features

* Task scheduling and reminders
* Project organization
* Team collaboration features
* Advanced analytics and insights

## Success Metrics

**User Adoption:**

* Daily active usage (task creation/completion)
* Journal entry frequency
* Summary generation usage

**Data Quality:**

* Git commit capture accuracy
* Task completion rates
* Report generation reliability

**User Retention:**

* Continued daily usage after first week
* Long-term data accumulation (30+ days)

## Risk Considerations

* **Data Loss:** All data stored locally - users responsible for backups
* **Git Discovery:** May miss repositories in non-standard locations
* **Performance:** Large git history scanning could be slow
* **User Adoption:** CLI interface may limit user base
* **Data Migration:** Future cloud sync needs to handle existing local data

## Development Priorities

1. **Core CLI Framework:** Basic command structure and argument parsing
2. **Task Management:** CRUD operations with markdown storage
3. **Git Integration:** Repository discovery and commit collection
4. **Daily Reports:** Combine tasks, commits, and journal into summaries
5. **Date Range Queries:** Aggregate reports across time periods
6. **Journal System:** Simple text entry and editing
7. **Configuration Management:** User settings and preferences

## Technical Considerations

### File Locking

* [Inference] May need file locking for concurrent access
* Handle graceful failures when files are locked

### Performance

* Efficient git repository scanning
* Incremental commit collection
* Fast file system operations

### Cross-Platform Compatibility

* Windows, macOS, Linux support
* Path handling differences
* Git binary availability

### Data Integrity

* Atomic file operations
* Backup strategies for user data
* Graceful handling of corrupted files
