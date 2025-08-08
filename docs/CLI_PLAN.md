# Complish CLI Implementation Plan

> [!NOTE]
> This document is subject to change as development progresses and requirements evolve.

This document outlines all CLI commands to be implemented, organized by resource type.

## Task Commands

### Core CRUD Operations

- [ ] `complish task create <title>` - Create a new task
- [ ] `complish task show <id>` - Show detailed task information
- [ ] `complish task list` - List tasks (with filtering options)
- [ ] `complish task update <id>` - Update task properties
- [ ] `complish task delete <id>` - Delete a task

### Workflow Management

- [ ] `complish task start <id>` - Start working on a task (time tracking)
- [ ] `complish task stop <id>` - Stop working on a task
- [ ] `complish task complete <id>` - Mark task as complete
- [ ] `complish task cancel <id>` - Cancel a task
- [ ] `complish task block <id>` - Mark task as blocked
- [ ] `complish task unblock <id>` - Unblock a task

### Relationships

- [ ] `complish task relate <id> <relationship> <target-id>` - Create task relationships
- [ ] `complish task unrelate <id> <relationship> <target-id>` - Remove relationships

### Organization

- [ ] `complish task tag <id> <tag>` - Add a tag to a task
- [ ] `complish task untag <id> <tag>` - Remove a tag from a task

## Project Commands

### Core CRUD Operations

- [ ] `complish project create <name>` - Create a new project
- [ ] `complish project show <key>` - Show detailed project information
- [ ] `complish project list` - List all projects
- [ ] `complish project update <key>` - Update project properties
- [ ] `complish project delete <key>` - Delete a project

### Workflow Management

- [ ] `complish project complete <key>` - Mark project as complete
- [ ] `complish project cancel <key>` - Cancel a project
- [ ] `complish project archive <key>` - Archive a completed project

### Organization

- [ ] `complish project tag <key> <tag>` - Add a tag to a project
- [ ] `complish project untag <key> <tag>` - Remove a tag from a project

## Sprint Commands

### Core Operations

- [ ] `complish sprint create` - Create a new sprint for today
- [ ] `complish sprint show [date]` - Show sprint details (defaults to today)
- [ ] `complish sprint list` - List recent sprints
- [ ] `complish sprint complete` - Complete the current sprint

### Task Management

- [ ] `complish sprint add <task-id>` - Add a task to current sprint
- [ ] `complish sprint remove <task-id>` - Remove a task from current sprint
- [ ] `complish sprint carryover` - Handle carryover tasks from previous sprint

## Journal Commands

### Core Operations

- [ ] `complish journal create [title]` - Create a new journal entry
- [ ] `complish journal show <id>` - Show a specific journal entry
- [ ] `complish journal list [date]` - List journal entries (defaults to today)
- [ ] `complish journal update <id>` - Update a journal entry
- [ ] `complish journal delete <id>` - Delete a journal entry

### Search and Discovery

- [ ] `complish journal search <query>` - Full-text search across journal entries

## Report Commands

### Accomplishment Reports

- [ ] `complish report accomplishments <start-date> <end-date>` - Generate accomplishment report
- [ ] `complish report daily [date]` - Generate daily summary report
- [ ] `complish report weekly [week]` - Generate weekly summary report
- [ ] `complish report monthly [month]` - Generate monthly summary report

### Export Options

- [ ] `complish report export <type> <start-date> <end-date>` - Export in various formats (markdown, json, text)

## Tag Commands

### Management

- [ ] `complish tag list` - List all tags
- [ ] `complish tag show <tag>` - Show items with specific tag
- [ ] `complish tag delete <tag>` - Delete a tag (removes from all items)

## Configuration Commands

### Settings Management

- [ ] `complish config get <key>` - Show current configuration
- [ ] `complish config set <key> <value>` - Set configuration value
- [ ] `complish config unset <key>` - Unset configuration value

## Global Commands

### General Operations

- [ ] `complish help [command]` - Show help information
- [ ] `complish version` - Show version information

### Default Behavior

- [ ] `complish` (no arguments) - Start/continue daily sprint or show status
